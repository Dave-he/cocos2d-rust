/// 类型安全的事件总线系统
///
/// 提供发布/订阅模式，支持强类型事件，不依赖 Any 进行类型擦除。
///
/// # 用法
/// ```rust
/// let mut bus = EventBus::new();
///
/// // 订阅事件
/// let id = bus.subscribe("player_die", |evt: &PlayerDieEvent| {
///     println!("Player {} died at {:?}", evt.player_id, evt.position);
/// });
///
/// // 发布事件
/// bus.publish("player_die", PlayerDieEvent { player_id: 1, position: Vec2::ZERO });
///
/// // 取消订阅
/// bus.unsubscribe("player_die", id);
/// ```

use std::any::{Any, TypeId};
use std::collections::HashMap;

// ─── 强类型事件 ID ────────────────────────────────────────────────
/// 用于唯一标识一个订阅者
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SubscriberId(u64);

static NEXT_ID: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);

fn next_id() -> SubscriberId {
    SubscriberId(NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
}

// ─── 事件槽（Slot）抽象 ──────────────────────────────────────────
trait AnySlot: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

struct TypedSlot<T: 'static> {
    /// 按订阅 ID 存储回调
    handlers: HashMap<SubscriberId, Box<dyn Fn(&T)>>,
}

impl<T: 'static> AnySlot for TypedSlot<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

// ─── EventBus ───────────────────────────────────────────────────

/// 类型安全的事件总线
///
/// 内部按 TypeId 分槽存储订阅者。
pub struct EventBus {
    /// TypeId → 该类型的所有订阅槽
    slots: HashMap<TypeId, Box<dyn AnySlot>>,
    /// 统计发布次数（用于调试/测试）
    dispatch_count: u64,
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            slots: HashMap::new(),
            dispatch_count: 0,
        }
    }

    /// 订阅一个强类型事件 `T`，返回订阅者 ID（用于取消订阅）
    pub fn subscribe<T: 'static, F>(&mut self, handler: F) -> SubscriberId
    where
        F: Fn(&T) + 'static,
    {
        let id = next_id();
        let type_id = TypeId::of::<T>();

        let slot = self.slots
            .entry(type_id)
            .or_insert_with(|| Box::new(TypedSlot::<T> { handlers: HashMap::new() }));

        let slot = slot
            .as_any_mut()
            .downcast_mut::<TypedSlot<T>>()
            .expect("slot type mismatch (should never happen)");

        slot.handlers.insert(id, Box::new(handler));
        id
    }

    /// 取消订阅
    pub fn unsubscribe<T: 'static>(&mut self, id: SubscriberId) -> bool {
        let type_id = TypeId::of::<T>();
        if let Some(slot) = self.slots.get_mut(&type_id) {
            if let Some(slot) = slot.as_any_mut().downcast_mut::<TypedSlot<T>>() {
                return slot.handlers.remove(&id).is_some();
            }
        }
        false
    }

    /// 发布事件，调用所有订阅了类型 `T` 的处理器
    pub fn publish<T: 'static>(&mut self, event: &T) {
        let type_id = TypeId::of::<T>();
        self.dispatch_count += 1;

        if let Some(slot) = self.slots.get(&type_id) {
            if let Some(slot) = slot.as_any().downcast_ref::<TypedSlot<T>>() {
                for handler in slot.handlers.values() {
                    handler(event);
                }
            }
        }
    }

    /// 返回订阅了类型 `T` 的处理器数量
    pub fn subscriber_count<T: 'static>(&self) -> usize {
        let type_id = TypeId::of::<T>();
        self.slots
            .get(&type_id)
            .and_then(|s| s.as_any().downcast_ref::<TypedSlot<T>>())
            .map(|s| s.handlers.len())
            .unwrap_or(0)
    }

    /// 清除所有针对类型 `T` 的订阅
    pub fn clear<T: 'static>(&mut self) {
        self.slots.remove(&TypeId::of::<T>());
    }

    /// 清除所有订阅
    pub fn clear_all(&mut self) {
        self.slots.clear();
    }

    /// 获取总发布次数（用于调试）
    pub fn get_dispatch_count(&self) -> u64 {
        self.dispatch_count
    }

    /// 重置发布计数
    pub fn reset_dispatch_count(&mut self) {
        self.dispatch_count = 0;
    }
}

impl std::fmt::Debug for EventBus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventBus")
            .field("slot_types", &self.slots.len())
            .field("dispatch_count", &self.dispatch_count)
            .finish()
    }
}

// ─── 延迟事件队列 ────────────────────────────────────────────────

/// 包装任意类型事件以便在队列中存储
trait BoxedEvent: Any {
    fn as_any(&self) -> &dyn Any;
    /// 把事件分发给 EventBus（需要具体类型信息）
    fn dispatch(&self, bus: &mut EventBus);
}

struct ConcreteEvent<T: 'static>(T);

impl<T: 'static> BoxedEvent for ConcreteEvent<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn dispatch(&self, bus: &mut EventBus) {
        bus.publish(&self.0);
    }
}

/// 延迟事件队列 — 先入队，在帧末统一派发
pub struct DeferredEventQueue {
    queue: Vec<Box<dyn BoxedEvent>>,
}

impl Default for DeferredEventQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl DeferredEventQueue {
    pub fn new() -> Self {
        Self { queue: Vec::new() }
    }

    /// 将事件加入队列（不立即派发）
    pub fn enqueue<T: 'static>(&mut self, event: T) {
        self.queue.push(Box::new(ConcreteEvent(event)));
    }

    /// 一次性把队列中的所有事件派发到总线
    pub fn flush(&mut self, bus: &mut EventBus) {
        // drain 避免 borrow 冲突
        let events: Vec<Box<dyn BoxedEvent>> = self.queue.drain(..).collect();
        for e in events {
            e.dispatch(bus);
        }
    }

    /// 丢弃队列中所有事件（不派发）
    pub fn clear(&mut self) {
        self.queue.clear();
    }

    /// 队列中待派发的事件数
    pub fn pending_count(&self) -> usize {
        self.queue.len()
    }
}

impl std::fmt::Debug for DeferredEventQueue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DeferredEventQueue")
            .field("pending", &self.queue.len())
            .finish()
    }
}

// ─── 常用内置游戏事件 ────────────────────────────────────────────

use crate::math::Vec2;
use crate::base::types::Color4F;

/// 场景切换事件
#[derive(Debug, Clone)]
pub struct SceneChangeEvent {
    pub from_scene: Option<String>,
    pub to_scene: String,
}

/// 节点进入/退出场景事件
#[derive(Debug, Clone)]
pub struct NodeSceneEvent {
    pub node_tag: i32,
    pub node_name: String,
    pub enter: bool,
}

/// 触摸开始事件
#[derive(Debug, Clone)]
pub struct TouchBeganEvent {
    pub position: Vec2,
    pub touch_id: i32,
}

/// 触摸移动事件
#[derive(Debug, Clone)]
pub struct TouchMovedEvent {
    pub position: Vec2,
    pub delta: Vec2,
    pub touch_id: i32,
}

/// 触摸结束事件
#[derive(Debug, Clone)]
pub struct TouchEndedEvent {
    pub position: Vec2,
    pub touch_id: i32,
}

/// 键盘按下事件
#[derive(Debug, Clone)]
pub struct KeyDownEvent {
    pub key_code: i32,
}

/// 键盘释放事件
#[derive(Debug, Clone)]
pub struct KeyUpEvent {
    pub key_code: i32,
}

/// 鼠标点击事件
#[derive(Debug, Clone)]
pub struct MouseClickEvent {
    pub position: Vec2,
    pub button: u8,
    pub pressed: bool,
}

/// 渲染帧开始/结束事件
#[derive(Debug, Clone)]
pub struct FrameEvent {
    pub delta: f32,
    pub frame_count: u64,
}

/// 自定义游戏事件（通用）
#[derive(Debug, Clone)]
pub struct GameEvent {
    pub name: String,
    pub data: Option<String>,  // 序列化后的数据
}

impl GameEvent {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), data: None }
    }

    pub fn with_data(mut self, data: impl Into<String>) -> Self {
        self.data = Some(data.into());
        self
    }
}

/// 内存警告事件
#[derive(Debug, Clone)]
pub struct MemoryWarningEvent {
    pub level: u8, // 0=轻度 1=中度 2=严重
}

/// 分辨率/画面变化事件
#[derive(Debug, Clone)]
pub struct ResolutionChangeEvent {
    pub width: u32,
    pub height: u32,
}

/// 音频状态事件
#[derive(Debug, Clone)]
pub struct AudioEvent {
    pub sound_id: u32,
    pub finished: bool,
}

// ─── 测试 ────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    /// 测试事件
    #[derive(Debug, Clone)]
    struct ScoreEvent {
        score: i32,
    }

    #[derive(Debug, Clone)]
    struct DeathEvent {
        player_id: u32,
    }

    #[test]
    fn test_event_bus_subscribe_and_publish() {
        let mut bus = EventBus::new();
        let received = Arc::new(Mutex::new(0i32));
        let recv = received.clone();

        bus.subscribe::<ScoreEvent, _>(move |e| {
            *recv.lock().unwrap() += e.score;
        });

        bus.publish(&ScoreEvent { score: 10 });
        bus.publish(&ScoreEvent { score: 20 });

        assert_eq!(*received.lock().unwrap(), 30);
    }

    #[test]
    fn test_event_bus_multiple_subscribers() {
        let mut bus = EventBus::new();
        let count = Arc::new(Mutex::new(0u32));

        for _ in 0..5 {
            let c = count.clone();
            bus.subscribe::<ScoreEvent, _>(move |_| {
                *c.lock().unwrap() += 1;
            });
        }

        bus.publish(&ScoreEvent { score: 1 });
        assert_eq!(*count.lock().unwrap(), 5);
        assert_eq!(bus.subscriber_count::<ScoreEvent>(), 5);
    }

    #[test]
    fn test_event_bus_unsubscribe() {
        let mut bus = EventBus::new();
        let count = Arc::new(Mutex::new(0u32));
        let c = count.clone();

        let id = bus.subscribe::<ScoreEvent, _>(move |_| {
            *c.lock().unwrap() += 1;
        });

        bus.publish(&ScoreEvent { score: 1 });
        assert_eq!(*count.lock().unwrap(), 1);

        bus.unsubscribe::<ScoreEvent>(id);
        bus.publish(&ScoreEvent { score: 1 });
        assert_eq!(*count.lock().unwrap(), 1); // 不再触发
    }

    #[test]
    fn test_event_bus_different_types() {
        let mut bus = EventBus::new();
        let scores = Arc::new(Mutex::new(Vec::<i32>::new()));
        let deaths = Arc::new(Mutex::new(Vec::<u32>::new()));

        let s = scores.clone();
        let d = deaths.clone();

        bus.subscribe::<ScoreEvent, _>(move |e| s.lock().unwrap().push(e.score));
        bus.subscribe::<DeathEvent, _>(move |e| d.lock().unwrap().push(e.player_id));

        bus.publish(&ScoreEvent { score: 100 });
        bus.publish(&DeathEvent { player_id: 7 });
        bus.publish(&ScoreEvent { score: 200 });

        assert_eq!(*scores.lock().unwrap(), vec![100, 200]);
        assert_eq!(*deaths.lock().unwrap(), vec![7]);
    }

    #[test]
    fn test_event_bus_clear() {
        let mut bus = EventBus::new();
        let count = Arc::new(Mutex::new(0u32));
        let c = count.clone();

        bus.subscribe::<ScoreEvent, _>(move |_| *c.lock().unwrap() += 1);
        bus.clear::<ScoreEvent>();

        bus.publish(&ScoreEvent { score: 1 });
        assert_eq!(*count.lock().unwrap(), 0);
    }

    #[test]
    fn test_event_bus_clear_all() {
        let mut bus = EventBus::new();
        let score_count = Arc::new(Mutex::new(0u32));
        let death_count = Arc::new(Mutex::new(0u32));
        let sc = score_count.clone();
        let dc = death_count.clone();

        bus.subscribe::<ScoreEvent, _>(move |_| *sc.lock().unwrap() += 1);
        bus.subscribe::<DeathEvent, _>(move |_| *dc.lock().unwrap() += 1);
        bus.clear_all();

        bus.publish(&ScoreEvent { score: 1 });
        bus.publish(&DeathEvent { player_id: 1 });
        assert_eq!(*score_count.lock().unwrap(), 0);
        assert_eq!(*death_count.lock().unwrap(), 0);
    }

    #[test]
    fn test_event_bus_dispatch_count() {
        let mut bus = EventBus::new();
        bus.subscribe::<ScoreEvent, _>(|_| {});

        bus.publish(&ScoreEvent { score: 1 });
        bus.publish(&ScoreEvent { score: 2 });
        assert_eq!(bus.get_dispatch_count(), 2);

        bus.reset_dispatch_count();
        assert_eq!(bus.get_dispatch_count(), 0);
    }

    #[test]
    fn test_deferred_event_queue_basic() {
        let mut bus = EventBus::new();
        let mut queue = DeferredEventQueue::new();
        let received = Arc::new(Mutex::new(Vec::<i32>::new()));
        let r = received.clone();

        bus.subscribe::<ScoreEvent, _>(move |e| r.lock().unwrap().push(e.score));

        // 入队不立即派发
        queue.enqueue(ScoreEvent { score: 10 });
        queue.enqueue(ScoreEvent { score: 20 });
        assert_eq!(queue.pending_count(), 2);
        assert_eq!(received.lock().unwrap().len(), 0);

        // flush 才派发
        queue.flush(&mut bus);
        assert_eq!(queue.pending_count(), 0);
        assert_eq!(*received.lock().unwrap(), vec![10, 20]);
    }

    #[test]
    fn test_deferred_event_queue_clear() {
        let mut bus = EventBus::new();
        let mut queue = DeferredEventQueue::new();
        let count = Arc::new(Mutex::new(0u32));
        let c = count.clone();

        bus.subscribe::<ScoreEvent, _>(move |_| *c.lock().unwrap() += 1);

        queue.enqueue(ScoreEvent { score: 1 });
        queue.enqueue(ScoreEvent { score: 2 });
        queue.clear(); // 丢弃
        queue.flush(&mut bus);

        assert_eq!(*count.lock().unwrap(), 0);
    }

    #[test]
    fn test_deferred_event_queue_multiple_types() {
        let mut bus = EventBus::new();
        let mut queue = DeferredEventQueue::new();
        let scores = Arc::new(Mutex::new(Vec::<i32>::new()));
        let deaths = Arc::new(Mutex::new(Vec::<u32>::new()));
        let s = scores.clone();
        let d = deaths.clone();

        bus.subscribe::<ScoreEvent, _>(move |e| s.lock().unwrap().push(e.score));
        bus.subscribe::<DeathEvent, _>(move |e| d.lock().unwrap().push(e.player_id));

        queue.enqueue(ScoreEvent { score: 50 });
        queue.enqueue(DeathEvent { player_id: 3 });
        queue.enqueue(ScoreEvent { score: 100 });
        queue.flush(&mut bus);

        assert_eq!(*scores.lock().unwrap(), vec![50, 100]);
        assert_eq!(*deaths.lock().unwrap(), vec![3]);
    }

    #[test]
    fn test_builtin_events() {
        let mut bus = EventBus::new();
        let received = Arc::new(Mutex::new(false));
        let r = received.clone();

        bus.subscribe::<GameEvent, _>(move |e| {
            if e.name == "level_complete" {
                *r.lock().unwrap() = true;
            }
        });

        bus.publish(&GameEvent::new("level_complete").with_data("level=5"));
        assert!(*received.lock().unwrap());
    }

    #[test]
    fn test_touch_events() {
        let mut bus = EventBus::new();
        let positions = Arc::new(Mutex::new(Vec::<Vec2>::new()));
        let p = positions.clone();

        bus.subscribe::<TouchBeganEvent, _>(move |e| {
            p.lock().unwrap().push(e.position);
        });

        bus.publish(&TouchBeganEvent { position: Vec2::new(100.0, 200.0), touch_id: 0 });
        bus.publish(&TouchBeganEvent { position: Vec2::new(300.0, 400.0), touch_id: 1 });

        let pos = positions.lock().unwrap();
        assert_eq!(pos.len(), 2);
        assert!((pos[0].x - 100.0).abs() < 0.01);
    }

    #[test]
    fn test_subscriber_id_uniqueness() {
        let mut bus = EventBus::new();
        let mut ids = Vec::new();

        for _ in 0..10 {
            let id = bus.subscribe::<ScoreEvent, _>(|_| {});
            ids.push(id);
        }

        // 所有 ID 唯一
        let unique: std::collections::HashSet<u64> = ids.iter().map(|id| id.0).collect();
        assert_eq!(unique.len(), 10);
    }
}

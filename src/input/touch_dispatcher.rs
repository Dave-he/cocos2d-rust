use super::touch::{Touch, TouchId, TouchPhase};
use crate::math::Vec2;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

/// 触摸事件监听器
pub trait TouchListener {
    /// 触摸开始
    fn on_touches_began(&mut self, touches: &[Touch]) -> bool;
    
    /// 触摸移动
    fn on_touches_moved(&mut self, touches: &[Touch]) -> bool;
    
    /// 触摸结束
    fn on_touches_ended(&mut self, touches: &[Touch]) -> bool;
    
    /// 触摸取消
    fn on_touches_cancelled(&mut self, touches: &[Touch]) -> bool;

    /// 获取监听器优先级
    fn priority(&self) -> i32 {
        0
    }

    /// 是否吞没触摸事件
    fn swallow_touches(&self) -> bool {
        false
    }
}

/// 触摸分发器
pub struct TouchDispatcher {
    /// 活动的触摸
    active_touches: HashMap<TouchId, Touch>,
    /// 注册的监听器
    listeners: Vec<Rc<RefCell<dyn TouchListener>>>,
    /// 是否需要重新排序监听器
    needs_sort: bool,
    /// 是否正在分发事件
    is_dispatching: bool,
    /// 待添加的监听器
    pending_add: Vec<Rc<RefCell<dyn TouchListener>>>,
    /// 待移除的监听器
    pending_remove: Vec<Rc<RefCell<dyn TouchListener>>>,
}

impl TouchDispatcher {
    /// 创建新的触摸分发器
    pub fn new() -> Self {
        Self {
            active_touches: HashMap::new(),
            listeners: Vec::new(),
            needs_sort: false,
            is_dispatching: false,
            pending_add: Vec::new(),
            pending_remove: Vec::new(),
        }
    }

    /// 添加触摸监听器
    pub fn add_listener(&mut self, listener: Rc<RefCell<dyn TouchListener>>) {
        if self.is_dispatching {
            self.pending_add.push(listener);
        } else {
            self.listeners.push(listener);
            self.needs_sort = true;
        }
    }

    /// 移除触摸监听器
    pub fn remove_listener(&mut self, listener: Rc<RefCell<dyn TouchListener>>) {
        if self.is_dispatching {
            self.pending_remove.push(listener);
        } else {
            self.listeners.retain(|l| !Rc::ptr_eq(l, &listener));
        }
    }

    /// 移除所有监听器
    pub fn remove_all_listeners(&mut self) {
        if !self.is_dispatching {
            self.listeners.clear();
        }
    }

    /// 处理触摸开始
    pub fn handle_touches_began(&mut self, touches: Vec<Touch>) {
        // 添加到活动触摸集合
        for touch in &touches {
            self.active_touches.insert(touch.id(), touch.clone());
        }

        self.dispatch_touches(&touches, |listener, touches| {
            listener.borrow_mut().on_touches_began(touches)
        });
    }

    /// 处理触摸移动
    pub fn handle_touches_moved(&mut self, touches: Vec<Touch>) {
        // 更新活动触摸
        for touch in &touches {
            if let Some(active_touch) = self.active_touches.get_mut(&touch.id()) {
                *active_touch = touch.clone();
            }
        }

        self.dispatch_touches(&touches, |listener, touches| {
            listener.borrow_mut().on_touches_moved(touches)
        });
    }

    /// 处理触摸结束
    pub fn handle_touches_ended(&mut self, touches: Vec<Touch>) {
        // 从活动触摸集合中移除
        for touch in &touches {
            self.active_touches.remove(&touch.id());
        }

        self.dispatch_touches(&touches, |listener, touches| {
            listener.borrow_mut().on_touches_ended(touches)
        });
    }

    /// 处理触摸取消
    pub fn handle_touches_cancelled(&mut self, touches: Vec<Touch>) {
        // 从活动触摸集合中移除
        for touch in &touches {
            self.active_touches.remove(&touch.id());
        }

        self.dispatch_touches(&touches, |listener, touches| {
            listener.borrow_mut().on_touches_cancelled(touches)
        });
    }

    /// 分发触摸事件
    fn dispatch_touches<F>(&mut self, touches: &[Touch], mut callback: F)
    where
        F: FnMut(&Rc<RefCell<dyn TouchListener>>, &[Touch]) -> bool,
    {
        if touches.is_empty() {
            return;
        }

        // 排序监听器
        if self.needs_sort {
            self.sort_listeners();
            self.needs_sort = false;
        }

        self.is_dispatching = true;

        // 按优先级分发事件
        for listener in &self.listeners {
            let handled = callback(listener, touches);
            
            // 如果监听器吞没事件，停止分发
            if handled && listener.borrow().swallow_touches() {
                break;
            }
        }

        self.is_dispatching = false;

        // 处理待添加/移除的监听器
        self.process_pending_operations();
    }

    /// 排序监听器（按优先级降序）
    fn sort_listeners(&mut self) {
        self.listeners.sort_by(|a, b| {
            let priority_a = a.borrow().priority();
            let priority_b = b.borrow().priority();
            priority_b.cmp(&priority_a) // 降序
        });
    }

    /// 处理待添加/移除的操作
    fn process_pending_operations(&mut self) {
        // 添加待添加的监听器
        for listener in self.pending_add.drain(..) {
            self.listeners.push(listener);
            self.needs_sort = true;
        }

        // 移除待移除的监听器
        for listener in self.pending_remove.drain(..) {
            self.listeners.retain(|l| !Rc::ptr_eq(l, &listener));
        }
    }

    /// 获取指定 ID 的触摸
    pub fn get_touch(&self, id: TouchId) -> Option<&Touch> {
        self.active_touches.get(&id)
    }

    /// 获取所有活动触摸
    pub fn active_touches(&self) -> Vec<&Touch> {
        self.active_touches.values().collect()
    }

    /// 获取活动触摸数量
    pub fn active_touch_count(&self) -> usize {
        self.active_touches.len()
    }

    /// 清空所有触摸
    pub fn clear_touches(&mut self) {
        self.active_touches.clear();
    }
}

impl Default for TouchDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestListener {
        priority: i32,
        swallow: bool,
        touches_began_count: usize,
        touches_moved_count: usize,
        touches_ended_count: usize,
    }

    impl TestListener {
        fn new(priority: i32, swallow: bool) -> Self {
            Self {
                priority,
                swallow,
                touches_began_count: 0,
                touches_moved_count: 0,
                touches_ended_count: 0,
            }
        }
    }

    impl TouchListener for TestListener {
        fn on_touches_began(&mut self, _touches: &[Touch]) -> bool {
            self.touches_began_count += 1;
            true
        }

        fn on_touches_moved(&mut self, _touches: &[Touch]) -> bool {
            self.touches_moved_count += 1;
            true
        }

        fn on_touches_ended(&mut self, _touches: &[Touch]) -> bool {
            self.touches_ended_count += 1;
            true
        }

        fn on_touches_cancelled(&mut self, _touches: &[Touch]) -> bool {
            true
        }

        fn priority(&self) -> i32 {
            self.priority
        }

        fn swallow_touches(&self) -> bool {
            self.swallow
        }
    }

    #[test]
    fn test_touch_dispatcher_basic() {
        let mut dispatcher = TouchDispatcher::new();
        
        let listener1 = Rc::new(RefCell::new(TestListener::new(0, false)));
        dispatcher.add_listener(listener1.clone());

        let touches = vec![Touch::new(1, Vec2::new(100.0, 200.0))];
        dispatcher.handle_touches_began(touches);

        assert_eq!(listener1.borrow().touches_began_count, 1);
        assert_eq!(dispatcher.active_touch_count(), 1);
    }

    #[test]
    fn test_touch_dispatcher_priority() {
        let mut dispatcher = TouchDispatcher::new();
        
        let listener1 = Rc::new(RefCell::new(TestListener::new(10, false)));
        let listener2 = Rc::new(RefCell::new(TestListener::new(20, false)));
        
        dispatcher.add_listener(listener1.clone());
        dispatcher.add_listener(listener2.clone());

        let touches = vec![Touch::new(1, Vec2::new(100.0, 200.0))];
        dispatcher.handle_touches_began(touches);

        // 优先级高的先执行
        assert_eq!(listener1.borrow().touches_began_count, 1);
        assert_eq!(listener2.borrow().touches_began_count, 1);
    }

    #[test]
    fn test_touch_dispatcher_swallow() {
        let mut dispatcher = TouchDispatcher::new();
        
        let listener1 = Rc::new(RefCell::new(TestListener::new(20, true))); // 吞没事件
        let listener2 = Rc::new(RefCell::new(TestListener::new(10, false)));
        
        dispatcher.add_listener(listener1.clone());
        dispatcher.add_listener(listener2.clone());

        let touches = vec![Touch::new(1, Vec2::new(100.0, 200.0))];
        dispatcher.handle_touches_began(touches);

        // listener1 吞没事件，listener2 不会收到
        assert_eq!(listener1.borrow().touches_began_count, 1);
        assert_eq!(listener2.borrow().touches_began_count, 0);
    }

    #[test]
    fn test_touch_lifecycle() {
        let mut dispatcher = TouchDispatcher::new();
        
        let listener = Rc::new(RefCell::new(TestListener::new(0, false)));
        dispatcher.add_listener(listener.clone());

        // 开始
        let touches = vec![Touch::new(1, Vec2::new(100.0, 200.0))];
        dispatcher.handle_touches_began(touches.clone());
        assert_eq!(dispatcher.active_touch_count(), 1);

        // 移动
        dispatcher.handle_touches_moved(touches.clone());
        assert_eq!(dispatcher.active_touch_count(), 1);

        // 结束
        dispatcher.handle_touches_ended(touches);
        assert_eq!(dispatcher.active_touch_count(), 0);

        let listener_ref = listener.borrow();
        assert_eq!(listener_ref.touches_began_count, 1);
        assert_eq!(listener_ref.touches_moved_count, 1);
        assert_eq!(listener_ref.touches_ended_count, 1);
    }
}

/// Component 系统 - 组件模式
///
/// 对应 cocos2d-x 的 Component 系统，允许为 Node 附加可复用的组件。
///
/// 功能：
/// - 在 Node 上附加自定义行为组件
/// - 组件生命周期管理（onEnter/onExit/update）
/// - 通过名称访问组件
/// - 组件之间通信

use std::any::Any;

/// 组件特征 - 所有组件的基础接口
pub trait Component: Any + Send {
    /// 获取组件名称
    fn get_name(&self) -> &str;

    /// 组件进入场景时调用
    fn on_enter(&mut self) {}

    /// 组件退出场景时调用
    fn on_exit(&mut self) {}

    /// 每帧更新
    fn update(&mut self, _delta: f32) {}

    /// 销毁时调用
    fn on_destroy(&mut self) {}

    /// 是否启用
    fn is_enabled(&self) -> bool;

    /// 设置启用/禁用
    fn set_enabled(&mut self, enabled: bool);

    /// 转换为 Any（用于向下转型）
    fn as_any(&self) -> &dyn Any;

    /// 转换为可变 Any
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// 基础组件结构
#[derive(Debug)]
pub struct ComponentBase {
    name: String,
    enabled: bool,
    running: bool,
}

impl ComponentBase {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            enabled: true,
            running: false,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn set_running(&mut self, running: bool) {
        self.running = running;
    }
}

/// 组件管理器 - 管理一组组件
#[derive(Default)]
pub struct ComponentContainer {
    components: Vec<Box<dyn Component>>,
}

impl ComponentContainer {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }

    /// 添加组件
    pub fn add_component(&mut self, component: Box<dyn Component>) -> bool {
        let name = component.get_name().to_string();
        // 检查是否已存在同名组件
        if self.get_component(&name).is_some() {
            return false;
        }
        self.components.push(component);
        true
    }

    /// 通过名称获取组件
    pub fn get_component(&self, name: &str) -> Option<&dyn Component> {
        self.components.iter()
            .find(|c| c.get_name() == name)
            .map(|c| c.as_ref())
    }

    /// 通过名称获取可变组件
    pub fn get_component_mut(&mut self, name: &str) -> Option<&mut dyn Component> {
        self.components.iter_mut()
            .find(|c| c.get_name() == name)
            .map(|c| c.as_mut())
    }

    /// 通过类型获取组件
    pub fn get_component_by_type<T: Component + 'static>(&self) -> Option<&T> {
        for c in &self.components {
            if let Some(typed) = c.as_any().downcast_ref::<T>() {
                return Some(typed);
            }
        }
        None
    }

    /// 通过类型获取可变组件
    pub fn get_component_by_type_mut<T: Component + 'static>(&mut self) -> Option<&mut T> {
        for c in &mut self.components {
            if let Some(typed) = c.as_any_mut().downcast_mut::<T>() {
                return Some(typed);
            }
        }
        None
    }

    /// 移除组件
    pub fn remove_component(&mut self, name: &str) -> bool {
        let len_before = self.components.len();
        self.components.retain(|c| c.get_name() != name);
        self.components.len() < len_before
    }

    /// 移除所有组件
    pub fn remove_all_components(&mut self) {
        self.components.clear();
    }

    /// 获取组件数量
    pub fn get_component_count(&self) -> usize {
        self.components.len()
    }

    /// 是否包含某组件
    pub fn has_component(&self, name: &str) -> bool {
        self.components.iter().any(|c| c.get_name() == name)
    }

    /// 更新所有组件
    pub fn update(&mut self, delta: f32) {
        for component in &mut self.components {
            if component.is_enabled() {
                component.update(delta);
            }
        }
    }

    /// 通知所有组件进入场景
    pub fn on_enter(&mut self) {
        for component in &mut self.components {
            component.on_enter();
        }
    }

    /// 通知所有组件退出场景
    pub fn on_exit(&mut self) {
        for component in &mut self.components {
            component.on_exit();
        }
    }

    /// 获取所有组件名称
    pub fn get_component_names(&self) -> Vec<&str> {
        self.components.iter().map(|c| c.get_name()).collect()
    }

    /// 迭代器
    pub fn iter(&self) -> impl Iterator<Item = &dyn Component> {
        self.components.iter().map(|c| c.as_ref())
    }
}

impl std::fmt::Debug for ComponentContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let names: Vec<&str> = self.components.iter().map(|c| c.get_name()).collect();
        f.debug_struct("ComponentContainer")
            .field("components", &names)
            .finish()
    }
}

/// 定时器组件 - 用于延迟执行逻辑
pub struct TimerComponent {
    base: ComponentBase,
    interval: f32,
    elapsed: f32,
    repeat: bool,
    times_executed: u32,
    max_times: Option<u32>,
    callback: Option<Box<dyn Fn(u32) + Send>>,
}

impl TimerComponent {
    pub fn new(name: impl Into<String>, interval: f32, repeat: bool) -> Self {
        Self {
            base: ComponentBase::new(name),
            interval,
            elapsed: 0.0,
            repeat,
            times_executed: 0,
            max_times: None,
            callback: None,
        }
    }

    pub fn with_max_times(mut self, max_times: u32) -> Self {
        self.max_times = Some(max_times);
        self
    }

    pub fn with_callback(mut self, callback: impl Fn(u32) + Send + 'static) -> Self {
        self.callback = Some(Box::new(callback));
        self
    }

    pub fn get_elapsed(&self) -> f32 {
        self.elapsed
    }

    pub fn get_times_executed(&self) -> u32 {
        self.times_executed
    }

    pub fn reset(&mut self) {
        self.elapsed = 0.0;
        self.times_executed = 0;
    }
}

impl Component for TimerComponent {
    fn get_name(&self) -> &str {
        self.base.name()
    }

    fn update(&mut self, delta: f32) {
        if !self.base.is_enabled() {
            return;
        }

        self.elapsed += delta;

        if self.elapsed >= self.interval {
            self.elapsed -= self.interval;
            self.times_executed += 1;

            if let Some(cb) = &self.callback {
                cb(self.times_executed);
            }

            if let Some(max) = self.max_times {
                if self.times_executed >= max {
                    self.base.set_enabled(false);
                }
            }

            if !self.repeat {
                self.base.set_enabled(false);
            }
        }
    }

    fn is_enabled(&self) -> bool {
        self.base.is_enabled()
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.base.set_enabled(enabled);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl std::fmt::Debug for TimerComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TimerComponent")
            .field("name", &self.base.name())
            .field("interval", &self.interval)
            .field("elapsed", &self.elapsed)
            .field("repeat", &self.repeat)
            .field("times_executed", &self.times_executed)
            .finish()
    }
}

/// 自定义逻辑组件 - 支持 Fn 回调
pub struct ScriptComponent {
    base: ComponentBase,
    update_fn: Option<Box<dyn Fn(f32) + Send>>,
    enter_fn: Option<Box<dyn Fn() + Send>>,
    exit_fn: Option<Box<dyn Fn() + Send>>,
}

impl ScriptComponent {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            base: ComponentBase::new(name),
            update_fn: None,
            enter_fn: None,
            exit_fn: None,
        }
    }

    pub fn set_update(mut self, f: impl Fn(f32) + Send + 'static) -> Self {
        self.update_fn = Some(Box::new(f));
        self
    }

    pub fn set_on_enter(mut self, f: impl Fn() + Send + 'static) -> Self {
        self.enter_fn = Some(Box::new(f));
        self
    }

    pub fn set_on_exit(mut self, f: impl Fn() + Send + 'static) -> Self {
        self.exit_fn = Some(Box::new(f));
        self
    }
}

impl Component for ScriptComponent {
    fn get_name(&self) -> &str {
        self.base.name()
    }

    fn on_enter(&mut self) {
        if let Some(f) = &self.enter_fn {
            f();
        }
    }

    fn on_exit(&mut self) {
        if let Some(f) = &self.exit_fn {
            f();
        }
    }

    fn update(&mut self, delta: f32) {
        if self.base.is_enabled() {
            if let Some(f) = &self.update_fn {
                f(delta);
            }
        }
    }

    fn is_enabled(&self) -> bool {
        self.base.is_enabled()
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.base.set_enabled(enabled);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl std::fmt::Debug for ScriptComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ScriptComponent")
            .field("name", &self.base.name())
            .field("enabled", &self.base.is_enabled())
            .finish()
    }
}

/// 状态机组件 - 管理游戏对象的状态
#[derive(Debug)]
pub struct StateMachineComponent {
    base: ComponentBase,
    current_state: String,
    previous_state: String,
    transitions: Vec<(String, String)>, // (from, to)
}

impl StateMachineComponent {
    pub fn new(name: impl Into<String>, initial_state: impl Into<String>) -> Self {
        let state = initial_state.into();
        Self {
            base: ComponentBase::new(name),
            current_state: state.clone(),
            previous_state: state,
            transitions: Vec::new(),
        }
    }

    pub fn add_transition(&mut self, from: impl Into<String>, to: impl Into<String>) {
        self.transitions.push((from.into(), to.into()));
    }

    pub fn transition_to(&mut self, new_state: impl Into<String>) -> bool {
        let new_state = new_state.into();
        let can_transition = self.transitions.iter()
            .any(|(from, to)| from == &self.current_state && to == &new_state);

        if can_transition || self.transitions.is_empty() {
            self.previous_state = self.current_state.clone();
            self.current_state = new_state;
            true
        } else {
            false
        }
    }

    pub fn get_current_state(&self) -> &str {
        &self.current_state
    }

    pub fn get_previous_state(&self) -> &str {
        &self.previous_state
    }

    pub fn is_in_state(&self, state: &str) -> bool {
        self.current_state == state
    }

    pub fn get_transition_count(&self) -> usize {
        self.transitions.len()
    }
}

impl Component for StateMachineComponent {
    fn get_name(&self) -> &str {
        self.base.name()
    }

    fn is_enabled(&self) -> bool {
        self.base.is_enabled()
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.base.set_enabled(enabled);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/// 属性组件 - 通用键值存储
#[derive(Debug)]
pub struct PropertyComponent {
    base: ComponentBase,
    int_props: std::collections::HashMap<String, i64>,
    float_props: std::collections::HashMap<String, f64>,
    string_props: std::collections::HashMap<String, String>,
    bool_props: std::collections::HashMap<String, bool>,
}

impl PropertyComponent {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            base: ComponentBase::new(name),
            int_props: std::collections::HashMap::new(),
            float_props: std::collections::HashMap::new(),
            string_props: std::collections::HashMap::new(),
            bool_props: std::collections::HashMap::new(),
        }
    }

    pub fn set_int(&mut self, key: impl Into<String>, value: i64) {
        self.int_props.insert(key.into(), value);
    }

    pub fn get_int(&self, key: &str) -> Option<i64> {
        self.int_props.get(key).copied()
    }

    pub fn set_float(&mut self, key: impl Into<String>, value: f64) {
        self.float_props.insert(key.into(), value);
    }

    pub fn get_float(&self, key: &str) -> Option<f64> {
        self.float_props.get(key).copied()
    }

    pub fn set_string(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.string_props.insert(key.into(), value.into());
    }

    pub fn get_string(&self, key: &str) -> Option<&str> {
        self.string_props.get(key).map(|s| s.as_str())
    }

    pub fn set_bool(&mut self, key: impl Into<String>, value: bool) {
        self.bool_props.insert(key.into(), value);
    }

    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.bool_props.get(key).copied()
    }

    pub fn has_property(&self, key: &str) -> bool {
        self.int_props.contains_key(key)
            || self.float_props.contains_key(key)
            || self.string_props.contains_key(key)
            || self.bool_props.contains_key(key)
    }

    pub fn remove_property(&mut self, key: &str) {
        self.int_props.remove(key);
        self.float_props.remove(key);
        self.string_props.remove(key);
        self.bool_props.remove(key);
    }

    pub fn get_all_keys(&self) -> Vec<&str> {
        let mut keys: Vec<&str> = Vec::new();
        keys.extend(self.int_props.keys().map(|k| k.as_str()));
        keys.extend(self.float_props.keys().map(|k| k.as_str()));
        keys.extend(self.string_props.keys().map(|k| k.as_str()));
        keys.extend(self.bool_props.keys().map(|k| k.as_str()));
        keys.sort();
        keys.dedup();
        keys
    }

    pub fn get_int_count(&self) -> usize {
        self.int_props.len()
    }

    pub fn get_float_count(&self) -> usize {
        self.float_props.len()
    }

    pub fn get_string_count(&self) -> usize {
        self.string_props.len()
    }

    pub fn get_bool_count(&self) -> usize {
        self.bool_props.len()
    }
}

impl Component for PropertyComponent {
    fn get_name(&self) -> &str {
        self.base.name()
    }

    fn is_enabled(&self) -> bool {
        self.base.is_enabled()
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.base.set_enabled(enabled);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_container_add_get() {
        let mut container = ComponentContainer::new();
        
        // 添加定时器组件
        let timer = TimerComponent::new("timer1", 1.0, true);
        assert!(container.add_component(Box::new(timer)));
        assert_eq!(container.get_component_count(), 1);
        
        // 不允许同名
        let timer2 = TimerComponent::new("timer1", 2.0, false);
        assert!(!container.add_component(Box::new(timer2)));
        assert_eq!(container.get_component_count(), 1);
    }

    #[test]
    fn test_component_container_remove() {
        let mut container = ComponentContainer::new();
        
        container.add_component(Box::new(TimerComponent::new("comp1", 0.5, true)));
        container.add_component(Box::new(TimerComponent::new("comp2", 1.0, false)));
        assert_eq!(container.get_component_count(), 2);
        
        assert!(container.remove_component("comp1"));
        assert_eq!(container.get_component_count(), 1);
        
        assert!(!container.remove_component("non_existent"));
        assert_eq!(container.get_component_count(), 1);
    }

    #[test]
    fn test_component_container_remove_all() {
        let mut container = ComponentContainer::new();
        
        container.add_component(Box::new(TimerComponent::new("a", 1.0, false)));
        container.add_component(Box::new(TimerComponent::new("b", 2.0, false)));
        assert_eq!(container.get_component_count(), 2);
        
        container.remove_all_components();
        assert_eq!(container.get_component_count(), 0);
    }

    #[test]
    fn test_component_container_has_component() {
        let mut container = ComponentContainer::new();
        
        assert!(!container.has_component("timer"));
        container.add_component(Box::new(TimerComponent::new("timer", 1.0, true)));
        assert!(container.has_component("timer"));
    }

    #[test]
    fn test_component_container_update() {
        let mut container = ComponentContainer::new();
        
        let timer = TimerComponent::new("timer", 0.5, true);
        container.add_component(Box::new(timer));
        
        // 更新 0.3 秒，不应触发
        container.update(0.3);
        let timer = container.get_component_by_type::<TimerComponent>().unwrap();
        assert_eq!(timer.get_times_executed(), 0);
        
        // 再更新 0.3 秒，应触发一次
        container.update(0.3);
        let timer = container.get_component_by_type::<TimerComponent>().unwrap();
        assert_eq!(timer.get_times_executed(), 1);
    }

    #[test]
    fn test_timer_component_repeat() {
        let mut timer = TimerComponent::new("test", 0.1, true);
        
        for _ in 0..5 {
            timer.update(0.1);
        }
        assert_eq!(timer.get_times_executed(), 5);
        assert!(timer.is_enabled()); // 重复模式下仍启用
    }

    #[test]
    fn test_timer_component_no_repeat() {
        let mut timer = TimerComponent::new("test", 0.1, false);
        
        timer.update(0.1);
        assert_eq!(timer.get_times_executed(), 1);
        assert!(!timer.is_enabled()); // 非重复模式下自动禁用
        
        // 再更新不应触发
        timer.update(1.0);
        assert_eq!(timer.get_times_executed(), 1);
    }

    #[test]
    fn test_timer_component_max_times() {
        let mut timer = TimerComponent::new("test", 0.1, true)
            .with_max_times(3);
        
        for _ in 0..5 {
            timer.update(0.1);
        }
        assert_eq!(timer.get_times_executed(), 3);
        assert!(!timer.is_enabled());
    }

    #[test]
    fn test_script_component() {
        let executed = std::sync::Arc::new(std::sync::Mutex::new(0u32));
        let counter = executed.clone();
        
        let mut script = ScriptComponent::new("script")
            .set_update(move |_delta| {
                *counter.lock().unwrap() += 1;
            });
        
        script.update(0.016);
        script.update(0.016);
        script.update(0.016);
        
        assert_eq!(*executed.lock().unwrap(), 3);
    }

    #[test]
    fn test_state_machine_component() {
        let mut sm = StateMachineComponent::new("state_machine", "idle");
        sm.add_transition("idle", "run");
        sm.add_transition("run", "jump");
        sm.add_transition("jump", "idle");
        
        assert!(sm.is_in_state("idle"));
        
        // 有效转换
        assert!(sm.transition_to("run"));
        assert!(sm.is_in_state("run"));
        assert_eq!(sm.get_previous_state(), "idle");
        
        // 无效转换（idle -> jump 不允许）
        assert!(!sm.transition_to("idle"));
        assert!(sm.is_in_state("run")); // 状态不变
        
        // 有效转换
        assert!(sm.transition_to("jump"));
        assert!(sm.is_in_state("jump"));
    }

    #[test]
    fn test_property_component() {
        let mut props = PropertyComponent::new("properties");
        
        props.set_int("health", 100);
        props.set_float("speed", 5.5);
        props.set_string("name", "Player");
        props.set_bool("alive", true);
        
        assert_eq!(props.get_int("health"), Some(100));
        assert_eq!(props.get_float("speed"), Some(5.5));
        assert_eq!(props.get_string("name"), Some("Player"));
        assert_eq!(props.get_bool("alive"), Some(true));
        assert_eq!(props.get_int("non_existent"), None);
        
        assert!(props.has_property("health"));
        assert!(!props.has_property("missing"));
        
        props.remove_property("health");
        assert!(!props.has_property("health"));
    }

    #[test]
    fn test_property_component_counts() {
        let mut props = PropertyComponent::new("props");
        
        props.set_int("a", 1);
        props.set_int("b", 2);
        props.set_float("x", 1.0);
        props.set_string("s", "hello");
        props.set_bool("flag", true);
        
        assert_eq!(props.get_int_count(), 2);
        assert_eq!(props.get_float_count(), 1);
        assert_eq!(props.get_string_count(), 1);
        assert_eq!(props.get_bool_count(), 1);
    }

    #[test]
    fn test_component_get_by_type() {
        let mut container = ComponentContainer::new();
        
        let timer = TimerComponent::new("mytimer", 0.5, true);
        container.add_component(Box::new(timer));
        
        let found = container.get_component_by_type::<TimerComponent>();
        assert!(found.is_some());
        assert_eq!(found.unwrap().get_name(), "mytimer");
    }

    #[test]
    fn test_component_names() {
        let mut container = ComponentContainer::new();
        
        container.add_component(Box::new(TimerComponent::new("comp_a", 1.0, false)));
        container.add_component(Box::new(ScriptComponent::new("comp_b")));
        container.add_component(Box::new(PropertyComponent::new("comp_c")));
        
        let names = container.get_component_names();
        assert_eq!(names.len(), 3);
        assert!(names.contains(&"comp_a"));
        assert!(names.contains(&"comp_b"));
        assert!(names.contains(&"comp_c"));
    }
}

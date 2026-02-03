/// Action - 动作基类和系统
///
/// 动作系统是 Cocos2d-Rust 中动画和动态效果的核心。

use std::rc::Rc;
use std::cell::RefCell;
use crate::scene::Node;

/// Action 标签常量
pub const INVALID_TAG: i32 = -1;

/// Action - 动作基类
pub trait Action: std::fmt::Debug {
    /// 克隆动作
    fn clone_action(&self) -> Box<dyn Action>;
    
    /// 获取原始目标
    fn get_original_target(&self) -> Option<Rc<RefCell<Node>>>;
    
    /// 获取目标
    fn get_target(&self) -> Option<Rc<RefCell<Node>>>;
    
    /// 设置目标
    fn set_target(&mut self, target: Option<Rc<RefCell<Node>>>);
    
    /// 获取原始目标
    fn get_original_target_ref(&self) -> Option<&Rc<RefCell<Node>>>;
    
    /// 获取标签
    fn get_tag(&self) -> i32;
    
    /// 设置标签
    fn set_tag(&mut self, tag: i32);
    
    /// 开始执行动作
    fn start_with_target(&mut self, target: &Rc<RefCell<Node>>);
    
    /// 停止动作
    fn stop(&mut self);
    
    /// 更新动作
    fn update(&mut self, dt: f32);
    
    /// 获取是否完成
    fn is_done(&self) -> bool;
    
    /// 步进动作（用于时间映射）
    fn step(&mut self, dt: f32);
}

impl Clone for Box<dyn Action> {
    fn clone(&self) -> Self {
        self.clone_action()
    }
}

/// FiniteTimeAction - 有限时间动作基类
///
/// 在指定时间内完成的动作。
pub trait FiniteTimeAction: Action {
    /// 获取持续时间（秒）
    fn get_duration(&self) -> f32;
    
    /// 设置持续时间
    fn set_duration(&mut self, duration: f32);
    
    /// 反转动作
    fn reverse(&self) -> Box<dyn FiniteTimeAction>;
}

/// ActionInterval - 间隔动作基类
///
/// 持续一定时间的动作。
pub trait ActionInterval: FiniteTimeAction {
    /// 获取已用时间
    fn get_elapsed(&self) -> f32;
    
    /// 获取是否首次调用
    fn is_first_tick(&self) -> bool;
}

/// Speed - 速度修改动作
#[derive(Debug, Clone)]
pub struct Speed {
    inner: Box<dyn Action>,
    speed: f32,
    target: Option<Rc<RefCell<Node>>>,
    tag: i32,
}

impl Speed {
    pub fn new(action: Box<dyn Action>, speed: f32) -> Self {
        Self {
            inner: action,
            speed,
            target: None,
            tag: INVALID_TAG,
        }
    }
    
    pub fn with_action(action: Box<dyn Action>) -> Self {
        Self::new(action, 1.0)
    }
    
    pub fn inner(&self) -> &Box<dyn Action> {
        &self.inner
    }
    
    pub fn inner_mut(&mut self) -> &mut Box<dyn Action> {
        &mut self.inner
    }
    
    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed;
    }
    
    pub fn speed(&self) -> f32 {
        self.speed
    }
}

impl Action for Speed {
    fn clone_action(&self) -> Box<dyn Action> {
        Box::new(self.clone())
    }
    
    fn get_original_target(&self) -> Option<Rc<RefCell<Node>>> {
        self.target.clone()
    }
    
    fn get_target(&self) -> Option<Rc<RefCell<Node>>> {
        self.target.clone()
    }
    
    fn set_target(&mut self, target: Option<Rc<RefCell<Node>>>) {
        self.target = target;
        self.inner.set_target(target.clone());
    }
    
    fn get_original_target_ref(&self) -> Option<&Rc<RefCell<Node>>> {
        self.target.as_ref()
    }
    
    fn get_tag(&self) -> i32 {
        self.tag
    }
    
    fn set_tag(&mut self, tag: i32) {
        self.tag = tag;
    }
    
    fn start_with_target(&mut self, target: &Rc<RefCell<Node>>) {
        self.target = Some(Rc::clone(target));
        self.inner.start_with_target(target);
    }
    
    fn stop(&mut self) {
        self.inner.stop();
    }
    
    fn update(&mut self, dt: f32) {
        self.inner.update(dt * self.speed);
    }
    
    fn is_done(&self) -> bool {
        self.inner.is_done()
    }
    
    fn step(&mut self, dt: f32) {
        self.inner.step(dt * self.speed);
    }
}

/// Follow - 跟随动作
///
/// 让节点跟随另一个节点移动。
#[derive(Debug, Clone)]
pub struct Follow {
    target: Option<Rc<RefCell<Node>>>,
    followed_node: Option<Rc<RefCell<Node>>>,
    tag: i32,
    boundary_set: bool,
    boundary_rect: crate::math::geometry::Rect,
}

impl Follow {
    pub fn new(followed_node: Rc<RefCell<Node>>) -> Self {
        Self {
            target: None,
            followed_node: Some(followed_node),
            tag: INVALID_TAG,
            boundary_set: false,
            boundary_rect: crate::math::geometry::Rect::zero(),
        }
    }
    
    pub fn with_boundary(
        followed_node: Rc<RefCell<Node>>,
        rect: crate::math::geometry::Rect,
    ) -> Self {
        Self {
            target: None,
            followed_node: Some(followed_node),
            tag: INVALID_TAG,
            boundary_set: true,
            boundary_rect: rect,
        }
    }
    
    pub fn set_boundary(&mut self, rect: crate::math::geometry::Rect) {
        self.boundary_rect = rect;
        self.boundary_set = true;
    }
    
    pub fn boundary(&self) -> crate::math::geometry::Rect {
        self.boundary_rect
    }
    
    pub fn set_followed_node(&mut self, node: Option<Rc<RefCell<Node>>>) {
        self.followed_node = node;
    }
    
    pub fn followed_node(&self) -> Option<Rc<RefCell<Node>>> {
        self.followed_node.clone()
    }
}

impl Action for Follow {
    fn clone_action(&self) -> Box<dyn Action> {
        Box::new(self.clone())
    }
    
    fn get_original_target(&self) -> Option<Rc<RefCell<Node>>> {
        self.target.clone()
    }
    
    fn get_target(&self) -> Option<Rc<RefCell<Node>>> {
        self.target.clone()
    }
    
    fn set_target(&mut self, target: Option<Rc<RefCell<Node>>>) {
        self.target = target;
    }
    
    fn get_original_target_ref(&self) -> Option<&Rc<RefCell<Node>>> {
        self.target.as_ref()
    }
    
    fn get_tag(&self) -> i32 {
        self.tag
    }
    
    fn set_tag(&mut self, tag: i32) {
        self.tag = tag;
    }
    
    fn start_with_target(&mut self, target: &Rc<RefCell<Node>>) {
        self.target = Some(Rc::clone(target));
    }
    
    fn stop(&mut self) {
        // Follow 动作永不停止
    }
    
    fn update(&mut self, _dt: f32) {
        if let Some(ref followed) = self.followed_node {
            if let Some(ref target) = self.target {
                let followed_pos = followed.borrow().position();
                let mut temp_pos = followed_pos;
                
                if self.boundary_set {
                    let half_size = crate::math::Vec2::new(
                        self.boundary_rect.size.width / 2.0,
                        self.boundary_rect.size.height / 2.0,
                    );
                    
                    let left = self.boundary_rect.origin.x + half_size.x;
                    let right = self.boundary_rect.origin.x + self.boundary_rect.size.width - half_size.x;
                    let bottom = self.boundary_rect.origin.y + half_size.y;
                    let top = self.boundary_rect.origin.y + self.boundary_rect.size.height - half_size.y;
                    
                    temp_pos.x = temp_pos.x.clamp(left, right);
                    temp_pos.y = temp_pos.y.clamp(bottom, top);
                }
                
                target.borrow_mut().set_position(temp_pos);
            }
        }
    }
    
    fn is_done(&self) -> bool {
        false // Follow 动作永不完成
    }
    
    fn step(&mut self, dt: f32) {
        self.update(dt);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;
    use std::cell::RefCell;
    use crate::scene::Node;

    /// 测试用的简单动作实现
    #[derive(Debug, Clone)]
    struct TestAction {
        target: Option<Rc<RefCell<Node>>>,
        tag: i32,
        done: bool,
        duration: f32,
        elapsed: f32,
    }
    
    impl TestAction {
        fn new(duration: f32) -> Self {
            Self {
                target: None,
                tag: INVALID_TAG,
                done: false,
                duration,
                elapsed: 0.0,
            }
        }
    }
    
    impl Action for TestAction {
        fn clone_action(&self) -> Box<dyn Action> {
            Box::new(self.clone())
        }
        
        fn get_original_target(&self) -> Option<Rc<RefCell<Node>>> {
            self.target.clone()
        }
        
        fn get_target(&self) -> Option<Rc<RefCell<Node>>> {
            self.target.clone()
        }
        
        fn set_target(&mut self, target: Option<Rc<RefCell<Node>>>) {
            self.target = target;
        }
        
        fn get_original_target_ref(&self) -> Option<&Rc<RefCell<Node>>> {
            self.target.as_ref()
        }
        
        fn get_tag(&self) -> i32 {
            self.tag
        }
        
        fn set_tag(&mut self, tag: i32) {
            self.tag = tag;
        }
        
        fn start_with_target(&mut self, target: &Rc<RefCell<Node>>) {
            self.target = Some(Rc::clone(target));
        }
        
        fn stop(&mut self) {
            self.done = true;
        }
        
        fn update(&mut self, dt: f32) {
            self.elapsed += dt;
            if self.elapsed >= self.duration {
                self.done = true;
            }
        }
        
        fn is_done(&self) -> bool {
            self.done
        }
        
        fn step(&mut self, dt: f32) {
            self.update(dt);
        }
    }
    
    impl FiniteTimeAction for TestAction {
        fn get_duration(&self) -> f32 {
            self.duration
        }
        
        fn set_duration(&mut self, duration: f32) {
            self.duration = duration;
        }
        
        fn reverse(&self) -> Box<dyn FiniteTimeAction> {
            Box::new(self.clone())
        }
    }

    #[test]
    fn test_action_tag() {
        let mut action = TestAction::new(1.0);
        assert_eq!(action.get_tag(), INVALID_TAG);

        action.set_tag(100);
        assert_eq!(action.get_tag(), 100);
    }

    #[test]
    fn test_action_target() {
        let target = Rc::new(RefCell::new(Node::new()));
        let mut action = TestAction::new(1.0);

        action.start_with_target(&target);
        assert!(action.get_target().is_some());
        assert!(Rc::ptr_eq(&action.get_target().unwrap(), &target));
    }

    #[test]
    fn test_action_done() {
        let target = Rc::new(RefCell::new(Node::new()));
        let mut action = TestAction::new(0.1);

        action.start_with_target(&target);
        assert!(!action.is_done());

        action.update(0.15);
        assert!(action.is_done());
    }

    #[test]
    fn test_action_stop() {
        let target = Rc::new(RefCell::new(Node::new()));
        let mut action = TestAction::new(1.0);

        action.start_with_target(&target);
        action.stop();
        assert!(action.is_done());
    }

    #[test]
    fn test_action_clone() {
        let mut action1 = TestAction::new(1.0);
        action1.set_tag(100);

        let action2 = action1.clone_action();
        assert_eq!(action2.get_tag(), 100);
    }

    #[test]
    fn test_finite_time_action_duration() {
        let mut action = TestAction::new(2.5);
        assert_eq!(action.get_duration(), 2.5);

        action.set_duration(5.0);
        assert_eq!(action.get_duration(), 5.0);
    }

    #[test]
    fn test_finite_time_action_reverse() {
        let action = TestAction::new(1.0);
        let reversed = action.reverse();
        assert_eq!(reversed.get_duration(), 1.0);
    }

    #[test]
    fn test_speed_action() {
        let inner = Box::new(TestAction::new(1.0)) as Box<dyn Action>;
        let mut speed = Speed::new(inner, 2.0);

        assert_eq!(speed.speed(), 2.0);

        speed.set_speed(0.5);
        assert_eq!(speed.speed(), 0.5);
    }

    #[test]
    fn test_speed_action_update() {
        let target = Rc::new(RefCell::new(Node::new()));
        let inner = Box::new(TestAction::new(1.0)) as Box<dyn Action>;
        let mut speed = Speed::new(inner, 2.0);

        speed.start_with_target(&target);
        assert!(!speed.is_done());

        // 速度为 2.0，所以实际更新时间是 2 * dt
        speed.update(0.4); // 实际更新 0.8s
        assert!(!speed.is_done());

        speed.update(0.2); // 实际更新 0.4s, 累计 1.2s > 1.0s
        assert!(speed.is_done());
    }

    #[test]
    fn test_follow_action() {
        let followed = Rc::new(RefCell::new(Node::new()));
        followed.borrow_mut().set_position(crate::math::Vec2::new(100.0, 200.0));

        let mut follow = Follow::new(Rc::clone(&followed));
        follow.update(0.1);

        let target = Rc::new(RefCell::new(Node::new()));
        follow.start_with_target(&target);
        follow.update(0.1);

        assert_eq!(target.borrow().position(), crate::math::Vec2::new(100.0, 200.0));
    }

    #[test]
    fn test_follow_action_with_boundary() {
        let followed = Rc::new(RefCell::new(Node::new()));
        followed.borrow_mut().set_position(crate::math::Vec2::new(100.0, 100.0));

        let rect = crate::math::geometry::Rect::new(
            crate::math::Vec2::new(0.0, 0.0),
            crate::math::geometry::Size::new(100.0, 100.0),
        );
        let mut follow = Follow::with_boundary(Rc::clone(&followed), rect);

        let target = Rc::new(RefCell::new(Node::new()));
        follow.start_with_target(&target);
        follow.update(0.1);

        // 目标应该在边界内
        let pos = target.borrow().position();
        assert!(pos.x >= 50.0 && pos.x <= 50.0); // center x
        assert!(pos.y >= 50.0 && pos.y <= 50.0); // center y
    }

    #[test]
    fn test_follow_action_never_done() {
        let followed = Rc::new(RefCell::new(Node::new()));
        let follow = Follow::new(followed);
        assert!(!follow.is_done());

        let target = Rc::new(RefCell::new(Node::new()));
        let mut follow = Follow::new(followed);
        follow.start_with_target(&target);
        follow.update(1.0);
        assert!(!follow.is_done());
    }

    #[test]
    fn test_action_step() {
        let target = Rc::new(RefCell::new(Node::new()));
        let mut action = TestAction::new(1.0);

        action.start_with_target(&target);
        action.step(0.5);
        assert!(!action.is_done());

        action.step(0.6);
        assert!(action.is_done());
    }
}

use std::cell::RefCell;
use std::rc::Rc;

use crate::scene::Node;

use super::action::{Action, ActionInterval, FiniteTimeAction, INVALID_TAG};

pub struct Repeat {
    inner: Box<dyn FiniteTimeAction>,
    times: u32,
    total: u32,
    next_dt: f32,
    action_instant: bool,
    target: Option<Rc<RefCell<Node>>>,
    original_target: Option<Rc<RefCell<Node>>>,
    tag: i32,
    elapsed: f32,
    first_tick: bool,
    duration: f32,
}

impl std::fmt::Debug for Repeat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Repeat")
            .field("times", &self.times)
            .field("total", &self.total)
            .field("duration", &self.duration)
            .field("elapsed", &self.elapsed)
            .field("tag", &self.tag)
            .finish()
    }
}

impl Clone for Repeat {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.reverse().reverse(),
            times: self.times,
            total: self.total,
            next_dt: self.next_dt,
            action_instant: self.action_instant,
            target: self.target.clone(),
            original_target: self.original_target.clone(),
            tag: self.tag,
            elapsed: self.elapsed,
            first_tick: self.first_tick,
            duration: self.duration,
        }
    }
}

impl Repeat {
    pub fn new(action: Box<dyn FiniteTimeAction>, times: u32) -> Self {
        let duration = action.get_duration() * times as f32;
        let action_instant = action.get_duration() == 0.0;
        
        Self {
            inner: action,
            times,
            total: 0,
            next_dt: 0.0,
            action_instant,
            target: None,
            original_target: None,
            tag: INVALID_TAG,
            elapsed: 0.0,
            first_tick: true,
            duration,
        }
    }

    pub fn set_inner_action(&mut self, action: Box<dyn FiniteTimeAction>) {
        if !std::ptr::eq(&*self.inner as *const _, &*action as *const _) {
            let times = self.times;
            self.inner = action;
            self.duration = self.inner.get_duration() * times as f32;
        }
    }

    pub fn get_inner_action(&self) -> &dyn FiniteTimeAction {
        &*self.inner
    }
}

impl Action for Repeat {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn clone_action(&self) -> Box<dyn Action> {
        Box::new(self.clone())
    }

    fn get_original_target(&self) -> Option<Rc<RefCell<Node>>> {
        self.original_target.clone()
    }

    fn get_target(&self) -> Option<Rc<RefCell<Node>>> {
        self.target.clone()
    }

    fn set_target(&mut self, target: Option<Rc<RefCell<Node>>>) {
        self.target = target;
    }

    fn get_original_target_ref(&self) -> Option<&Rc<RefCell<Node>>> {
        self.original_target.as_ref()
    }

    fn get_tag(&self) -> i32 {
        self.tag
    }

    fn set_tag(&mut self, tag: i32) {
        self.tag = tag;
    }

    fn start_with_target(&mut self, target: &Rc<RefCell<Node>>) {
        self.total = 0;
        self.next_dt = self.inner.get_duration() / self.duration;
        self.target = Some(target.clone());
        self.original_target = Some(target.clone());
        self.elapsed = 0.0;
        self.first_tick = true;
        self.inner.start_with_target(target);
    }

    fn stop(&mut self) {
        self.inner.stop();
        self.target = None;
    }

    fn update(&mut self, dt: f32) {
        // 对于瞬时动作，一次性执行所有重复
        if self.action_instant {
            if self.total < self.times {
                loop {
                    if let Some(target) = &self.target {
                        self.inner.update(1.0);
                        self.inner.stop();
                        self.total += 1;

                        if self.total >= self.times {
                            break;
                        }

                        self.inner.start_with_target(target);
                    } else {
                        break;
                    }
                }
            }
            return;
        }

        // 普通动作的重复逻辑
        if dt >= self.next_dt {
            loop {
                if self.target.is_some() {
                    self.inner.update(1.0);
                    self.inner.stop();
                    self.total += 1;

                    if self.total == self.times {
                        break;
                    }

                    self.next_dt = self.inner.get_duration() / self.duration * (self.total + 1) as f32;
                    
                    if let Some(target) = &self.target {
                        self.inner.start_with_target(target);
                    }

                    if dt < self.next_dt {
                        let t = if self.next_dt > 0.0 {
                            (dt - self.next_dt + self.inner.get_duration()) / self.inner.get_duration()
                        } else {
                            1.0
                        };
                        self.inner.update(t.max(0.0));
                        break;
                    }
                } else {
                    break;
                }
            }
        } else {
            let t = if self.next_dt > 0.0 {
                (dt * self.times as f32) % 1.0
            } else if dt == 1.0 {
                1.0
            } else {
                0.0
            };
            self.inner.update(t);
        }
    }

    fn is_done(&self) -> bool {
        self.total >= self.times
    }

    fn step(&mut self, dt: f32) {
        self.elapsed += dt;
        let t = if self.duration > 0.0 {
            (self.elapsed / self.duration).min(1.0)
        } else {
            1.0
        };
        self.update(t);
        self.first_tick = false;
    }
}

impl FiniteTimeAction for Repeat {
    fn get_duration(&self) -> f32 {
        self.duration
    }

    fn set_duration(&mut self, duration: f32) {
        self.duration = duration;
    }

    fn reverse(&self) -> Box<dyn FiniteTimeAction> {
        Box::new(Repeat::new(self.inner.reverse(), self.times))
    }
}

impl ActionInterval for Repeat {
    fn get_elapsed(&self) -> f32 {
        self.elapsed
    }

    fn is_first_tick(&self) -> bool {
        self.first_tick
    }
}

pub struct RepeatForever {
    inner: Box<dyn FiniteTimeAction>,
    target: Option<Rc<RefCell<Node>>>,
    original_target: Option<Rc<RefCell<Node>>>,
    tag: i32,
    elapsed: f32,
    first_tick: bool,
}

impl std::fmt::Debug for RepeatForever {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RepeatForever")
            .field("elapsed", &self.elapsed)
            .field("tag", &self.tag)
            .finish()
    }
}

impl Clone for RepeatForever {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.reverse().reverse(),
            target: self.target.clone(),
            original_target: self.original_target.clone(),
            tag: self.tag,
            elapsed: self.elapsed,
            first_tick: self.first_tick,
        }
    }
}

impl RepeatForever {
    pub fn new(action: Box<dyn FiniteTimeAction>) -> Self {
        Self {
            inner: action,
            target: None,
            original_target: None,
            tag: INVALID_TAG,
            elapsed: 0.0,
            first_tick: true,
        }
    }

    pub fn set_inner_action(&mut self, action: Box<dyn FiniteTimeAction>) {
        if !std::ptr::eq(&*self.inner as *const _, &*action as *const _) {
            self.inner = action;
        }
    }

    pub fn get_inner_action(&self) -> &dyn FiniteTimeAction {
        &*self.inner
    }
}

impl Action for RepeatForever {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn clone_action(&self) -> Box<dyn Action> {
        Box::new(self.clone())
    }

    fn get_original_target(&self) -> Option<Rc<RefCell<Node>>> {
        self.original_target.clone()
    }

    fn get_target(&self) -> Option<Rc<RefCell<Node>>> {
        self.target.clone()
    }

    fn set_target(&mut self, target: Option<Rc<RefCell<Node>>>) {
        self.target = target;
    }

    fn get_original_target_ref(&self) -> Option<&Rc<RefCell<Node>>> {
        self.original_target.as_ref()
    }

    fn get_tag(&self) -> i32 {
        self.tag
    }

    fn set_tag(&mut self, tag: i32) {
        self.tag = tag;
    }

    fn start_with_target(&mut self, target: &Rc<RefCell<Node>>) {
        self.target = Some(target.clone());
        self.original_target = Some(target.clone());
        self.elapsed = 0.0;
        self.first_tick = true;
        self.inner.start_with_target(target);
    }

    fn stop(&mut self) {
        self.inner.stop();
        self.target = None;
    }

    fn update(&mut self, dt: f32) {
        self.inner.update(dt);
        
        if self.inner.is_done() {
            // 内部动作完成，重新开始
            self.inner.stop();
            if let Some(target) = &self.target {
                self.inner.start_with_target(target);
            }
            // 如果有多余时间，继续更新
            self.inner.update(0.0);
        }
    }

    fn is_done(&self) -> bool {
        false
    }

    fn step(&mut self, dt: f32) {
        self.elapsed += dt;
        self.inner.step(dt);
        self.first_tick = false;
    }
}

impl FiniteTimeAction for RepeatForever {
    fn get_duration(&self) -> f32 {
        f32::INFINITY
    }

    fn set_duration(&mut self, _duration: f32) {}

    fn reverse(&self) -> Box<dyn FiniteTimeAction> {
        Box::new(RepeatForever::new(self.inner.reverse()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::action::action_interval::MoveBy;
    use crate::math::Vec2;

    #[test]
    fn test_repeat_creation() {
        let move_action = Box::new(MoveBy::new(1.0, Vec2::new(100.0, 0.0)));
        let repeat = Repeat::new(move_action, 3);
        
        assert_eq!(repeat.get_duration(), 3.0);
        assert_eq!(repeat.times, 3);
    }

    #[test]
    fn test_repeat_execution() {
        let move_action = Box::new(MoveBy::new(1.0, Vec2::new(100.0, 0.0)));
        let mut repeat = Repeat::new(move_action, 3);
        
        let node = Rc::new(RefCell::new(Node::new()));
        repeat.start_with_target(&node);
        
        repeat.update(0.0);
        assert!(!repeat.is_done());
        
        repeat.update(0.33);
        assert!(!repeat.is_done());
        
        repeat.update(0.66);
        assert!(!repeat.is_done());
        
        repeat.update(1.0);
        assert!(repeat.is_done());
    }

    #[test]
    fn test_repeat_forever_creation() {
        let move_action = Box::new(MoveBy::new(1.0, Vec2::new(100.0, 0.0)));
        let repeat = RepeatForever::new(move_action);
        
        assert!(repeat.get_duration().is_infinite());
    }

    #[test]
    fn test_repeat_forever_never_done() {
        let move_action = Box::new(MoveBy::new(1.0, Vec2::new(100.0, 0.0)));
        let mut repeat = RepeatForever::new(move_action);
        
        let node = Rc::new(RefCell::new(Node::new()));
        repeat.start_with_target(&node);
        
        for _ in 0..100 {
            repeat.update(0.5);
            assert!(!repeat.is_done());
        }
    }

    #[test]
    fn test_repeat_with_instant_action() {
        use crate::action::action_instant::Hide;
        
        let hide = Box::new(Hide::new());
        let mut repeat = Repeat::new(hide, 5);
        
        let node = Rc::new(RefCell::new(Node::new()));
        node.borrow_mut().set_visible(true);
        
        repeat.start_with_target(&node);
        repeat.update(1.0);
        
        assert!(!node.borrow().is_visible());
        assert!(repeat.is_done());
    }

    #[test]
    fn test_repeat_reverse() {
        let move_action = Box::new(MoveBy::new(1.0, Vec2::new(100.0, 50.0)));
        let repeat = Repeat::new(move_action, 3);
        
        let _reversed = repeat.reverse();
    }

    #[test]
    fn test_repeat_forever_stop() {
        let move_action = Box::new(MoveBy::new(1.0, Vec2::new(100.0, 0.0)));
        let mut repeat = RepeatForever::new(move_action);
        
        let node = Rc::new(RefCell::new(Node::new()));
        repeat.start_with_target(&node);
        
        repeat.update(0.5);
        repeat.stop();
        
        assert!(repeat.get_target().is_none());
    }
}

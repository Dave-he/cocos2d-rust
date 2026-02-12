use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use crate::scene::node::Node;
use super::action::{Action, FiniteTimeAction};
use super::action_interval::ActionIntervalImpl;

#[derive(Debug)]
pub struct Sequence {
    base: ActionIntervalImpl,
    actions: [Option<Rc<RefCell<dyn FiniteTimeAction>>>; 2],
    split: f32,
    last: i32,
}

impl Sequence {
    pub fn create_with_two_actions(
        action_one: Rc<RefCell<dyn FiniteTimeAction>>,
        action_two: Rc<RefCell<dyn FiniteTimeAction>>
    ) -> Result<Rc<RefCell<Self>>, String> {
        let duration = {
            let a1 = action_one.borrow();
            let a2 = action_two.borrow();
            a1.get_duration() + a2.get_duration()
        };

        let base = ActionIntervalImpl::new(duration);

        let sequence = Rc::new(RefCell::new(Sequence {
            base,
            actions: [Some(action_one), Some(action_two)],
            split: 0.0,
            last: -1,
        }));

        Ok(sequence)
    }

    pub fn create(actions: Vec<Rc<RefCell<dyn FiniteTimeAction>>>) -> Result<Rc<RefCell<Self>>, String> {
        if actions.is_empty() {
            return Err("Sequence::create: actions array is empty".to_string());
        }

        if actions.len() == 1 {
            return Err("Sequence::create: need at least 2 actions".to_string());
        }

        let mut prev = actions[0].clone();
        for i in 1..actions.len() - 1 {
            prev = Self::create_with_two_actions(prev, actions[i].clone())? as Rc<RefCell<dyn FiniteTimeAction>>;
        }

        Self::create_with_two_actions(prev, actions[actions.len() - 1].clone())
    }
}

impl Action for Sequence {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_action(&self) -> Box<dyn Action> {
        Box::new(ActionIntervalImpl::new(self.base.get_duration()))
    }

    fn get_original_target(&self) -> Option<Rc<RefCell<Node>>> {
        self.base.get_original_target()
    }

    fn get_target(&self) -> Option<Rc<RefCell<Node>>> {
        self.base.get_target()
    }

    fn set_target(&mut self, target: Option<Rc<RefCell<Node>>>) {
        self.base.set_target(target);
    }

    fn get_original_target_ref(&self) -> Option<&Rc<RefCell<Node>>> {
        self.base.get_original_target_ref()
    }

    fn get_tag(&self) -> i32 {
        self.base.get_tag()
    }

    fn set_tag(&mut self, tag: i32) {
        self.base.set_tag(tag);
    }

    fn start_with_target(&mut self, target: &Rc<RefCell<Node>>) {
        if let (Some(a0), Some(a1)) = (&self.actions[0], &self.actions[1]) {
            let duration = self.base.get_duration();
            if duration > f32::EPSILON {
                let a0_duration = a0.borrow().get_duration();
                self.split = if a0_duration > f32::EPSILON {
                    a0_duration / duration
                } else {
                    0.0
                };
            }
        }

        self.base.start_with_target(target);
        self.last = -1;
    }

    fn stop(&mut self) {
        if self.last != -1 {
            let idx = self.last as usize;
            if let Some(action) = &self.actions[idx] {
                action.borrow_mut().stop();
            }
        }
        self.base.stop();
    }

    fn update(&mut self, t: f32) {
        let mut found = 0;
        let mut new_t = 0.0;

        if t < self.split {
            found = 0;
            new_t = if self.split != 0.0 {
                t / self.split
            } else {
                1.0
            };
        } else {
            found = 1;
            new_t = if self.split == 1.0 {
                1.0
            } else {
                (t - self.split) / (1.0 - self.split)
            };
        }

        if found == 1 {
            if self.last == -1 {
                if let (Some(a0), Some(target)) = (&self.actions[0], &self.base.get_target()) {
                    a0.borrow_mut().start_with_target(target);
                    a0.borrow_mut().update(1.0);
                    a0.borrow_mut().stop();
                }
            } else if self.last == 0 {
                if let Some(a0) = &self.actions[0] {
                    a0.borrow_mut().update(1.0);
                    a0.borrow_mut().stop();
                }
            }
        } else if found == 0 && self.last == 1 {
            if let Some(a1) = &self.actions[1] {
                a1.borrow_mut().update(0.0);
                a1.borrow_mut().stop();
            }
        }

        if let Some(action) = &self.actions[found as usize] {
            let is_done = action.borrow().is_done();
            if found == self.last && is_done {
                return;
            }

            if found != self.last {
                if let Some(target) = self.base.get_target() {
                    action.borrow_mut().start_with_target(&target);
                }
            }
            action.borrow_mut().update(new_t);
            self.last = found;
        }
    }

    fn is_done(&self) -> bool {
        self.base.is_done()
    }

    fn step(&mut self, dt: f32) {
        self.base.step(dt);
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl FiniteTimeAction for Sequence {
    fn get_duration(&self) -> f32 {
        self.base.get_duration()
    }

    fn set_duration(&mut self, duration: f32) {
        self.base.set_duration(duration);
    }

    fn reverse(&self) -> Box<dyn FiniteTimeAction> {
        Box::new(ActionIntervalImpl::new(self.base.get_duration()))
    }
}

#[cfg(test)]
#[path = "action_composite_tests.rs"]
mod tests;

#[derive(Debug)]
pub struct Spawn {
    base: ActionIntervalImpl,
    one: Option<Rc<RefCell<dyn FiniteTimeAction>>>,
    two: Option<Rc<RefCell<dyn FiniteTimeAction>>>,
}

impl Spawn {
    pub fn create_with_two_actions(
        action1: Rc<RefCell<dyn FiniteTimeAction>>,
        action2: Rc<RefCell<dyn FiniteTimeAction>>
    ) -> Result<Rc<RefCell<Self>>, String> {
        let (d1, d2) = {
            let a1 = action1.borrow();
            let a2 = action2.borrow();
            (a1.get_duration(), a2.get_duration())
        };

        let max_duration = d1.max(d2);
        let base = ActionIntervalImpl::new(max_duration);

        let mut one = Some(action1);
        let mut two = Some(action2);

        if d1 > d2 {
            if let Some(a2) = two.clone() {
                use super::action_interval::DelayTime;
                let delay = Rc::new(RefCell::new(DelayTime::new(d1 - d2)));
                let seq = Sequence::create_with_two_actions(
                    a2,
                    delay as Rc<RefCell<dyn FiniteTimeAction>>
                )?;
                two = Some(seq as Rc<RefCell<dyn FiniteTimeAction>>);
            }
        } else if d1 < d2 {
            if let Some(a1) = one.clone() {
                use super::action_interval::DelayTime;
                let delay = Rc::new(RefCell::new(DelayTime::new(d2 - d1)));
                let seq = Sequence::create_with_two_actions(
                    a1,
                    delay as Rc<RefCell<dyn FiniteTimeAction>>
                )?;
                one = Some(seq as Rc<RefCell<dyn FiniteTimeAction>>);
            }
        }

        let spawn = Rc::new(RefCell::new(Spawn {
            base,
            one,
            two,
        }));

        Ok(spawn)
    }

    pub fn create(actions: Vec<Rc<RefCell<dyn FiniteTimeAction>>>) -> Result<Rc<RefCell<Self>>, String> {
        if actions.is_empty() {
            return Err("Spawn::create: actions array is empty".to_string());
        }

        if actions.len() == 1 {
            return Err("Spawn::create: need at least 2 actions".to_string());
        }

        let mut prev = actions[0].clone();
        for i in 1..actions.len() - 1 {
            prev = Self::create_with_two_actions(prev, actions[i].clone())? as Rc<RefCell<dyn FiniteTimeAction>>;
        }

        Self::create_with_two_actions(prev, actions[actions.len() - 1].clone())
    }
}

impl Action for Spawn {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_action(&self) -> Box<dyn Action> {
        Box::new(ActionIntervalImpl::new(self.base.get_duration()))
    }

    fn get_original_target(&self) -> Option<Rc<RefCell<Node>>> {
        self.base.get_original_target()
    }

    fn get_target(&self) -> Option<Rc<RefCell<Node>>> {
        self.base.get_target()
    }

    fn set_target(&mut self, target: Option<Rc<RefCell<Node>>>) {
        self.base.set_target(target);
    }

    fn get_original_target_ref(&self) -> Option<&Rc<RefCell<Node>>> {
        self.base.get_original_target_ref()
    }

    fn get_tag(&self) -> i32 {
        self.base.get_tag()
    }

    fn set_tag(&mut self, tag: i32) {
        self.base.set_tag(tag);
    }

    fn start_with_target(&mut self, target: &Rc<RefCell<Node>>) {
        self.base.start_with_target(target);
        
        if let Some(one) = &self.one {
            one.borrow_mut().start_with_target(target);
        }
        if let Some(two) = &self.two {
            two.borrow_mut().start_with_target(target);
        }
    }

    fn stop(&mut self) {
        if let Some(one) = &self.one {
            one.borrow_mut().stop();
        }
        if let Some(two) = &self.two {
            two.borrow_mut().stop();
        }
        self.base.stop();
    }

    fn update(&mut self, time: f32) {
        if let Some(one) = &self.one {
            one.borrow_mut().update(time);
        }
        if let Some(two) = &self.two {
            two.borrow_mut().update(time);
        }
    }

    fn is_done(&self) -> bool {
        self.base.is_done()
    }

    fn step(&mut self, dt: f32) {
        self.base.step(dt);
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl FiniteTimeAction for Spawn {
    fn get_duration(&self) -> f32 {
        self.base.get_duration()
    }

    fn set_duration(&mut self, duration: f32) {
        self.base.set_duration(duration);
    }

    fn reverse(&self) -> Box<dyn FiniteTimeAction> {
        Box::new(ActionIntervalImpl::new(self.base.get_duration()))
    }
}

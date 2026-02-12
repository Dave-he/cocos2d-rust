use std::cell::RefCell;
use std::rc::Rc;

use crate::scene::Node;

use super::action::{Action, FiniteTimeAction, INVALID_TAG};

pub trait CallbackFn: FnMut() + 'static {}
impl<T> CallbackFn for T where T: FnMut() + 'static {}

#[derive(Clone)]
pub struct CallFunc {
    target: Option<Rc<RefCell<Node>>>,
    original_target: Option<Rc<RefCell<Node>>>,
    tag: i32,
    done: bool,
    callback: Option<Rc<RefCell<Box<dyn CallbackFn>>>>,
}

impl std::fmt::Debug for CallFunc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CallFunc")
            .field("tag", &self.tag)
            .field("done", &self.done)
            .field("has_callback", &self.callback.is_some())
            .finish()
    }
}

impl CallFunc {
    pub fn new<F>(callback: F) -> Self
    where
        F: FnMut() + 'static,
    {
        Self {
            target: None,
            original_target: None,
            tag: INVALID_TAG,
            done: false,
            callback: Some(Rc::new(RefCell::new(Box::new(callback)))),
        }
    }

    fn execute(&mut self) {
        if let Some(cb) = &self.callback {
            let mut callback = cb.borrow_mut();
            callback();
        }
        self.done = true;
    }
}

impl Action for CallFunc {
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
        self.done = false;
    }

    fn stop(&mut self) {
        self.done = true;
    }

    fn update(&mut self, _dt: f32) {
        if !self.done {
            self.execute();
        }
    }

    fn is_done(&self) -> bool {
        self.done
    }

    fn step(&mut self, _dt: f32) {
        if !self.done {
            self.execute();
        }
    }
}

impl FiniteTimeAction for CallFunc {
    fn get_duration(&self) -> f32 {
        0.0
    }

    fn set_duration(&mut self, _duration: f32) {}

    fn reverse(&self) -> Box<dyn FiniteTimeAction> {
        Box::new(self.clone())
    }
}

pub trait CallbackWithNodeFn: FnMut(&Rc<RefCell<Node>>) + 'static {}
impl<T> CallbackWithNodeFn for T where T: FnMut(&Rc<RefCell<Node>>) + 'static {}

#[derive(Clone)]
pub struct CallFuncN {
    target: Option<Rc<RefCell<Node>>>,
    original_target: Option<Rc<RefCell<Node>>>,
    tag: i32,
    done: bool,
    callback: Option<Rc<RefCell<Box<dyn CallbackWithNodeFn>>>>,
}

impl std::fmt::Debug for CallFuncN {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CallFuncN")
            .field("tag", &self.tag)
            .field("done", &self.done)
            .field("has_callback", &self.callback.is_some())
            .finish()
    }
}

impl CallFuncN {
    pub fn new<F>(callback: F) -> Self
    where
        F: FnMut(&Rc<RefCell<Node>>) + 'static,
    {
        Self {
            target: None,
            original_target: None,
            tag: INVALID_TAG,
            done: false,
            callback: Some(Rc::new(RefCell::new(Box::new(callback)))),
        }
    }

    fn execute(&mut self) {
        if let Some(target) = &self.target {
            if let Some(cb) = &self.callback {
                let mut callback = cb.borrow_mut();
                callback(target);
            }
        }
        self.done = true;
    }
}

impl Action for CallFuncN {
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
        self.done = false;
    }

    fn stop(&mut self) {
        self.done = true;
    }

    fn update(&mut self, _dt: f32) {
        if !self.done {
            self.execute();
        }
    }

    fn is_done(&self) -> bool {
        self.done
    }

    fn step(&mut self, _dt: f32) {
        if !self.done {
            self.execute();
        }
    }
}

impl FiniteTimeAction for CallFuncN {
    fn get_duration(&self) -> f32 {
        0.0
    }

    fn set_duration(&mut self, _duration: f32) {}

    fn reverse(&self) -> Box<dyn FiniteTimeAction> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct Hide {
    target: Option<Rc<RefCell<Node>>>,
    original_target: Option<Rc<RefCell<Node>>>,
    tag: i32,
    done: bool,
}

impl Hide {
    pub fn new() -> Self {
        Self {
            target: None,
            original_target: None,
            tag: INVALID_TAG,
            done: false,
        }
    }
}

impl Default for Hide {
    fn default() -> Self {
        Self::new()
    }
}

impl Action for Hide {
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
        self.done = false;
    }

    fn stop(&mut self) {
        self.done = true;
    }

    fn update(&mut self, _dt: f32) {
        if let Some(target) = &self.target {
            target.borrow_mut().set_visible(false);
        }
        self.done = true;
    }

    fn is_done(&self) -> bool {
        self.done
    }

    fn step(&mut self, _dt: f32) {
        self.update(_dt);
    }
}

impl FiniteTimeAction for Hide {
    fn get_duration(&self) -> f32 {
        0.0
    }

    fn set_duration(&mut self, _duration: f32) {}

    fn reverse(&self) -> Box<dyn FiniteTimeAction> {
        Box::new(Show::new())
    }
}

#[derive(Debug, Clone)]
pub struct Show {
    target: Option<Rc<RefCell<Node>>>,
    original_target: Option<Rc<RefCell<Node>>>,
    tag: i32,
    done: bool,
}

impl Show {
    pub fn new() -> Self {
        Self {
            target: None,
            original_target: None,
            tag: INVALID_TAG,
            done: false,
        }
    }
}

impl Default for Show {
    fn default() -> Self {
        Self::new()
    }
}

impl Action for Show {
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
        self.done = false;
    }

    fn stop(&mut self) {
        self.done = true;
    }

    fn update(&mut self, _dt: f32) {
        if let Some(target) = &self.target {
            target.borrow_mut().set_visible(true);
        }
        self.done = true;
    }

    fn is_done(&self) -> bool {
        self.done
    }

    fn step(&mut self, _dt: f32) {
        self.update(_dt);
    }
}

impl FiniteTimeAction for Show {
    fn get_duration(&self) -> f32 {
        0.0
    }

    fn set_duration(&mut self, _duration: f32) {}

    fn reverse(&self) -> Box<dyn FiniteTimeAction> {
        Box::new(Hide::new())
    }
}

#[derive(Debug, Clone)]
pub struct ToggleVisibility {
    target: Option<Rc<RefCell<Node>>>,
    original_target: Option<Rc<RefCell<Node>>>,
    tag: i32,
    done: bool,
}

impl ToggleVisibility {
    pub fn new() -> Self {
        Self {
            target: None,
            original_target: None,
            tag: INVALID_TAG,
            done: false,
        }
    }
}

impl Default for ToggleVisibility {
    fn default() -> Self {
        Self::new()
    }
}

impl Action for ToggleVisibility {
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
        self.done = false;
    }

    fn stop(&mut self) {
        self.done = true;
    }

    fn update(&mut self, _dt: f32) {
        if let Some(target) = &self.target {
            let mut node = target.borrow_mut();
            let is_visible = node.is_visible();
            node.set_visible(!is_visible);
        }
        self.done = true;
    }

    fn is_done(&self) -> bool {
        self.done
    }

    fn step(&mut self, _dt: f32) {
        self.update(_dt);
    }
}

impl FiniteTimeAction for ToggleVisibility {
    fn get_duration(&self) -> f32 {
        0.0
    }

    fn set_duration(&mut self, _duration: f32) {}

    fn reverse(&self) -> Box<dyn FiniteTimeAction> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_call_func_creation() {
        let mut called = false;
        let _action = CallFunc::new(move || {
            called = true;
        });
    }

    #[test]
    fn test_call_func_execution() {
        use std::sync::{Arc, Mutex};
        
        let counter = Arc::new(Mutex::new(0));
        let counter_clone = counter.clone();
        
        let mut action = CallFunc::new(move || {
            let mut c = counter_clone.lock().unwrap();
            *c += 1;
        });
        
        let node = Rc::new(RefCell::new(Node::new()));
        
        action.start_with_target(&node);
        action.update(0.0);
        
        assert_eq!(*counter.lock().unwrap(), 1);
        assert!(action.is_done());
    }

    #[test]
    fn test_call_func_n() {
        use std::sync::{Arc, Mutex};
        
        let node_name = Arc::new(Mutex::new(String::new()));
        let node_name_clone = node_name.clone();
        
        let mut action = CallFuncN::new(move |node: &Rc<RefCell<Node>>| {
            let n = node.borrow();
            let mut name = node_name_clone.lock().unwrap();
            *name = n.name().to_string();
        });
        
        let node = Rc::new(RefCell::new(Node::new()));
        node.borrow_mut().set_name("TestNode");
        
        action.start_with_target(&node);
        action.update(0.0);
        
        assert_eq!(*node_name.lock().unwrap(), "TestNode");
        assert!(action.is_done());
    }

    #[test]
    fn test_hide_action() {
        let mut action = Hide::new();
        let node = Rc::new(RefCell::new(Node::new()));
        
        node.borrow_mut().set_visible(true);
        
        action.start_with_target(&node);
        action.update(0.0);
        
        assert!(!node.borrow().is_visible());
        assert!(action.is_done());
    }

    #[test]
    fn test_show_action() {
        let mut action = Show::new();
        let node = Rc::new(RefCell::new(Node::new()));
        
        node.borrow_mut().set_visible(false);
        
        action.start_with_target(&node);
        action.update(0.0);
        
        assert!(node.borrow().is_visible());
        assert!(action.is_done());
    }

    #[test]
    fn test_toggle_visibility() {
        let mut action = ToggleVisibility::new();
        let node = Rc::new(RefCell::new(Node::new()));
        
        node.borrow_mut().set_visible(true);
        action.start_with_target(&node);
        action.update(0.0);
        assert!(!node.borrow().is_visible());
        
        let mut action2 = ToggleVisibility::new();
        action2.start_with_target(&node);
        action2.update(0.0);
        assert!(node.borrow().is_visible());
    }

    #[test]
    fn test_show_hide_reverse() {
        let show = Show::new();
        let hide = show.reverse();
        
        let node = Rc::new(RefCell::new(Node::new()));
        node.borrow_mut().set_visible(true);
        
        let mut hide_action = hide;
        hide_action.start_with_target(&node);
        hide_action.update(0.0);
        
        assert!(!node.borrow().is_visible());
    }
}

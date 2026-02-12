use crate::action::{Action, ActionInterval, ActionIntervalImpl, FiniteTimeAction};
use crate::scene::Node;
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

/// ActionEase - 缓动动作基类
#[derive(Debug)]
pub struct ActionEase {
    base: ActionIntervalImpl,
    inner: Option<Box<dyn ActionInterval>>,
}

impl ActionEase {
    pub fn new(action: Box<dyn ActionInterval>) -> Self {
        let duration = action.get_duration();
        Self {
            base: ActionIntervalImpl::new(duration),
            inner: Some(action),
        }
    }

    pub fn get_inner_action(&self) -> Option<&dyn ActionInterval> {
        self.inner.as_ref().map(|a| a.as_ref())
    }
}

impl Action for ActionEase {
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
        if let Some(inner) = &mut self.inner {
            inner.start_with_target(target);
        }
    }

    fn stop(&mut self) {
        if let Some(inner) = &mut self.inner {
            inner.stop();
        }
        self.base.stop();
    }

    fn update(&mut self, time: f32) {
        if let Some(inner) = &mut self.inner {
            inner.update(time);
        }
    }

    fn is_done(&self) -> bool {
        self.base.is_done()
    }

    fn step(&mut self, dt: f32) {
        self.base.step(dt);
    }
}

impl FiniteTimeAction for ActionEase {
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

impl ActionInterval for ActionEase {
    fn get_elapsed(&self) -> f32 {
        FiniteTimeAction::get_elapsed(&self.base)
    }

    fn is_first_tick(&self) -> bool {
        self.base.is_first_tick()
    }
}

/// EaseRateAction - 带速率参数的缓动动作
#[derive(Debug)]
pub struct EaseRateAction {
    base: ActionIntervalImpl,
    inner: Option<Box<dyn ActionInterval>>,
    rate: f32,
}

impl EaseRateAction {
    pub fn new(action: Box<dyn ActionInterval>, rate: f32) -> Self {
        let duration = action.get_duration();
        Self {
            base: ActionIntervalImpl::new(duration),
            inner: Some(action),
            rate,
        }
    }

    pub fn get_rate(&self) -> f32 {
        self.rate
    }

    pub fn set_rate(&mut self, rate: f32) {
        self.rate = rate;
    }
}

impl Action for EaseRateAction {
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
        if let Some(inner) = &mut self.inner {
            inner.start_with_target(target);
        }
    }

    fn stop(&mut self) {
        if let Some(inner) = &mut self.inner {
            inner.stop();
        }
        self.base.stop();
    }

    fn update(&mut self, time: f32) {
        if let Some(inner) = &mut self.inner {
            inner.update(time.powf(self.rate));
        }
    }

    fn is_done(&self) -> bool {
        self.base.is_done()
    }

    fn step(&mut self, dt: f32) {
        self.base.step(dt);
    }
}

impl FiniteTimeAction for EaseRateAction {
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

impl ActionInterval for EaseRateAction {
    fn get_elapsed(&self) -> f32 {
        FiniteTimeAction::get_elapsed(&self.base)
    }

    fn is_first_tick(&self) -> bool {
        self.base.is_first_tick()
    }
}

/// EaseIn - 缓入动作
#[derive(Debug)]
pub struct EaseIn {
    base: EaseRateAction,
}

impl EaseIn {
    pub fn new(action: Box<dyn ActionInterval>, rate: f32) -> Self {
        Self {
            base: EaseRateAction::new(action, rate),
        }
    }
}

impl Action for EaseIn {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_action(&self) -> Box<dyn Action> {
        self.base.clone_action()
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
    }

    fn stop(&mut self) {
        self.base.stop();
    }

    fn update(&mut self, time: f32) {
        self.base.update(time);
    }

    fn is_done(&self) -> bool {
        self.base.is_done()
    }

    fn step(&mut self, dt: f32) {
        self.base.step(dt);
    }
}

impl FiniteTimeAction for EaseIn {
    fn get_duration(&self) -> f32 {
        self.base.get_duration()
    }

    fn set_duration(&mut self, duration: f32) {
        self.base.set_duration(duration);
    }

    fn reverse(&self) -> Box<dyn FiniteTimeAction> {
        self.base.reverse()
    }
}

impl ActionInterval for EaseIn {
    fn get_elapsed(&self) -> f32 {
        FiniteTimeAction::get_elapsed(&self.base)
    }

    fn is_first_tick(&self) -> bool {
        self.base.is_first_tick()
    }
}

/// EaseOut - 缓出动作
#[derive(Debug)]
pub struct EaseOut {
    base: EaseRateAction,
}

impl EaseOut {
    pub fn new(action: Box<dyn ActionInterval>, rate: f32) -> Self {
        Self {
            base: EaseRateAction::new(action, rate),
        }
    }
}

impl Action for EaseOut {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_action(&self) -> Box<dyn Action> {
        self.base.clone_action()
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
    }

    fn stop(&mut self) {
        self.base.stop();
    }

    fn update(&mut self, time: f32) {
        self.base.update(time);
    }

    fn is_done(&self) -> bool {
        self.base.is_done()
    }

    fn step(&mut self, dt: f32) {
        self.base.step(dt);
    }
}

impl FiniteTimeAction for EaseOut {
    fn get_duration(&self) -> f32 {
        self.base.get_duration()
    }

    fn set_duration(&mut self, duration: f32) {
        self.base.set_duration(duration);
    }

    fn reverse(&self) -> Box<dyn FiniteTimeAction> {
        self.base.reverse()
    }

    fn get_elapsed(&self) -> f32 {
        FiniteTimeAction::get_elapsed(&self.base)
    }
}

impl ActionInterval for EaseOut {
    fn get_elapsed(&self) -> f32 {
        ActionInterval::get_elapsed(&self.base)
    }

    fn is_first_tick(&self) -> bool {
        self.base.is_first_tick()
    }
}

/// EaseInOut - 缓入缓出动作
#[derive(Debug)]
pub struct EaseInOut {
    base: EaseRateAction,
}

impl EaseInOut {
    pub fn new(action: Box<dyn ActionInterval>, rate: f32) -> Self {
        Self {
            base: EaseRateAction::new(action, rate),
        }
    }
}

impl Action for EaseInOut {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_action(&self) -> Box<dyn Action> {
        self.base.clone_action()
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
    }

    fn stop(&mut self) {
        self.base.stop();
    }

    fn update(&mut self, time: f32) {
        let new_time = if time < 0.5 {
            0.5 * time.powf(self.base.get_rate())
        } else {
            1.0 - 0.5 * (2.0 - 2.0 * time).powf(self.base.get_rate())
        };
        self.base.update(new_time);
    }

    fn is_done(&self) -> bool {
        self.base.is_done()
    }

    fn step(&mut self, dt: f32) {
        self.base.step(dt);
    }
}

impl FiniteTimeAction for EaseInOut {
    fn get_duration(&self) -> f32 {
        self.base.get_duration()
    }

    fn set_duration(&mut self, duration: f32) {
        self.base.set_duration(duration);
    }

    fn reverse(&self) -> Box<dyn FiniteTimeAction> {
        self.base.reverse()
    }

    fn get_elapsed(&self) -> f32 {
        FiniteTimeAction::get_elapsed(&self.base)
    }
}

impl ActionInterval for EaseInOut {
    fn get_elapsed(&self) -> f32 {
        ActionInterval::get_elapsed(&self.base)
    }

    fn is_first_tick(&self) -> bool {
        ActionInterval::is_first_tick(&self.base)
    }
}

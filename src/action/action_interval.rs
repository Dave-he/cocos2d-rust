/// ActionInterval - 间隔动作实现
///
/// 持续一定时间的动作，如移动、旋转、缩放等。

use std::rc::Rc;
use std::cell::RefCell;
use std::f32::consts::PI;
use std::mem;

use super::action::{Action, FiniteTimeAction, INVALID_TAG};
use crate::scene::Node;
use crate::math::Vec2;
use crate::math::Vec3;
use crate::math::geometry::Size;

/// ActionInterval - 间隔动作基类
#[derive(Debug, Clone)]
pub struct ActionIntervalImpl {
    target: Option<Rc<RefCell<Node>>>,
    original_target: Option<Rc<RefCell<Node>>>,
    tag: i32,
    duration: f32,
    elapsed: f32,
    first_tick: bool,
}

impl ActionIntervalImpl {
    pub fn new(duration: f32) -> Self {
        Self {
            target: None,
            original_target: None,
            tag: INVALID_TAG,
            duration,
            elapsed: 0.0,
            first_tick: true,
        }
    }

    pub fn with_action(action: Box<dyn Action>) -> Self {
        if let Some(fta) = action.as_any().downcast_ref::<ActionIntervalImpl>() {
            Self::new(fta.duration)
        } else {
            Self::new(0.0)
        }
    }

    pub fn set_duration(&mut self, duration: f32) {
        self.duration = duration;
    }

    pub fn get_duration(&self) -> f32 {
        self.duration
    }

    pub fn get_elapsed(&self) -> f32 {
        self.elapsed
    }

    pub fn is_first_tick(&self) -> bool {
        self.first_tick
    }
}

impl Action for ActionIntervalImpl {
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
        self.original_target = Some(Rc::clone(target));
        self.target = Some(Rc::clone(target));
        self.elapsed = 0.0;
        self.first_tick = true;
    }

    fn stop(&mut self) {
        self.target = None;
    }

    fn update(&mut self, dt: f32) {
        if self.first_tick {
            self.first_tick = false;
            self.elapsed = 0.0;
        } else {
            self.elapsed += dt;
        }
        
        let time = self.elapsed.min(self.duration);
        self.update_with_time(time / self.duration);
    }

    fn is_done(&self) -> bool {
        self.elapsed >= self.duration
    }

    fn step(&mut self, dt: f32) {
        if self.first_tick {
            self.first_tick = false;
            self.elapsed = 0.0;
        } else {
            self.elapsed += dt;
        }
        
        let time = self.elapsed.min(self.duration);
        self.update_with_time(time / self.duration);
    }
}

impl ActionIntervalImpl {
    fn update_with_time(&mut self, _time: f32) {
        // 子类覆盖
    }

    pub fn reverse(&self) -> Box<dyn FiniteTimeAction> {
        Box::new(self.clone())
    }
}

/// MoveBy - 相对移动动作
#[derive(Debug, Clone)]
pub struct MoveBy {
    interval: ActionIntervalImpl,
    delta: Vec2,
    start_position: Vec2,
    previous_position: Vec2,
}

impl MoveBy {
    pub fn new(duration: f32, delta: Vec2) -> Self {
        Self {
            interval: ActionIntervalImpl::new(duration),
            delta,
            start_position: Vec2::zero(),
            previous_position: Vec2::zero(),
        }
    }

    pub fn with_duration_delta(duration: f32, delta_x: f32, delta_y: f32) -> Self {
        Self::new(duration, Vec2::new(delta_x, delta_y))
    }

    pub fn delta(&self) -> Vec2 {
        self.delta
    }
}

impl Action for MoveBy {
    fn clone_action(&self) -> Box<dyn Action> {
        Box::new(self.clone())
    }

    fn get_original_target(&self) -> Option<Rc<RefCell<Node>>> {
        self.interval.get_original_target()
    }

    fn get_target(&self) -> Option<Rc<RefCell<Node>>> {
        self.interval.get_target()
    }

    fn set_target(&mut self, target: Option<Rc<RefCell<Node>>>) {
        self.interval.set_target(target);
    }

    fn get_original_target_ref(&self) -> Option<&Rc<RefCell<Node>>> {
        self.interval.get_original_target_ref()
    }

    fn get_tag(&self) -> i32 {
        self.interval.get_tag()
    }

    fn set_tag(&mut self, tag: i32) {
        self.interval.set_tag(tag);
    }

    fn start_with_target(&mut self, target: &Rc<RefCell<Node>>) {
        self.interval.start_with_target(target);
        self.start_position = target.borrow().position();
        self.previous_position = self.start_position;
    }

    fn stop(&mut self) {
        self.interval.stop();
    }

    fn update(&mut self, dt: f32) {
        self.interval.update(dt);
    }

    fn is_done(&self) -> bool {
        self.interval.is_done()
    }

    fn step(&mut self, dt: f32) {
        self.interval.step(dt);
    }
}

impl MoveBy {
    pub fn update_with_time(&mut self, time: f32) {
        let current_pos = self.start_position + self.delta * time;
        
        if let Some(ref target) = self.interval.get_target() {
            let diff = current_pos - self.previous_position;
            target.borrow_mut().set_position(target.borrow().position() + diff);
            self.previous_position = current_pos;
        }
    }
}

/// MoveTo - 绝对移动动作
#[derive(Debug, Clone)]
pub struct MoveTo {
    move_by: MoveBy,
    end_position: Vec2,
}

impl MoveTo {
    pub fn new(duration: f32, position: Vec2) -> Self {
        Self {
            move_by: MoveBy::new(duration, Vec2::zero()),
            end_position: position,
        }
    }

    pub fn with_position(duration: f32, x: f32, y: f32) -> Self {
        Self::new(duration, Vec2::new(x, y))
    }

    pub fn end_position(&self) -> Vec2 {
        self.end_position
    }
}

impl Action for MoveTo {
    fn clone_action(&self) -> Box<dyn Action> {
        Box::new(self.clone())
    }

    fn get_original_target(&self) -> Option<Rc<RefCell<Node>>> {
        self.move_by.get_original_target()
    }

    fn get_target(&self) -> Option<Rc<RefCell<Node>>> {
        self.move_by.get_target()
    }

    fn set_target(&mut self, target: Option<Rc<RefCell<Node>>>) {
        self.move_by.set_target(target);
    }

    fn get_original_target_ref(&self) -> Option<&Rc<RefCell<Node>>> {
        self.move_by.get_original_target_ref()
    }

    fn get_tag(&self) -> i32 {
        self.move_by.get_tag()
    }

    fn set_tag(&mut self, tag: i32) {
        self.move_by.set_tag(tag);
    }

    fn start_with_target(&mut self, target: &Rc<RefCell<Node>>) {
        let start = target.borrow().position();
        self.move_by.delta = self.end_position - start;
        self.move_by.start_with_target(target);
    }

    fn stop(&mut self) {
        self.move_by.stop();
    }

    fn update(&mut self, dt: f32) {
        self.move_by.update(dt);
    }

    fn is_done(&self) -> bool {
        self.move_by.is_done()
    }

    fn step(&mut self, dt: f32) {
        self.move_by.step(dt);
    }
}

impl MoveTo {
    pub fn update_with_time(&mut self, time: f32) {
        self.move_by.update_with_time(time);
    }
}

/// RotateBy - 相对旋转动作
#[derive(Debug, Clone)]
pub struct RotateBy {
    interval: ActionIntervalImpl,
    delta: f32,
    start_angle: f32,
}

impl RotateBy {
    pub fn new(duration: f32, delta: f32) -> Self {
        Self {
            interval: ActionIntervalImpl::new(duration),
            delta,
            start_angle: 0.0,
        }
    }

    pub fn delta(&self) -> f32 {
        self.delta
    }
}

impl Action for RotateBy {
    fn clone_action(&self) -> Box<dyn Action> {
        Box::new(self.clone())
    }

    fn get_original_target(&self) -> Option<Rc<RefCell<Node>>> {
        self.interval.get_original_target()
    }

    fn get_target(&self) -> Option<Rc<RefCell<Node>>> {
        self.interval.get_target()
    }

    fn set_target(&mut self, target: Option<Rc<RefCell<Node>>>) {
        self.interval.set_target(target);
    }

    fn get_original_target_ref(&self) -> Option<&Rc<RefCell<Node>>> {
        self.interval.get_original_target_ref()
    }

    fn get_tag(&self) -> i32 {
        self.interval.get_tag()
    }

    fn set_tag(&mut self, tag: i32) {
        self.interval.set_tag(tag);
    }

    fn start_with_target(&mut self, target: &Rc<RefCell<Node>>) {
        self.interval.start_with_target(target);
        self.start_angle = target.borrow().rotation();
    }

    fn stop(&mut self) {
        self.interval.stop();
    }

    fn update(&mut self, dt: f32) {
        self.interval.update(dt);
    }

    fn is_done(&self) -> bool {
        self.interval.is_done()
    }

    fn step(&mut self, dt: f32) {
        self.interval.step(dt);
    }
}

impl RotateBy {
    pub fn update_with_time(&mut self, time: f32) {
        let new_angle = self.start_angle + self.delta * time;
        if let Some(ref target) = self.interval.get_target() {
            target.borrow_mut().set_rotation(new_angle);
        }
    }
}

/// RotateTo - 绝对旋转动作
#[derive(Debug, Clone)]
pub struct RotateTo {
    rotate_by: RotateBy,
    end_angle: f32,
}

impl RotateTo {
    pub fn new(duration: f32, angle: f32) -> Self {
        Self {
            rotate_by: RotateBy::new(duration, 0.0),
            end_angle: angle,
        }
    }

    pub fn end_angle(&self) -> f32 {
        self.end_angle
    }
}

impl Action for RotateTo {
    fn clone_action(&self) -> Box<dyn Action> {
        Box::new(self.clone())
    }

    fn get_original_target(&self) -> Option<Rc<RefCell<Node>>> {
        self.rotate_by.get_original_target()
    }

    fn get_target(&self) -> Option<Rc<RefCell<Node>>> {
        self.rotate_by.get_target()
    }

    fn set_target(&mut self, target: Option<Rc<RefCell<Node>>>) {
        self.rotate_by.set_target(target);
    }

    fn get_original_target_ref(&self) -> Option<&Rc<RefCell<Node>>> {
        self.rotate_by.get_original_target_ref()
    }

    fn get_tag(&self) -> i32 {
        self.rotate_by.get_tag()
    }

    fn set_tag(&mut self, tag: i32) {
        self.rotate_by.set_tag(tag);
    }

    fn start_with_target(&mut self, target: &Rc<RefCell<Node>>) {
        let start = target.borrow().rotation();
        self.rotate_by.delta = self.end_angle - start;
        self.rotate_by.start_with_target(target);
    }

    fn stop(&mut self) {
        self.rotate_by.stop();
    }

    fn update(&mut self, dt: f32) {
        self.rotate_by.update(dt);
    }

    fn is_done(&self) -> bool {
        self.rotate_by.is_done()
    }

    fn step(&mut self, dt: f32) {
        self.rotate_by.step(dt);
    }
}

impl RotateTo {
    pub fn update_with_time(&mut self, time: f32) {
        self.rotate_by.update_with_time(time);
    }
}

/// ScaleBy - 相对缩放动作
#[derive(Debug, Clone)]
pub struct ScaleBy {
    interval: ActionIntervalImpl,
    delta: Vec2,
    start_scale: Vec2,
}

impl ScaleBy {
    pub fn new(duration: f32, scale: f32) -> Self {
        Self {
            interval: ActionIntervalImpl::new(duration),
            delta: Vec2::new(scale, scale),
            start_scale: Vec2::new(1.0, 1.0),
        }
    }

    pub fn new_xy(duration: f32, scale_x: f32, scale_y: f32) -> Self {
        Self {
            interval: ActionIntervalImpl::new(duration),
            delta: Vec2::new(scale_x, scale_y),
            start_scale: Vec2::new(1.0, 1.0),
        }
    }

    pub fn delta(&self) -> Vec2 {
        self.delta
    }
}

impl Action for ScaleBy {
    fn clone_action(&self) -> Box<dyn Action> {
        Box::new(self.clone())
    }

    fn get_original_target(&self) -> Option<Rc<RefCell<Node>>> {
        self.interval.get_original_target()
    }

    fn get_target(&self) -> Option<Rc<RefCell<Node>>> {
        self.interval.get_target()
    }

    fn set_target(&mut self, target: Option<Rc<RefCell<Node>>>) {
        self.interval.set_target(target);
    }

    fn get_original_target_ref(&self) -> Option<&Rc<RefCell<Node>>> {
        self.interval.get_original_target_ref()
    }

    fn get_tag(&self) -> i32 {
        self.interval.get_tag()
    }

    fn set_tag(&mut self, tag: i32) {
        self.interval.set_tag(tag);
    }

    fn start_with_target(&mut self, target: &Rc<RefCell<Node>>) {
        self.interval.start_with_target(target);
        self.start_scale.x = target.borrow().scale_x();
        self.start_scale.y = target.borrow().scale_y();
    }

    fn stop(&mut self) {
        self.interval.stop();
    }

    fn update(&mut self, dt: f32) {
        self.interval.update(dt);
    }

    fn is_done(&self) -> bool {
        self.interval.is_done()
    }

    fn step(&mut self, dt: f32) {
        self.interval.step(dt);
    }
}

impl ScaleBy {
    pub fn update_with_time(&mut self, time: f32) {
        let new_scale = self.start_scale + self.delta * time;
        if let Some(ref target) = self.interval.get_target() {
            let mut target_mut = target.borrow_mut();
            target_mut.set_scale_xy(new_scale.x, new_scale.y);
        }
    }
}

/// ScaleTo - 绝对缩放动作
#[derive(Debug, Clone)]
pub struct ScaleTo {
    scale_by: ScaleBy,
    end_scale: Vec2,
}

impl ScaleTo {
    pub fn new(duration: f32, scale: f32) -> Self {
        Self {
            scale_by: ScaleBy::new(duration, 0.0),
            end_scale: Vec2::new(scale, scale),
        }
    }

    pub fn new_xy(duration: f32, scale_x: f32, scale_y: f32) -> Self {
        Self {
            scale_by: ScaleBy::new_xy(duration, 0.0, 0.0),
            end_scale: Vec2::new(scale_x, scale_y),
        }
    }

    pub fn end_scale(&self) -> Vec2 {
        self.end_scale
    }
}

impl Action for ScaleTo {
    fn clone_action(&self) -> Box<dyn Action> {
        Box::new(self.clone())
    }

    fn get_original_target(&self) -> Option<Rc<RefCell<Node>>> {
        self.scale_by.get_original_target()
    }

    fn get_target(&self) -> Option<Rc<RefCell<Node>>> {
        self.scale_by.get_target()
    }

    fn set_target(&mut self, target: Option<Rc<RefCell<Node>>>) {
        self.scale_by.set_target(target);
    }

    fn get_original_target_ref(&self) -> Option<&Rc<RefCell<Node>>> {
        self.scale_by.get_original_target_ref()
    }

    fn get_tag(&self) -> i32 {
        self.scale_by.get_tag()
    }

    fn set_tag(&mut self, tag: i32) {
        self.scale_by.set_tag(tag);
    }

    fn start_with_target(&mut self, target: &Rc<RefCell<Node>>) {
        let mut start = Vec2::new(1.0, 1.0);
        start.x = target.borrow().scale_x();
        start.y = target.borrow().scale_y();
        self.scale_by.delta = self.end_scale - start;
        self.scale_by.start_with_target(target);
    }

    fn stop(&mut self) {
        self.scale_by.stop();
    }

    fn update(&mut self, dt: f32) {
        self.scale_by.update(dt);
    }

    fn is_done(&self) -> bool {
        self.scale_by.is_done()
    }

    fn step(&mut self, dt: f32) {
        self.scale_by.step(dt);
    }
}

impl ScaleTo {
    pub fn update_with_time(&mut self, time: f32) {
        self.scale_by.update_with_time(time);
    }
}

/// SkewBy - 相对倾斜动作
#[derive(Debug, Clone)]
pub struct SkewBy {
    interval: ActionIntervalImpl,
    delta: Vec2,
    start_skew: Vec2,
}

impl SkewBy {
    pub fn new(duration: f32, skew_x: f32, skew_y: f32) -> Self {
        Self {
            interval: ActionIntervalImpl::new(duration),
            delta: Vec2::new(skew_x, skew_y),
            start_skew: Vec2::zero(),
        }
    }

    pub fn delta(&self) -> Vec2 {
        self.delta
    }
}

impl Action for SkewBy {
    fn clone_action(&self) -> Box<dyn Action> {
        Box::new(self.clone())
    }

    fn get_original_target(&self) -> Option<Rc<RefCell<Node>>> {
        self.interval.get_original_target()
    }

    fn get_target(&self) -> Option<Rc<RefCell<Node>>> {
        self.interval.get_target()
    }

    fn set_target(&mut self, target: Option<Rc<RefCell<Node>>>) {
        self.interval.set_target(target);
    }

    fn get_original_target_ref(&self) -> Option<&Rc<RefCell<Node>>> {
        self.interval.get_original_target_ref()
    }

    fn get_tag(&self) -> i32 {
        self.interval.get_tag()
    }

    fn set_tag(&mut self, tag: i32) {
        self.interval.set_tag(tag);
    }

    fn start_with_target(&mut self, target: &Rc<RefCell<Node>>) {
        self.interval.start_with_target(target);
        self.start_skew.x = target.borrow().skew_x();
        self.start_skew.y = target.borrow().skew_y();
    }

    fn stop(&mut self) {
        self.interval.stop();
    }

    fn update(&mut self, dt: f32) {
        self.interval.update(dt);
    }

    fn is_done(&self) -> bool {
        self.interval.is_done()
    }

    fn step(&mut self, dt: f32) {
        self.interval.step(dt);
    }
}

impl SkewBy {
    pub fn update_with_time(&mut self, time: f32) {
        let new_skew = self.start_skew + self.delta * time;
        if let Some(ref target) = self.interval.get_target() {
            let mut target_mut = target.borrow_mut();
            // 需要在 Node 中添加 set_skew_xy 方法
            // target_mut.set_skew_xy(new_skew.x, new_skew.y);
        }
    }
}

/// Blink - 闪烁动作
#[derive(Debug, Clone)]
pub struct Blink {
    interval: ActionIntervalImpl,
    times: u32,
    original_visible: bool,
}

impl Blink {
    pub fn new(duration: f32, times: u32) -> Self {
        Self {
            interval: ActionIntervalImpl::new(duration),
            times,
            original_visible: true,
        }
    }

    pub fn times(&self) -> u32 {
        self.times
    }
}

impl Action for Blink {
    fn clone_action(&self) -> Box<dyn Action> {
        Box::new(self.clone())
    }

    fn get_original_target(&self) -> Option<Rc<RefCell<Node>>> {
        self.interval.get_original_target()
    }

    fn get_target(&self) -> Option<Rc<RefCell<Node>>> {
        self.interval.get_target()
    }

    fn set_target(&mut self, target: Option<Rc<RefCell<Node>>>) {
        self.interval.set_target(target);
    }

    fn get_original_target_ref(&self) -> Option<&Rc<RefCell<Node>>> {
        self.interval.get_original_target_ref()
    }

    fn get_tag(&self) -> i32 {
        self.interval.get_tag()
    }

    fn set_tag(&mut self, tag: i32) {
        self.interval.set_tag(tag);
    }

    fn start_with_target(&mut self, target: &Rc<RefCell<Node>>) {
        self.interval.start_with_target(target);
        self.original_visible = target.borrow().is_visible();
    }

    fn stop(&mut self) {
        if let Some(ref target) = self.interval.get_target() {
            target.borrow_mut().set_visible(self.original_visible);
        }
        self.interval.stop();
    }

    fn update(&mut self, dt: f32) {
        self.interval.update(dt);
    }

    fn is_done(&self) -> bool {
        self.interval.is_done()
    }

    fn step(&mut self, dt: f32) {
        self.interval.step(dt);
    }
}

impl Blink {
    pub fn update_with_time(&mut self, time: f32) {
        let slice = 1.0 / self.times as f32;
        let m = (time / slice).floor() as u32;
        if let Some(ref target) = self.interval.get_target() {
            target.borrow_mut().set_visible(m % 2 == 0);
        }
    }
}

/// DelayTime - 延迟动作
#[derive(Debug, Clone)]
pub struct DelayTime {
    interval: ActionIntervalImpl,
}

impl DelayTime {
    pub fn new(duration: f32) -> Self {
        Self {
            interval: ActionIntervalImpl::new(duration),
        }
    }
}

impl Action for DelayTime {
    fn clone_action(&self) -> Box<dyn Action> {
        Box::new(self.clone())
    }

    fn get_original_target(&self) -> Option<Rc<RefCell<Node>>> {
        self.interval.get_original_target()
    }

    fn get_target(&self) -> Option<Rc<RefCell<Node>>> {
        self.interval.get_target()
    }

    fn set_target(&mut self, target: Option<Rc<RefCell<Node>>>) {
        self.interval.set_target(target);
    }

    fn get_original_target_ref(&self) -> Option<&Rc<RefCell<Node>>> {
        self.interval.get_original_target_ref()
    }

    fn get_tag(&self) -> i32 {
        self.interval.get_tag()
    }

    fn set_tag(&mut self, tag: i32) {
        self.interval.set_tag(tag);
    }

    fn start_with_target(&mut self, target: &Rc<RefCell<Node>>) {
        self.interval.start_with_target(target);
    }

    fn stop(&mut self) {
        self.interval.stop();
    }

    fn update(&mut self, dt: f32) {
        self.interval.update(dt);
    }

    fn is_done(&self) -> bool {
        self.interval.is_done()
    }

    fn step(&mut self, dt: f32) {
        self.interval.step(dt);
    }
}

/// FadeTo - 渐变到指定透明度
#[derive(Debug, Clone)]
pub struct FadeTo {
    interval: ActionIntervalImpl,
    to_opacity: u8,
    from_opacity: u8,
}

impl FadeTo {
    pub fn new(duration: f32, opacity: u8) -> Self {
        Self {
            interval: ActionIntervalImpl::new(duration),
            to_opacity: opacity,
            from_opacity: 255,
        }
    }

    pub fn to_opacity(&self) -> u8 {
        self.to_opacity
    }
}

impl Action for FadeTo {
    fn clone_action(&self) -> Box<dyn Action> {
        Box::new(self.clone())
    }

    fn get_original_target(&self) -> Option<Rc<RefCell<Node>>> {
        self.interval.get_original_target()
    }

    fn get_target(&self) -> Option<Rc<RefCell<Node>>> {
        self.interval.get_target()
    }

    fn set_target(&mut self, target: Option<Rc<RefCell<Node>>>) {
        self.interval.set_target(target);
    }

    fn get_original_target_ref(&self) -> Option<&Rc<RefCell<Node>>> {
        self.interval.get_original_target_ref()
    }

    fn get_tag(&self) -> i32 {
        self.interval.get_tag()
    }

    fn set_tag(&mut self, tag: i32) {
        self.interval.set_tag(tag);
    }

    fn start_with_target(&mut self, target: &Rc<RefCell<Node>>) {
        self.interval.start_with_target(target);
        self.from_opacity = target.borrow().opacity();
    }

    fn stop(&mut self) {
        self.interval.stop();
    }

    fn update(&mut self, dt: f32) {
        self.interval.update(dt);
    }

    fn is_done(&self) -> bool {
        self.interval.is_done()
    }

    fn step(&mut self, dt: f32) {
        self.interval.step(dt);
    }
}

impl FadeTo {
    pub fn update_with_time(&mut self, time: f32) {
        let opacity = self.from_opacity as f32 + 
            (self.to_opacity as f32 - self.from_opacity as f32) * time;
        if let Some(ref target) = self.interval.get_target() {
            target.borrow_mut().set_opacity(opacity as u8);
        }
    }
}

/// FadeIn - 渐入动作
#[derive(Debug, Clone)]
pub struct FadeIn {
    fade_to: FadeTo,
}

impl FadeIn {
    pub fn new(duration: f32) -> Self {
        Self {
            fade_to: FadeTo::new(duration, 255),
        }
    }
}

impl Action for FadeIn {
    fn clone_action(&self) -> Box<dyn Action> {
        Box::new(self.clone())
    }

    fn get_original_target(&self) -> Option<Rc<RefCell<Node>>> {
        self.fade_to.get_original_target()
    }

    fn get_target(&self) -> Option<Rc<RefCell<Node>>> {
        self.fade_to.get_target()
    }

    fn set_target(&mut self, target: Option<Rc<RefCell<Node>>>) {
        self.fade_to.set_target(target);
    }

    fn get_original_target_ref(&self) -> Option<&Rc<RefCell<Node>>> {
        self.fade_to.get_original_target_ref()
    }

    fn get_tag(&self) -> i32 {
        self.fade_to.get_tag()
    }

    fn set_tag(&mut self, tag: i32) {
        self.fade_to.set_tag(tag);
    }

    fn start_with_target(&mut self, target: &Rc<RefCell<Node>>) {
        self.fade_to.start_with_target(target);
    }

    fn stop(&mut self) {
        self.fade_to.stop();
    }

    fn update(&mut self, dt: f32) {
        self.fade_to.update(dt);
    }

    fn is_done(&self) -> bool {
        self.fade_to.is_done()
    }

    fn step(&mut self, dt: f32) {
        self.fade_to.step(dt);
    }
}

/// FadeOut - 渐出动作
#[derive(Debug, Clone)]
pub struct FadeOut {
    fade_to: FadeTo,
}

impl FadeOut {
    pub fn new(duration: f32) -> Self {
        Self {
            fade_to: FadeTo::new(duration, 0),
        }
    }
}

impl Action for FadeOut {
    fn clone_action(&self) -> Box<dyn Action> {
        Box::new(self.clone())
    }

    fn get_original_target(&self) -> Option<Rc<RefCell<Node>>> {
        self.fade_to.get_original_target()
    }

    fn get_target(&self) -> Option<Rc<RefCell<Node>>> {
        self.fade_to.get_target()
    }

    fn set_target(&mut self, target: Option<Rc<RefCell<Node>>>) {
        self.fade_to.set_target(target);
    }

    fn get_original_target_ref(&self) -> Option<&Rc<RefCell<Node>>> {
        self.fade_to.get_original_target_ref()
    }

    fn get_tag(&self) -> i32 {
        self.fade_to.get_tag()
    }

    fn set_tag(&mut self, tag: i32) {
        self.fade_to.set_tag(tag);
    }

    fn start_with_target(&mut self, target: &Rc<RefCell<Node>>) {
        self.fade_to.start_with_target(target);
    }

    fn stop(&mut self) {
        self.fade_to.stop();
    }

    fn update(&mut self, dt: f32) {
        self.fade_to.update(dt);
    }

    fn is_done(&self) -> bool {
        self.fade_to.is_done()
    }

    fn step(&mut self, dt: f32) {
        self.fade_to.step(dt);
    }
}

/// BezierBy - 贝塞尔曲线移动
#[derive(Debug, Clone)]
pub struct BezierBy {
    interval: ActionIntervalImpl,
    config: BezierConfig,
    start_position: Vec2,
}

#[derive(Debug, Clone, Copy)]
pub struct BezierConfig {
    pub end_position: Vec2,
    pub control_point_1: Vec2,
    pub control_point_2: Vec2,
}

impl BezierBy {
    pub fn new(duration: f32, config: BezierConfig) -> Self {
        Self {
            interval: ActionIntervalImpl::new(duration),
            config,
            start_position: Vec2::zero(),
        }
    }
}

impl Action for BezierBy {
    fn clone_action(&self) -> Box<dyn Action> {
        Box::new(self.clone())
    }

    fn get_original_target(&self) -> Option<Rc<RefCell<Node>>> {
        self.interval.get_original_target()
    }

    fn get_target(&self) -> Option<Rc<RefCell<Node>>> {
        self.interval.get_target()
    }

    fn set_target(&mut self, target: Option<Rc<RefCell<Node>>>) {
        self.interval.set_target(target);
    }

    fn get_original_target_ref(&self) -> Option<&Rc<RefCell<Node>>> {
        self.interval.get_original_target_ref()
    }

    fn get_tag(&self) -> i32 {
        self.interval.get_tag()
    }

    fn set_tag(&mut self, tag: i32) {
        self.interval.set_tag(tag);
    }

    fn start_with_target(&mut self, target: &Rc<RefCell<Node>>) {
        self.interval.start_with_target(target);
        self.start_position = target.borrow().position();
    }

    fn stop(&mut self) {
        self.interval.stop();
    }

    fn update(&mut self, dt: f32) {
        self.interval.update(dt);
    }

    fn is_done(&self) -> bool {
        self.interval.is_done()
    }

    fn step(&mut self, dt: f32) {
        self.interval.step(dt);
    }
}

impl BezierBy {
    pub fn update_with_time(&mut self, time: f32) {
        let xa = 0.0;
        let xb = self.config.control_point_1.x;
        let xc = self.config.control_point_2.x;
        let xd = self.config.end_position.x;

        let ya = 0.0;
        let yb = self.config.control_point_1.y;
        let yc = self.config.control_point_2.y;
        let yd = self.config.end_position.y;

        let x = Self::bezier_at(xa, xb, xc, xd, time) + self.start_position.x;
        let y = Self::bezier_at(ya, yb, yc, yd, time) + self.start_position.y;

        if let Some(ref target) = self.interval.get_target() {
            target.borrow_mut().set_position(Vec2::new(x, y));
        }
    }

    fn bezier_at(a: f32, b: f32, c: f32, d: f32, t: f32) -> f32 {
        ((1.0 - t).powi(3) * a +
         3.0 * t * (1.0 - t).powi(2) * b +
         3.0 * t.powi(2) * (1.0 - t) * c +
         t.powi(3) * d)
    }
}

/// JumpBy - 跳跃动作
#[derive(Debug, Clone)]
pub struct JumpBy {
    interval: ActionIntervalImpl,
    delta: Vec2,
    height: f32,
    jumps: u32,
    previous_position: Vec2,
}

impl JumpBy {
    pub fn new(duration: f32, delta: Vec2, height: f32, jumps: u32) -> Self {
        Self {
            interval: ActionIntervalImpl::new(duration),
            delta,
            height,
            jumps,
            previous_position: Vec2::zero(),
        }
    }

    pub fn new_with_position(duration: f32, delta_x: f32, delta_y: f32, height: f32, jumps: u32) -> Self {
        Self::new(duration, Vec2::new(delta_x, delta_y), height, jumps)
    }
}

impl Action for JumpBy {
    fn clone_action(&self) -> Box<dyn Action> {
        Box::new(self.clone())
    }

    fn get_original_target(&self) -> Option<Rc<RefCell<Node>>> {
        self.interval.get_original_target()
    }

    fn get_target(&self) -> Option<Rc<RefCell<Node>>> {
        self.interval.get_target()
    }

    fn set_target(&mut self, target: Option<Rc<RefCell<Node>>>) {
        self.interval.set_target(target);
    }

    fn get_original_target_ref(&self) -> Option<&Rc<RefCell<Node>>> {
        self.interval.get_original_target_ref()
    }

    fn get_tag(&self) -> i32 {
        self.interval.get_tag()
    }

    fn set_tag(&mut self, tag: i32) {
        self.interval.set_tag(tag);
    }

    fn start_with_target(&mut self, target: &Rc<RefCell<Node>>) {
        self.interval.start_with_target(target);
        self.previous_position = target.borrow().position();
    }

    fn stop(&mut self) {
        self.interval.stop();
    }

    fn update(&mut self, dt: f32) {
        self.interval.update(dt);
    }

    fn is_done(&self) -> bool {
        self.interval.is_done()
    }

    fn step(&mut self, dt: f32) {
        self.interval.step(dt);
    }
}

impl JumpBy {
    pub fn update_with_time(&mut self, time: f32) {
        let frac = (self.interval.get_elapsed() * self.jumps as f32 / self.interval.get_duration()).fract();
        let y = self.height * 4.0 * frac * (1.0 - frac);
        let x = self.delta.x * time;
        let z = self.delta.y * time;

        if let Some(ref target) = self.interval.get_target() {
            let current_pos = target.borrow().position();
            let diff = Vec2::new(x, z + y) - (current_pos - self.previous_position);
            target.borrow_mut().set_position(current_pos + diff);
            self.previous_position = Vec2::new(x, z + y);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn test_move_by() {
        let target = Rc::new(RefCell::new(Node::new()));
        target.borrow_mut().set_position(Vec2::zero());

        let mut move_by = MoveBy::new(1.0, Vec2::new(100.0, 50.0));
        move_by.start_with_target(&target);

        assert_eq!(target.borrow().position(), Vec2::zero());

        move_by.update(0.5);
        let pos = target.borrow().position();
        assert!((pos.x - 50.0).abs() < 0.01);
        assert!((pos.y - 25.0).abs() < 0.01);

        move_by.update(0.5);
        assert!((target.borrow().position().x - 100.0).abs() < 0.01);
        assert!((target.borrow().position().y - 50.0).abs() < 0.01);
        assert!(move_by.is_done());
    }

    #[test]
    fn test_move_to() {
        let target = Rc::new(RefCell::new(Node::new()));
        target.borrow_mut().set_position(Vec2::zero());

        let mut move_to = MoveTo::new(1.0, Vec2::new(200.0, 100.0));
        move_to.start_with_target(&target);

        move_to.update(0.5);
        let pos = target.borrow().position();
        assert!((pos.x - 100.0).abs() < 0.01);
        assert!((pos.y - 50.0).abs() < 0.01);

        move_to.update(0.5);
        assert!((target.borrow().position().x - 200.0).abs() < 0.01);
        assert!((target.borrow().position().y - 100.0).abs() < 0.01);
    }

    #[test]
    fn test_rotate_by() {
        let target = Rc::new(RefCell::new(Node::new()));
        target.borrow_mut().set_rotation(0.0);

        let mut rotate_by = RotateBy::new(1.0, 90.0);
        rotate_by.start_with_target(&target);

        rotate_by.update(0.5);
        assert!((target.borrow().rotation() - 45.0).abs() < 0.01);

        rotate_by.update(0.5);
        assert!((target.borrow().rotation() - 90.0).abs() < 0.01);
    }

    #[test]
    fn test_rotate_to() {
        let target = Rc::new(RefCell::new(Node::new()));
        target.borrow_mut().set_rotation(0.0);

        let mut rotate_to = RotateTo::new(1.0, 180.0);
        rotate_to.start_with_target(&target);

        rotate_by.update(0.5);
        assert!((target.borrow().rotation() - 90.0).abs() < 0.01);

        rotate_by.update(0.5);
        assert!((target.borrow().rotation() - 180.0).abs() < 0.01);
    }

    #[test]
    fn test_scale_by() {
        let target = Rc::new(RefCell::new(Node::new()));
        target.borrow_mut().set_scale(1.0);

        let mut scale_by = ScaleBy::new(1.0, 2.0);
        scale_by.start_with_target(&target);

        scale_by.update(0.5);
        let scale = target.borrow().scale();
        assert!((scale - 1.5).abs() < 0.01);

        scale_by.update(0.5);
        assert!((target.borrow().scale() - 2.0).abs() < 0.01);
    }

    #[test]
    fn test_scale_to() {
        let target = Rc::new(RefCell::new(Node::new()));
        target.borrow_mut().set_scale(1.0);

        let mut scale_to = ScaleTo::new(1.0, 3.0);
        scale_to.start_with_target(&target);

        scale_to.update(0.5);
        let scale = target.borrow().scale();
        assert!((scale - 2.0).abs() < 0.01);

        scale_to.update(0.5);
        assert!((target.borrow().scale() - 3.0).abs() < 0.01);
    }

    #[test]
    fn test_delay_time() {
        let target = Rc::new(RefCell::new(Node::new()));
        let pos = Vec2::new(100.0, 200.0);
        target.borrow_mut().set_position(pos);

        let mut delay = DelayTime::new(0.5);
        delay.start_with_target(&target);

        delay.update(0.25);
        assert!(!delay.is_done());
        assert_eq!(target.borrow().position(), pos);

        delay.update(0.3);
        assert!(delay.is_done());
    }

    #[test]
    fn test_blink() {
        let target = Rc::new(RefCell::new(Node::new()));
        target.borrow_mut().set_visible(true);

        let mut blink = Blink::new(1.0, 4);
        blink.start_with_target(&target);

        blink.update(0.125);
        assert!(!target.borrow().is_visible());

        blink.update(0.125);
        assert!(target.borrow().is_visible());

        blink.stop();
        assert!(target.borrow().is_visible());
    }

    #[test]
    fn test_fade_to() {
        let target = Rc::new(RefCell::new(Node::new()));
        target.borrow_mut().set_opacity(255);

        let mut fade_to = FadeTo::new(1.0, 0);
        fade_to.start_with_target(&target);

        fade_to.update(0.5);
        assert_eq!(target.borrow().opacity(), 128);

        fade_to.update(0.5);
        assert_eq!(target.borrow().opacity(), 0);
    }

    #[test]
    fn test_fade_in() {
        let target = Rc::new(RefCell::new(Node::new()));
        target.borrow_mut().set_opacity(0);

        let mut fade_in = FadeIn::new(1.0);
        fade_in.start_with_target(&target);

        fade_in.update(0.5);
        assert_eq!(target.borrow().opacity(), 128);

        fade_in.update(0.5);
        assert_eq!(target.borrow().opacity(), 255);
    }

    #[test]
    fn test_fade_out() {
        let target = Rc::new(RefCell::new(Node::new()));
        target.borrow_mut().set_opacity(255);

        let mut fade_out = FadeOut::new(1.0);
        fade_out.start_with_target(&target);

        fade_out.update(0.5);
        assert_eq!(target.borrow().opacity(), 128);

        fade_out.update(0.5);
        assert_eq!(target.borrow().opacity(), 0);
    }
}

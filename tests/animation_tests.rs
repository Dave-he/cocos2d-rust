use cocos2d_rust::animation::{Animation, Animate, SpriteFrame};
use cocos2d_rust::math::{Vec2, Rect};
use std::rc::Rc;
use std::cell::RefCell;

#[test]
fn test_animation_creation() {
    let animation = Animation::new();
    
    assert_eq!(animation.duration(), 0.0);
}

#[test]
fn test_animation_add_frame() {
    let mut animation = Animation::new();
    let frame = Rc::new(RefCell::new(SpriteFrame::new("frame1")));
    
    animation.add_sprite_frame(frame);
    assert_eq!(animation.frame_count(), 1);
}

#[test]
fn test_animation_delay_per_unit() {
    let mut animation = Animation::new();
    
    animation.set_delay_per_unit(0.1);
    assert!((animation.delay_per_unit() - 0.1).abs() < f32::EPSILON);
}

#[test]
fn test_animation_loops() {
    let mut animation = Animation::new();
    
    animation.set_loops(5);
    assert_eq!(animation.loops(), 5);
}

#[test]
fn test_animate_action() {
    let animation = Animation::new();
    let animate = Animate::create(animation);
    
    assert!(!animate.is_done());
}

#[test]
fn test_sprite_frame_creation() {
    let frame = SpriteFrame::new("test_frame");
    
    assert!(frame.texture().is_none());
    assert_eq!(frame.name(), "test_frame");
}

#[test]
fn test_sprite_frame_rect() {
    let mut frame = SpriteFrame::new("test");
    
    frame.set_rect(Rect::new(0.0, 0.0, 100.0, 100.0));
    assert!((frame.rect().width() - 100.0).abs() < f32::EPSILON);
    assert!((frame.rect().height() - 100.0).abs() < f32::EPSILON);
}

#[test]
fn test_sprite_frame_offset() {
    let mut frame = SpriteFrame::new("test");
    
    frame.set_offset(10.0, 20.0);
    assert_eq!(frame.offset(), (10.0, 20.0));
}

#[test]
fn test_animation_duration() {
    let mut animation = Animation::new();
    animation.set_delay_per_unit(0.1);
    
    for i in 0..10 {
        let frame = Rc::new(RefCell::new(SpriteFrame::new(format!("frame{}", i))));
        animation.add_sprite_frame(frame);
    }
    
    assert!((animation.duration() - 1.0).abs() < 0.001, 
        "Expected duration 1.0, got {}", animation.duration());
}

#[test]
fn test_animation_restore_original_frame() {
    let mut animation = Animation::new();
    
    animation.set_restore_original_frame(true);
    assert!(animation.restore_original_frame());
}

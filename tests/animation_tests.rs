use cocos2d_rust::animation::{Animation, Animate, SpriteFrame};
use cocos2d_rust::action::ActionInterval;

#[test]
fn test_animation_creation() {
    let animation = Animation::new();
    
    assert_eq!(animation.get_total_delay_units(), 0.0);
}

#[test]
fn test_animation_add_frame() {
    let mut animation = Animation::new();
    let frame = SpriteFrame::new();
    
    animation.add_sprite_frame(frame);
    assert_eq!(animation.get_frames_count(), 1);
}

#[test]
fn test_animation_delay_per_unit() {
    let mut animation = Animation::new();
    
    animation.set_delay_per_unit(0.1);
    assert_eq!(animation.get_delay_per_unit(), 0.1);
}

#[test]
fn test_animation_loops() {
    let mut animation = Animation::new();
    
    animation.set_loops(5);
    assert_eq!(animation.get_loops(), 5);
}

#[test]
fn test_animate_action() {
    let animation = Animation::new();
    let animate = Animate::create(animation);
    
    assert!(!animate.is_done());
}

#[test]
fn test_sprite_frame_creation() {
    let frame = SpriteFrame::new();
    
    assert!(frame.get_texture().is_none());
}

#[test]
fn test_sprite_frame_rect() {
    let mut frame = SpriteFrame::new();
    
    frame.set_rect(Rect::new(0.0, 0.0, 100.0, 100.0));
    assert_eq!(frame.get_rect().size.width, 100.0);
}

#[test]
fn test_sprite_frame_offset() {
    let mut frame = SpriteFrame::new();
    
    frame.set_offset(Vec2::new(10.0, 20.0));
    assert_eq!(frame.get_offset(), Vec2::new(10.0, 20.0));
}

#[test]
fn test_animation_duration() {
    let mut animation = Animation::new();
    animation.set_delay_per_unit(0.1);
    
    for _ in 0..10 {
        animation.add_sprite_frame(SpriteFrame::new());
    }
    
    assert_eq!(animation.get_duration(), 1.0);
}

#[test]
fn test_animation_restore_original_frame() {
    let mut animation = Animation::new();
    
    animation.set_restore_original_frame(true);
    assert!(animation.get_restore_original_frame());
}

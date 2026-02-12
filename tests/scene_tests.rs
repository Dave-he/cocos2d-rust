use cocos2d_rust::scene::Node;
use cocos2d_rust::math::Vec2;
use cocos2d_rust::math::geometry::Size;

#[test]
fn test_node_creation() {
    let node = Node::new();
    assert!(node.is_visible());
    assert_eq!(node.get_position(), Vec2::ZERO);
    assert_eq!(node.get_scale_x(), 1.0);
    assert_eq!(node.get_scale_y(), 1.0);
}

#[test]
fn test_node_position() {
    let mut node = Node::new();
    
    node.set_position(Vec2::new(100.0, 200.0));
    assert_eq!(node.get_position(), Vec2::new(100.0, 200.0));
    
    node.set_position_x(50.0);
    assert_eq!(node.get_position_x(), 50.0);
    assert_eq!(node.get_position_y(), 200.0);
}

#[test]
fn test_node_scale() {
    let mut node = Node::new();
    
    node.set_scale(2.0);
    assert_eq!(node.get_scale_x(), 2.0);
    assert_eq!(node.get_scale_y(), 2.0);
    
    node.set_scale_x(3.0);
    node.set_scale_y(4.0);
    assert_eq!(node.get_scale_x(), 3.0);
    assert_eq!(node.get_scale_y(), 4.0);
}

#[test]
fn test_node_rotation() {
    let mut node = Node::new();
    
    node.set_rotation(45.0);
    assert_eq!(node.get_rotation(), 45.0);
    
    node.set_rotation(-90.0);
    assert_eq!(node.get_rotation(), -90.0);
}

#[test]
fn test_node_visibility() {
    let mut node = Node::new();
    
    assert!(node.is_visible());
    
    node.set_visible(false);
    assert!(!node.is_visible());
}

#[test]
fn test_node_opacity() {
    let mut node = Node::new();
    
    assert_eq!(node.get_opacity(), 255);
    
    node.set_opacity(128);
    assert_eq!(node.get_opacity(), 128);
}

#[test]
fn test_node_content_size() {
    let mut node = Node::new();
    
    let size = Size::new(100.0, 200.0);
    node.set_content_size(size);
    
    assert_eq!(node.get_content_size(), size);
}

#[test]
fn test_node_anchor_point() {
    let mut node = Node::new();
    
    node.set_anchor_point(Vec2::new(0.5, 0.5));
    assert_eq!(node.get_anchor_point(), Vec2::new(0.5, 0.5));
    
    node.set_anchor_point(Vec2::ANCHOR_BOTTOM_LEFT);
    assert_eq!(node.get_anchor_point(), Vec2::ANCHOR_BOTTOM_LEFT);
}

#[test]
fn test_node_z_order() {
    let mut node = Node::new();
    
    assert_eq!(node.get_local_z_order(), 0);
    
    node.set_local_z_order(10);
    assert_eq!(node.get_local_z_order(), 10);
}

#[test]
fn test_node_tag() {
    let mut node = Node::new();
    
    node.set_tag(100);
    assert_eq!(node.get_tag(), 100);
}

#[test]
fn test_node_name() {
    let mut node = Node::new();
    
    node.set_name("TestNode");
    assert_eq!(node.get_name(), "TestNode");
}

#[test]
fn test_node_user_data() {
    let mut node = Node::new();
    
    node.set_user_data(12345);
    assert_eq!(node.get_user_data(), 12345);
}

#[test]
fn test_node_parent_child_relationship() {
    let mut parent = Node::new();
    let child = Node::new();
    
    parent.add_child(child, 0, -1);
    
    assert_eq!(parent.get_children_count(), 1);
}

#[test]
fn test_node_remove_child() {
    let mut parent = Node::new();
    let child = Node::new();
    
    parent.add_child(child.clone(), 0, -1);
    assert_eq!(parent.get_children_count(), 1);
    
    parent.remove_child(&child, true);
    assert_eq!(parent.get_children_count(), 0);
}

#[test]
fn test_node_remove_all_children() {
    let mut parent = Node::new();
    
    for i in 0..5 {
        let child = Node::new();
        parent.add_child(child, i, -1);
    }
    
    assert_eq!(parent.get_children_count(), 5);
    
    parent.remove_all_children(true);
    assert_eq!(parent.get_children_count(), 0);
}

#[test]
fn test_node_get_child_by_tag() {
    let mut parent = Node::new();
    let mut child = Node::new();
    child.set_tag(999);
    
    parent.add_child(child, 0, -1);
    
    if let Some(found) = parent.get_child_by_tag(999) {
        assert_eq!(found.get_tag(), 999);
    } else {
        panic!("Child with tag 999 not found");
    }
}

#[test]
fn test_node_get_child_by_name() {
    let mut parent = Node::new();
    let mut child = Node::new();
    child.set_name("TestChild");
    
    parent.add_child(child, 0, -1);
    
    if let Some(found) = parent.get_child_by_name("TestChild") {
        assert_eq!(found.get_name(), "TestChild");
    } else {
        panic!("Child with name 'TestChild' not found");
    }
}

#[test]
fn test_node_world_position() {
    let mut parent = Node::new();
    parent.set_position(Vec2::new(100.0, 100.0));
    
    let mut child = Node::new();
    child.set_position(Vec2::new(50.0, 50.0));
    
    parent.add_child(child.clone(), 0, -1);
    
    let world_pos = child.get_world_position();
    assert_eq!(world_pos, Vec2::new(150.0, 150.0));
}

#[test]
fn test_node_running_state() {
    let mut node = Node::new();
    
    assert!(!node.is_running());
    
    node.on_enter();
    assert!(node.is_running());
    
    node.on_exit();
    assert!(!node.is_running());
}

#[test]
fn test_node_pause_resume() {
    let mut node = Node::new();
    
    assert!(!node.is_paused());
    
    node.pause();
    assert!(node.is_paused());
    
    node.resume();
    assert!(!node.is_paused());
}

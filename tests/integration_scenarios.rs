// 集成场景测试 - 模拟游戏开发中的完整使用场景

use cocos2d_rust::scene::{Scene, Node};
use cocos2d_rust::math::Vec2;
use cocos2d_rust::UserDefault;
use std::rc::Rc;
use std::cell::RefCell;

#[test]
fn test_complete_scene_setup() {
    // 创建场景
    let scene = Rc::new(RefCell::new(Scene::new()));
    let node = Rc::new(RefCell::new(Node::new()));
    node.borrow_mut().set_position(Vec2::new(400.0, 300.0));
    
    scene.borrow_mut().add_child(node.clone(), 0, None);
    
    // 验证场景包含节点
    assert_eq!(scene.borrow().children().len(), 1);
}

#[test]
fn test_multiple_nodes_interaction() {
    let parent = Rc::new(RefCell::new(Node::new()));
    
    for i in 0..10 {
        let node = Rc::new(RefCell::new(Node::new()));
        node.borrow_mut().set_position(Vec2::new(i as f32 * 50.0, 100.0));
        node.borrow_mut().set_tag(i);
        Node::add_child_to_parent(&parent, node, 0, None);
    }
    
    assert_eq!(parent.borrow().get_children_count(), 10);
}

#[test]
fn test_scene_hierarchy() {
    let scene = Rc::new(RefCell::new(Scene::new()));
    
    let parent_node = Rc::new(RefCell::new(Node::new()));
    let child_node = Rc::new(RefCell::new(Node::new()));
    
    Node::add_child_to_parent(&parent_node, child_node.clone(), 0, None);
    scene.borrow_mut().add_child(parent_node.clone(), 0, None);
    
    // 场景有 1 个直接子节点
    assert_eq!(scene.borrow().children().len(), 1);
    // 父节点有 1 个子节点
    assert_eq!(parent_node.borrow().get_children_count(), 1);
}

#[test]
fn test_node_transform() {
    let node = Rc::new(RefCell::new(Node::new()));
    
    node.borrow_mut().set_position(Vec2::new(100.0, 200.0));
    node.borrow_mut().set_scale(2.0);
    node.borrow_mut().set_rotation(45.0);
    
    let pos = node.borrow().get_position();
    assert!((pos.x - 100.0).abs() < 0.01);
    assert!((pos.y - 200.0).abs() < 0.01);
    
    assert!((node.borrow().scale() - 2.0).abs() < 0.01);
    assert!((node.borrow().get_rotation() - 45.0).abs() < 0.01);
}

#[test]
fn test_save_and_load_user_data() {
    let defaults = UserDefault::get_instance();
    let mut defaults = defaults.lock().unwrap();
    
    defaults.set_int("score", 1000);
    defaults.set_string("player_name", "Hero");
    defaults.set_bool("sound_enabled", true);
    
    assert_eq!(defaults.get_int("score", 0), 1000);
    assert_eq!(defaults.get_string("player_name", ""), "Hero");
    assert!(defaults.get_bool("sound_enabled", false));
}

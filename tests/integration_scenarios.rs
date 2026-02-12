use cocos2d_rust::base::Director;
use cocos2d_rust::scene::{Scene, Node, Layer};
use cocos2d_rust::sprite::Sprite;
use cocos2d_rust::math::Vec2;

#[test]
fn test_complete_scene_setup() {
    let mut director = Director::new();
    let mut scene = Scene::new();
    let mut layer = Layer::new();
    
    let mut sprite = Sprite::new();
    sprite.set_position(Vec2::new(400.0, 300.0));
    
    layer.add_child(sprite, 0, -1);
    scene.add_child(layer, 0, -1);
    director.run_scene(scene);
    
    assert!(director.get_running_scene().is_some());
}

#[test]
fn test_multiple_sprites_interaction() {
    let mut layer = Layer::new();
    
    for i in 0..10 {
        let mut sprite = Sprite::new();
        sprite.set_position(Vec2::new(i as f32 * 50.0, 100.0));
        sprite.set_tag(i);
        layer.add_child(sprite, 0, i);
    }
    
    assert_eq!(layer.get_children_count(), 10);
}

#[test]
fn test_scene_transition() {
    let mut director = Director::new();
    
    let scene1 = Scene::new();
    director.run_scene(scene1);
    
    let scene2 = Scene::new();
    director.replace_scene(scene2);
    
    assert!(director.get_running_scene().is_some());
}

#[test]
fn test_game_loop_simulation() {
    let mut director = Director::new();
    let scene = Scene::new();
    director.run_scene(scene);
    
    for _ in 0..60 {
        director.main_loop();
    }
    
    assert!(director.get_total_time() > 0.0);
}

#[test]
fn test_save_and_load_user_data() {
    let defaults = UserDefault::get_instance();
    
    defaults.set_integer_for_key("score", 1000);
    defaults.set_string_for_key("player_name", "Hero");
    defaults.set_bool_for_key("sound_enabled", true);
    defaults.flush();
    
    assert_eq!(defaults.get_integer_for_key("score", 0), 1000);
    assert_eq!(defaults.get_string_for_key("player_name", ""), "Hero");
    assert!(defaults.get_bool_for_key("sound_enabled", false));
}

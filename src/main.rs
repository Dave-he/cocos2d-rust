use cocos2d_rust::base::types::{Color3B, Rect};
use cocos2d_rust::base::RefPtr;
use cocos2d_rust::math::Vec2;
use cocos2d_rust::platform::application::{Application, ApplicationDelegate};
use cocos2d_rust::sprite::Sprite;
use cocos2d_rust::{Director, Scene};

struct AppDelegate;

impl ApplicationDelegate for AppDelegate {
    fn application_did_finish_launching(&mut self) -> bool {
        let mut director = Director::get_instance();
        let scene = Scene::new();

        let mut sprite = Sprite::new();
        sprite.set_texture_rect(Rect::new(0.0, 0.0, 200.0, 200.0));
        sprite.set_color(Color3B::new(255, 0, 0));
        sprite
            .get_node_mut()
            .borrow_mut()
            .set_position(Vec2::new(480.0, 320.0));

        scene.borrow_mut().add_child(sprite.get_node().clone());

        director.borrow_mut().run_scene(RefPtr::new(scene));
        log::info!("Application launched successfully!");
        true
    }

    fn application_did_enter_background(&mut self) {
        log::info!("Application enter background");
    }

    fn application_will_enter_foreground(&mut self) {
        log::info!("Application enter foreground");
    }
}

fn main() {
    env_logger::init();
    let mut app = Application::new();
    app.set_delegate(Box::new(AppDelegate));
    app.run();
}

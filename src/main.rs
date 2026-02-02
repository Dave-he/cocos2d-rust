use cocos2d_rust::platform::application::{Application, ApplicationDelegate};
use cocos2d_rust::{Director, Scene};
use cocos2d_rust::base::RefPtr;
use cocos2d_rust::sprite::Sprite;
use cocos2d_rust::base::types::{Rect, Color3B};
use cocos2d_rust::math::Vec2;
use cocos2d_rust::audio::{AudioEngine, generate_beep, generate_click};

struct AppDelegate;

impl ApplicationDelegate for AppDelegate {
    fn application_did_finish_launching(&mut self) -> bool {
        let director = Director::get_instance();
        let scene = Scene::new();
        
        let mut sprite = Sprite::new();
        sprite.set_texture_rect(Rect::new(0.0, 0.0, 200.0, 200.0));
        sprite.set_color(Color3B::new(255, 0, 0));
        sprite.get_node_mut().borrow_mut().set_position(Vec2::new(480.0, 320.0));
        
        let mut scene = scene;
        scene.add_child(sprite.get_node().clone());

        director.borrow_mut().run_scene(RefPtr::new(scene));

        if AudioEngine::init() {
            log::info!("AudioEngine initialized successfully!");
            
            generate_beep("beep.wav", 440.0, 0.5);
            generate_click("click.wav");
            log::info!("Generated test audio files");
            
            let audio_id = AudioEngine::play2d("beep.wav", false, 1.0);
            if audio_id > 0 {
                log::info!("Playing beep sound with ID: {}", audio_id);
            } else {
                log::warn!("Failed to play audio");
            }
        } else {
            log::error!("Failed to initialize AudioEngine");
        }

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

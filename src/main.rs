use cocos2d_rust::base::RefPtr;
use cocos2d_rust::platform::application::{Application, ApplicationDelegate};
use cocos2d_rust::{Director, Scene};
use cocos2d_rust::audio::{AudioEngine, generate_beep, generate_click};

struct AppDelegate;

impl ApplicationDelegate for AppDelegate {
    fn application_did_finish_launching(&mut self) -> bool {
        let director = Director::get_instance();
        let scene = Scene::new();

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

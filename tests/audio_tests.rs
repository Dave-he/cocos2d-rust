use cocos2d_rust::audio::{AudioEngine, SimpleAudioEngine};

#[test]
fn test_audio_engine_creation() {
    let engine = AudioEngine::new();
    assert!(!engine.is_initialized());
}

#[test]
fn test_simple_audio_engine_singleton() {
    let engine = SimpleAudioEngine::get_instance();
    assert!(engine.is_some());
}

#[test]
fn test_audio_preload_effect() {
    let mut engine = AudioEngine::new();
    
    engine.preload_effect("sound.wav");
}

#[test]
fn test_audio_play_effect() {
    let mut engine = AudioEngine::new();
    
    let id = engine.play_effect("sound.wav", false, 1.0, 1.0, 0.0);
    assert!(id > 0);
}

#[test]
fn test_audio_pause_effect() {
    let mut engine = AudioEngine::new();
    
    let id = engine.play_effect("sound.wav", false, 1.0, 1.0, 0.0);
    engine.pause_effect(id);
}

#[test]
fn test_audio_resume_effect() {
    let mut engine = AudioEngine::new();
    
    let id = engine.play_effect("sound.wav", false, 1.0, 1.0, 0.0);
    engine.pause_effect(id);
    engine.resume_effect(id);
}

#[test]
fn test_audio_stop_effect() {
    let mut engine = AudioEngine::new();
    
    let id = engine.play_effect("sound.wav", false, 1.0, 1.0, 0.0);
    engine.stop_effect(id);
}

#[test]
fn test_audio_set_volume() {
    let mut engine = AudioEngine::new();
    
    engine.set_effects_volume(0.5);
    assert_eq!(engine.get_effects_volume(), 0.5);
}

#[test]
fn test_audio_play_background_music() {
    let mut engine = AudioEngine::new();
    
    engine.play_background_music("music.mp3", false);
}

#[test]
fn test_audio_stop_background_music() {
    let mut engine = AudioEngine::new();
    
    engine.play_background_music("music.mp3", false);
    engine.stop_background_music(false);
}

#[test]
fn test_audio_pause_background_music() {
    let mut engine = AudioEngine::new();
    
    engine.play_background_music("music.mp3", false);
    engine.pause_background_music();
}

#[test]
fn test_audio_background_music_volume() {
    let mut engine = AudioEngine::new();
    
    engine.set_background_music_volume(0.7);
    assert_eq!(engine.get_background_music_volume(), 0.7);
}

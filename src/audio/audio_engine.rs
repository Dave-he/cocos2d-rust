use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::path::PathBuf;

use super::audio_player::{AudioPlayer, AudioSource, AudioBuffer, AudioListener, AudioState};

#[derive(Debug)]
pub struct AudioEngine {
    audio_players: HashMap<i32, Arc<Mutex<AudioPlayer>>>,
    audio_buffers: HashMap<String, Arc<Mutex<AudioBuffer>>>,
    audio_sources: HashMap<String, Arc<Mutex<AudioSource>>>,
    max_audio_sources: usize,
    current_audio_id: i32,
    mute: bool,
    volume: f32,
    listener: AudioListener,
}

impl AudioEngine {
    pub fn new() -> AudioEngine {
        AudioEngine {
            audio_players: HashMap::new(),
            audio_buffers: HashMap::new(),
            audio_sources: HashMap::new(),
            max_audio_sources: 32,
            current_audio_id: 0,
            mute: false,
            volume: 1.0,
            listener: AudioListener::new(),
        }
    }

    pub fn init() -> bool {
        true
    }

    pub fn end() {
        AudioEngine::stop_all();
    }

    pub fn get_instance() -> &'static mut AudioEngine {
        static mut AUDIO_ENGINE: Option<AudioEngine> = None;
        unsafe {
            if AUDIO_ENGINE.is_none() {
                AUDIO_ENGINE = Some(AudioEngine::new());
            }
            AUDIO_ENGINE.as_mut().unwrap()
        }
    }

    pub fn preload(file_path: &str) {
        let mut engine = Self::get_instance();
        engine.preload_internal(file_path);
    }

    fn preload_internal(&mut self, file_path: &str) {
        let path = PathBuf::from(file_path);
        if path.exists() {
            self.audio_buffers.insert(file_path.to_string(), Arc::new(Mutex::new(AudioBuffer::new())));
        }
    }

    pub fn play2d(file_path: &str, loop_enabled: bool, volume: f32) -> i32 {
        let mut engine = Self::get_instance();
        engine.play2d_internal(file_path, loop_enabled, volume)
    }

    fn play2d_internal(&mut self, file_path: &str, loop_enabled: bool, volume: f32) -> i32 {
        self.current_audio_id += 1;

        let mut player = AudioPlayer::new();
        player.set_id(self.current_audio_id);
        player.set_volume(volume);
        player.set_current_time(Duration::ZERO);

        let mut source = AudioSource::new(file_path);
        source.set_loop_enabled(loop_enabled);
        source.set_volume(volume);

        let id = self.current_audio_id;
        self.audio_players.insert(id, Arc::new(Mutex::new(player)));
        self.audio_sources.insert(file_path.to_string(), Arc::new(Mutex::new(source)));

        // 通过 HashMap 获取并调用 play
        if let Some(player_arc) = self.audio_players.get(&id) {
            let mut player = player_arc.lock().unwrap();
            player.play();
        }
        
        id
    }

    pub fn set_loop(audio_id: i32, loop_enabled: bool) {
        if let Some(player) = Self::get_instance().audio_players.get(&audio_id) {
            let mut player = player.lock().unwrap();
            if let Some(source) = &player.source {
                let mut source = source.lock().unwrap();
                source.set_loop_enabled(loop_enabled);
            }
        }
    }

    pub fn set_volume(audio_id: i32, volume: f32) {
        if let Some(player) = Self::get_instance().audio_players.get(&audio_id) {
            let mut player = player.lock().unwrap();
            player.set_volume(volume);
        }
    }

    pub fn pause(audio_id: i32) {
        if let Some(player) = Self::get_instance().audio_players.get(&audio_id) {
            let mut player = player.lock().unwrap();
            player.pause();
        }
    }

    pub fn resume(audio_id: i32) {
        if let Some(player) = Self::get_instance().audio_players.get(&audio_id) {
            let mut player = player.lock().unwrap();
            player.play();
        }
    }

    pub fn stop(audio_id: i32) {
        if let Some(player) = Self::get_instance().audio_players.get(&audio_id) {
            let mut player = player.lock().unwrap();
            player.stop();
        }
    }

    pub fn stop_all() {
        let engine = Self::get_instance();
        for player in engine.audio_players.values() {
            let mut player = player.lock().unwrap();
            player.stop();
        }
    }

    pub fn is_playing(audio_id: i32) -> bool {
        if let Some(player) = Self::get_instance().audio_players.get(&audio_id) {
            let player = player.lock().unwrap();
            player.is_playing()
        } else {
            false
        }
    }

    pub fn get_current_time(audio_id: i32) -> f32 {
        if let Some(player) = Self::get_instance().audio_players.get(&audio_id) {
            let player = player.lock().unwrap();
            player.get_current_time().as_secs_f32()
        } else {
            0.0
        }
    }

    pub fn set_current_time(audio_id: i32, time: f32) {
        if let Some(player) = Self::get_instance().audio_players.get(&audio_id) {
            let mut player = player.lock().unwrap();
            player.set_current_time(Duration::from_secs_f32(time));
        }
    }

    pub fn get_duration(audio_id: i32) -> f32 {
        if let Some(player) = Self::get_instance().audio_players.get(&audio_id) {
            let player = player.lock().unwrap();
            player.get_duration().as_secs_f32()
        } else {
            0.0
        }
    }

    pub fn get_max_audio_sources() -> usize {
        32
    }

    pub fn get_state(audio_id: i32) -> AudioState {
        if let Some(player) = Self::get_instance().audio_players.get(&audio_id) {
            let player = player.lock().unwrap();
            player.get_state()
        } else {
            AudioState::STOPPED
        }
    }

    pub fn set_mute(enabled: bool) {
        let engine = Self::get_instance();
        engine.mute = enabled;
    }

    pub fn is_mute() -> bool {
        Self::get_instance().mute
    }

    pub fn get_volume() -> f32 {
        Self::get_instance().volume
    }

    pub fn set_volume(volume: f32) {
        let engine = Self::get_instance();
        engine.volume = volume.clamp(0.0, 1.0);
    }

    pub fn get_listener_volume() -> f32 {
        Self::get_instance().listener.get_volume()
    }

    pub fn set_listener_volume(volume: f32) {
        Self::get_instance().listener.set_volume(volume);
    }

    pub fn uncache(file_path: &str) {
        let engine = Self::get_instance();
        engine.audio_buffers.remove(file_path);
    }

    pub fn uncache_all() {
        let engine = Self::get_instance();
        engine.audio_buffers.clear();
    }
}

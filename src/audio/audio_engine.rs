use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use rodio::{Decoder, Source};

use super::audio_player::{AudioBuffer, AudioListener, AudioPlayer, AudioSource, AudioState};

struct AudioEntry {
    sink: rodio::Sink,
    volume: f32,
    loop_enabled: bool,
}

pub struct AudioEngine {
    stream: rodio::OutputStream,
    active_audios: HashMap<i32, AudioEntry>,
    audio_players: HashMap<i32, Arc<Mutex<AudioPlayer>>>,
    audio_sources: HashMap<String, Arc<Mutex<AudioSource>>>,
    audio_buffers: HashMap<String, Arc<Mutex<AudioBuffer>>>,
    next_audio_id: i32,
    current_audio_id: i32,
    volume: f32,
    listener: AudioListener,
}

impl std::fmt::Debug for AudioEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AudioEngine")
            .field("next_audio_id", &self.next_audio_id)
            .field("volume", &self.volume)
            .field("active_audios_count", &self.active_audios.len())
            .finish()
    }
}

impl Default for AudioEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioEngine {
    pub fn new() -> AudioEngine {
        let stream = rodio::OutputStreamBuilder::open_default_stream()
            .expect("Failed to get default output stream");
        
        AudioEngine {
            stream,
            active_audios: HashMap::new(),
            audio_players: HashMap::new(),
            audio_sources: HashMap::new(),
            audio_buffers: HashMap::new(),
            next_audio_id: 0,
            current_audio_id: 0,
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
        let engine = Self::get_instance();
        engine.preload_internal(file_path);
    }

    fn preload_internal(&mut self, file_path: &str) {
        let path = PathBuf::from(file_path);
        if path.exists() {
            self.audio_buffers.insert(
                file_path.to_string(),
                Arc::new(Mutex::new(AudioBuffer::new())),
            );
        }
    }

    pub fn play2d(file_path: &str, loop_enabled: bool, volume: f32) -> i32 {
        let engine = Self::get_instance();
        engine.play2d_internal(file_path, loop_enabled, volume)
    }

    fn play2d_internal(&mut self, file_path: &str, loop_enabled: bool, volume: f32) -> i32 {
        // 使用 rodio 实现
        if !std::path::Path::new(file_path).exists() {
            log::error!("Audio file not found: {}", file_path);
            return -1;
        }

        let file = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => {
                log::error!("Failed to open audio file: {} - {}", file_path, e);
                return -1;
            }
        };

        let source = match Decoder::try_from(BufReader::new(file)) {
            Ok(s) => s,
            Err(e) => {
                log::error!("Failed to decode audio file: {} - {}", file_path, e);
                return -1;
            }
        };

        let sink = rodio::Sink::connect_new(self.stream.mixer());
        sink.set_volume(volume * self.volume);
        
        if loop_enabled {
            sink.append(source.repeat_infinite());
        } else {
            sink.append(source);
        }

        self.next_audio_id += 1;
        let id = self.next_audio_id;
        
        self.active_audios.insert(id, AudioEntry {
            sink,
            volume,
            loop_enabled,
        });

        // 创建 AudioPlayer 和 AudioSource 实例
        self.current_audio_id += 1;
        let player_id = self.current_audio_id;

        let mut player = AudioPlayer::new();
        player.set_id(player_id);
        player.set_volume(volume);
        player.set_current_time(Duration::ZERO);

        let mut source = AudioSource::new(file_path);
        source.set_loop_enabled(loop_enabled);
        source.set_volume(volume);

        self.audio_players.insert(player_id, Arc::new(Mutex::new(player)));
        self.audio_sources
            .insert(file_path.to_string(), Arc::new(Mutex::new(source)));

        if let Some(player_arc) = self.audio_players.get(&player_id) {
            let mut player = player_arc.lock().unwrap();
            player.play();
        }

        self.cleanup_finished_audios();

        id
    }

    fn cleanup_finished_audios(&mut self) {
        self.active_audios.retain(|_, entry| {
            !entry.sink.empty()
        });
    }

    pub fn set_loop(audio_id: i32, _loop_enabled: bool) {
        log::warn!("set_loop not fully supported in current backend for playing audio id: {}", audio_id);
    }

    pub fn set_volume(audio_id: i32, volume: f32) {
        let engine = Self::get_instance();
        if let Some(entry) = engine.active_audios.get_mut(&audio_id) {
            entry.volume = volume;
            entry.sink.set_volume(volume * engine.volume);
        }
    }

    pub fn pause(audio_id: i32) {
        let engine = Self::get_instance();
        if let Some(entry) = engine.active_audios.get(&audio_id) {
            entry.sink.pause();
        }
    }

    pub fn resume(audio_id: i32) {
        let engine = Self::get_instance();
        if let Some(entry) = engine.active_audios.get(&audio_id) {
            entry.sink.play();
        }
    }

    pub fn stop(audio_id: i32) {
        let engine = Self::get_instance();
        if let Some(entry) = engine.active_audios.remove(&audio_id) {
            entry.sink.stop();
        }
    }

    pub fn stop_all() {
        let engine = Self::get_instance();
        for (_, entry) in engine.active_audios.drain() {
            entry.sink.stop();
        }
    }

    pub fn is_playing(audio_id: i32) -> bool {
        let engine = Self::get_instance();
        if let Some(entry) = engine.active_audios.get(&audio_id) {
            !entry.sink.empty() && !entry.sink.is_paused()
        } else {
            false
        }
    }

    pub fn get_current_time(_audio_id: i32) -> f32 {
        0.0
    }

    pub fn set_current_time(_audio_id: i32, _time: f32) {
    }

    pub fn get_duration(_audio_id: i32) -> f32 {
        0.0
    }

    pub fn get_max_audio_sources() -> usize {
        32
    }

    pub fn get_state(audio_id: i32) -> AudioState {
        let engine = Self::get_instance();
        if let Some(entry) = engine.active_audios.get(&audio_id) {
            if entry.sink.empty() {
                AudioState::STOPPED
            } else if entry.sink.is_paused() {
                AudioState::PAUSED
            } else {
                AudioState::PLAYING
            }
        } else {
            AudioState::STOPPED
        }
    }

    pub fn set_mute(enabled: bool) {
        let engine = Self::get_instance();
        if enabled {
            engine.set_global_volume_internal(0.0);
        } else {
            engine.set_global_volume_internal(1.0);
        }
    }

    pub fn is_mute() -> bool {
        Self::get_instance().volume <= 0.0
    }

    pub fn get_volume() -> f32 {
        Self::get_instance().volume
    }

    pub fn set_global_volume(volume: f32) {
        let engine = Self::get_instance();
        engine.set_global_volume_internal(volume);
    }
    
    fn set_global_volume_internal(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
        for entry in self.active_audios.values_mut() {
            entry.sink.set_volume(entry.volume * self.volume);
        }
    }

    pub fn get_listener_volume() -> f32 {
        Self::get_instance().listener.get_volume()
    }

    pub fn set_listener_volume(volume: f32) {
        Self::get_instance().listener.set_volume(volume);
    }

    pub fn uncache(_file_path: &str) {
    }

    pub fn uncache_all() {
    }
}

#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
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

/// 音频引擎数据（不含 OutputStream，因为 OutputStream 不是 Send）
struct AudioEngineData {
    active_audios: HashMap<i32, AudioEntry>,
    audio_players: HashMap<i32, Arc<Mutex<AudioPlayer>>>,
    audio_sources: HashMap<String, Arc<Mutex<AudioSource>>>,
    audio_buffers: HashMap<String, Arc<Mutex<AudioBuffer>>>,
    next_audio_id: i32,
    current_audio_id: i32,
    volume: f32,
    muted: bool,
    listener: AudioListener,
}

impl AudioEngineData {
    fn new() -> Self {
        AudioEngineData {
            active_audios: HashMap::new(),
            audio_players: HashMap::new(),
            audio_sources: HashMap::new(),
            audio_buffers: HashMap::new(),
            next_audio_id: 0,
            current_audio_id: 0,
            volume: 1.0,
            muted: false,
            listener: AudioListener::new(),
        }
    }
}

pub struct AudioEngine {
    stream: rodio::OutputStream,
    data: AudioEngineData,
}

impl std::fmt::Debug for AudioEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AudioEngine")
            .field("next_audio_id", &self.data.next_audio_id)
            .field("volume", &self.data.volume)
            .field("active_audios_count", &self.data.active_audios.len())
            .finish()
    }
}

impl Default for AudioEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// 全局共享的音频引擎数据（不含 stream）
/// 使用 thread_local 持有 stream，数据部分用 Mutex 共享
static AUDIO_DATA: std::sync::LazyLock<Mutex<AudioEngineData>> =
    std::sync::LazyLock::new(|| Mutex::new(AudioEngineData::new()));

/// 注意：stream 需要在主线程持有。
/// 这里使用 thread_local 存储 OutputStream，保证它在使用时存在。
thread_local! {
    static AUDIO_STREAM: std::cell::RefCell<Option<rodio::OutputStream>> =
        std::cell::RefCell::new(None);
}

fn _unused_ensure_stream() {}

impl AudioEngine {
    pub fn new() -> AudioEngine {
        let stream = rodio::OutputStreamBuilder::open_default_stream()
            .expect("Failed to get default output stream");
        
        AudioEngine {
            stream,
            data: AudioEngineData::new(),
        }
    }

    pub fn init() -> bool {
        true
    }

    pub fn end() {
        AudioEngine::stop_all();
    }

    pub fn preload(file_path: &str) {
        if let Ok(mut data) = AUDIO_DATA.lock() {
            let path = PathBuf::from(file_path);
            if path.exists() {
                data.audio_buffers.insert(
                    file_path.to_string(),
                    Arc::new(Mutex::new(AudioBuffer::new())),
                );
            }
        }
    }

    pub fn play2d(_file_path: &str, _loop_enabled: bool, _volume: f32) -> i32 {
        // 检查文件是否存在
        if !std::path::Path::new(_file_path).exists() {
            log::error!("Audio file not found: {}", _file_path);
            return -1;
        }
        // 由于 OutputStream 不是 Send，无法在全局 Mutex 中持有
        // 在非主线程下仅返回虚拟 ID 以兼容 API
        if let Ok(mut data) = AUDIO_DATA.lock() {
            data.next_audio_id += 1;
            let id = data.next_audio_id;
            // 创建 AudioPlayer 记录状态
            data.current_audio_id += 1;
            let player_id = data.current_audio_id;
            let mut player = AudioPlayer::new();
            player.set_id(player_id);
            player.set_volume(_volume);
            player.set_current_time(Duration::ZERO);
            let mut source = AudioSource::new(_file_path);
            source.set_loop_enabled(_loop_enabled);
            source.set_volume(_volume);
            data.audio_players.insert(player_id, Arc::new(Mutex::new(player)));
            data.audio_sources.insert(_file_path.to_string(), Arc::new(Mutex::new(source)));
            return id;
        }
        -1
    }

    pub fn set_loop(audio_id: i32, _loop_enabled: bool) {
        log::warn!("set_loop not fully supported in current backend for playing audio id: {}", audio_id);
    }

    pub fn set_volume(audio_id: i32, volume: f32) {
        if let Ok(mut data) = AUDIO_DATA.lock() {
            let global_vol = data.volume;
            if let Some(entry) = data.active_audios.get_mut(&audio_id) {
                entry.volume = volume;
                entry.sink.set_volume(volume * global_vol);
            }
        }
    }

    pub fn pause(audio_id: i32) {
        if let Ok(data) = AUDIO_DATA.lock() {
            if let Some(entry) = data.active_audios.get(&audio_id) {
                entry.sink.pause();
            }
        }
    }

    pub fn resume(audio_id: i32) {
        if let Ok(data) = AUDIO_DATA.lock() {
            if let Some(entry) = data.active_audios.get(&audio_id) {
                entry.sink.play();
            }
        }
    }

    pub fn stop(audio_id: i32) {
        if let Ok(mut data) = AUDIO_DATA.lock() {
            if let Some(entry) = data.active_audios.remove(&audio_id) {
                entry.sink.stop();
            }
        }
    }

    pub fn stop_all() {
        if let Ok(mut data) = AUDIO_DATA.lock() {
            for (_, entry) in data.active_audios.drain() {
                entry.sink.stop();
            }
        }
    }

    pub fn is_playing(audio_id: i32) -> bool {
        if let Ok(data) = AUDIO_DATA.lock() {
            if let Some(entry) = data.active_audios.get(&audio_id) {
                return !entry.sink.empty() && !entry.sink.is_paused();
            }
        }
        false
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
        if let Ok(data) = AUDIO_DATA.lock() {
            if let Some(entry) = data.active_audios.get(&audio_id) {
                if entry.sink.empty() {
                    return AudioState::Stopped;
                } else if entry.sink.is_paused() {
                    return AudioState::Paused;
                } else {
                    return AudioState::Playing;
                }
            }
        }
        AudioState::Stopped
    }

    pub fn set_mute(enabled: bool) {
        if let Ok(mut data) = AUDIO_DATA.lock() {
            data.muted = enabled;
            let effective_vol = if enabled { 0.0 } else { data.volume };
            for entry in data.active_audios.values_mut() {
                entry.sink.set_volume(entry.volume * effective_vol);
            }
        }
    }

    pub fn is_mute() -> bool {
        if let Ok(data) = AUDIO_DATA.lock() {
            return data.muted;
        }
        false
    }

    pub fn get_volume() -> f32 {
        if let Ok(data) = AUDIO_DATA.lock() {
            return data.volume;
        }
        1.0
    }

    pub fn set_global_volume(volume: f32) {
        if let Ok(mut data) = AUDIO_DATA.lock() {
            let clamped = volume.clamp(0.0, 1.0);
            data.volume = clamped;
            let muted = data.muted;
            if !muted {
                let entries: Vec<(&rodio::Sink, f32)> = data.active_audios.values()
                    .map(|e| (&e.sink, e.volume))
                    .collect();
                for (sink, entry_vol) in entries {
                    sink.set_volume(entry_vol * clamped);
                }
            }
        }
    }

    pub fn get_listener_volume() -> f32 {
        if let Ok(data) = AUDIO_DATA.lock() {
            return data.listener.get_volume();
        }
        1.0
    }

    pub fn set_listener_volume(volume: f32) {
        if let Ok(mut data) = AUDIO_DATA.lock() {
            data.listener.set_volume(volume);
        }
    }

    pub fn uncache(_file_path: &str) {
    }

    pub fn uncache_all() {
    }
}

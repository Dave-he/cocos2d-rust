use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct AudioSource {
    path: String,
    volume: f32,
    loop_enabled: bool,
    pitch: f32,
    pan: f32,
    priority: i32,
}

impl AudioSource {
    pub fn new(path: &str) -> AudioSource {
        AudioSource {
            path: path.to_string(),
            volume: 1.0,
            loop_enabled: false,
            pitch: 1.0,
            pan: 0.0,
            priority: 0,
        }
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub fn get_volume(&self) -> f32 {
        self.volume
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
    }

    pub fn is_loop_enabled(&self) -> bool {
        self.loop_enabled
    }

    pub fn set_loop_enabled(&mut self, enabled: bool) {
        self.loop_enabled = enabled;
    }

    pub fn get_pitch(&self) -> f32 {
        self.pitch
    }

    pub fn set_pitch(&mut self, pitch: f32) {
        self.pitch = pitch;
    }

    pub fn get_pan(&self) -> f32 {
        self.pan
    }

    pub fn set_pan(&mut self, pan: f32) {
        self.pan = pan.clamp(-1.0, 1.0);
    }

    pub fn get_priority(&self) -> i32 {
        self.priority
    }

    pub fn set_priority(&mut self, priority: i32) {
        self.priority = priority;
    }
}

#[derive(Debug)]
pub struct AudioPlayer {
    id: i32,
    pub source: Option<Arc<Mutex<AudioSource>>>,
    state: AudioState,
    volume: f32,
    current_time: Duration,
    duration: Duration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioState {
    INITIALIZING,
    PLAYING,
    PAUSED,
    STOPPED,
}

impl AudioPlayer {
    pub fn new() -> AudioPlayer {
        AudioPlayer {
            id: 0,
            source: None,
            state: AudioState::INITIALIZING,
            volume: 1.0,
            current_time: Duration::ZERO,
            duration: Duration::ZERO,
        }
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn set_id(&mut self, id: i32) {
        self.id = id;
    }

    pub fn get_state(&self) -> AudioState {
        self.state
    }

    pub fn get_volume(&self) -> f32 {
        self.volume
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
    }

    pub fn get_current_time(&self) -> Duration {
        self.current_time
    }

    pub fn set_current_time(&mut self, time: Duration) {
        self.current_time = time;
    }

    pub fn get_duration(&self) -> Duration {
        self.duration
    }

    pub fn set_duration(&mut self, duration: Duration) {
        self.duration = duration;
    }

    pub fn play(&mut self) {
        self.state = AudioState::PLAYING;
    }

    pub fn pause(&mut self) {
        self.state = AudioState::PAUSED;
    }

    pub fn stop(&mut self) {
        self.state = AudioState::STOPPED;
    }

    pub fn is_playing(&self) -> bool {
        self.state == AudioState::PLAYING
    }

    pub fn is_paused(&self) -> bool {
        self.state == AudioState::PAUSED
    }

    pub fn is_stopped(&self) -> bool {
        self.state == AudioState::STOPPED
    }
}

#[derive(Debug)]
pub struct AudioBuffer {
    id: u32,
    sample_rate: u32,
    channels: u32,
    bits_per_sample: u32,
    duration: Duration,
    size: usize,
}

impl AudioBuffer {
    pub fn new() -> AudioBuffer {
        AudioBuffer {
            id: 0,
            sample_rate: 44100,
            channels: 2,
            bits_per_sample: 16,
            duration: Duration::ZERO,
            size: 0,
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn get_channels(&self) -> u32 {
        self.channels
    }

    pub fn get_bits_per_sample(&self) -> u32 {
        self.bits_per_sample
    }

    pub fn get_duration(&self) -> Duration {
        self.duration
    }

    pub fn get_size(&self) -> usize {
        self.size
    }
}

#[derive(Debug)]
pub struct AudioListener {
    volume: f32,
}

impl AudioListener {
    pub fn new() -> AudioListener {
        AudioListener { volume: 1.0 }
    }

    pub fn get_volume(&self) -> f32 {
        self.volume
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
    }
}

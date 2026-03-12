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
    Initializing,
    Playing,
    Paused,
    Stopped,
}

impl Default for AudioPlayer {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioPlayer {
    pub fn new() -> AudioPlayer {
        AudioPlayer {
            id: 0,
            source: None,
            state: AudioState::Initializing,
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
        self.state = AudioState::Playing;
    }

    pub fn pause(&mut self) {
        self.state = AudioState::Paused;
    }

    pub fn stop(&mut self) {
        self.state = AudioState::Stopped;
    }

    pub fn is_playing(&self) -> bool {
        self.state == AudioState::Playing
    }

    pub fn is_paused(&self) -> bool {
        self.state == AudioState::Paused
    }

    pub fn is_stopped(&self) -> bool {
        self.state == AudioState::Stopped
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

impl Default for AudioBuffer {
    fn default() -> Self {
        Self::new()
    }
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

impl Default for AudioListener {
    fn default() -> Self {
        Self::new()
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_source_creation() {
        let source = AudioSource::new("test.mp3");
        assert_eq!(source.get_path(), "test.mp3");
        assert_eq!(source.get_volume(), 1.0);
        assert!(!source.is_loop_enabled());
    }

    #[test]
    fn test_audio_source_volume() {
        let mut source = AudioSource::new("test.mp3");
        source.set_volume(0.5);
        assert_eq!(source.get_volume(), 0.5);
        
        source.set_volume(1.5);
        assert_eq!(source.get_volume(), 1.0);
        
        source.set_volume(-0.1);
        assert_eq!(source.get_volume(), 0.0);
    }

    #[test]
    fn test_audio_source_loop() {
        let mut source = AudioSource::new("test.mp3");
        assert!(!source.is_loop_enabled());
        source.set_loop_enabled(true);
        assert!(source.is_loop_enabled());
    }

    #[test]
    fn test_audio_source_pan() {
        let mut source = AudioSource::new("test.mp3");
        source.set_pan(0.5);
        assert_eq!(source.get_pan(), 0.5);
        
        source.set_pan(2.0);
        assert_eq!(source.get_pan(), 1.0);
        
        source.set_pan(-2.0);
        assert_eq!(source.get_pan(), -1.0);
    }

    #[test]
    fn test_audio_player_creation() {
        let player = AudioPlayer::new();
        assert_eq!(player.get_id(), 0);
        assert_eq!(player.get_state(), AudioState::Initializing);
        assert_eq!(player.get_volume(), 1.0);
    }

    #[test]
    fn test_audio_player_state() {
        let mut player = AudioPlayer::new();
        assert_eq!(player.get_state(), AudioState::Initializing);
        assert!(!player.is_playing());
        assert!(!player.is_paused());
        
        player.play();
        assert!(player.is_playing());
        assert!(!player.is_paused());
        
        player.pause();
        assert!(!player.is_playing());
        assert!(player.is_paused());
        
        player.stop();
        assert!(!player.is_playing());
        assert!(!player.is_paused());
        assert!(player.is_stopped());
    }

    #[test]
    fn test_audio_player_volume() {
        let mut player = AudioPlayer::new();
        player.set_volume(0.7);
        assert_eq!(player.get_volume(), 0.7);
    }

    #[test]
    fn test_audio_buffer_creation() {
        let buffer = AudioBuffer::new();
        assert_eq!(buffer.get_id(), 0);
        assert_eq!(buffer.get_sample_rate(), 44100);
        assert_eq!(buffer.get_channels(), 2);
        assert_eq!(buffer.get_bits_per_sample(), 16);
    }

    #[test]
    fn test_audio_listener() {
        let mut listener = AudioListener::new();
        assert_eq!(listener.get_volume(), 1.0);
        
        listener.set_volume(0.5);
        assert_eq!(listener.get_volume(), 0.5);
    }
}

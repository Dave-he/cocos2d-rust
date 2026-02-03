/// VideoPlayer - 视频播放器组件
///
/// 功能：
/// - 多种视频格式支持
/// - 播放控制（播放/暂停/停止/跳转）
/// - 音量控制
/// - 循环播放
/// - 进度回调
/// - 事件委托
/// - 全屏播放
/// - 画中画模式

use std::time::{Duration, Instant};
use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

use crate::base::Color4B;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VideoPlayerState {
    Unknown,
    Loading,
    ReadyToPlay,
    Playing,
    Paused,
    Stoped,
    Completed,
    Error(String),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VideoSourceType {
    Unknown,
    File,
    URL,
    Asset,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ScalingMode {
    AspectFit,
    AspectFill,
    Fill,
    FitWidth,
    FitHeight,
}

#[derive(Clone, Debug)]
pub struct VideoInfo {
    pub duration: Duration,
    pub width: u32,
    pub height: u32,
    pub frame_rate: f64,
    pub bit_rate: u32,
    pub audio_codec: String,
    pub video_codec: String,
}

impl Default for VideoInfo {
    fn default() -> Self {
        Self {
            duration: Duration::ZERO,
            width: 0,
            height: 0,
            frame_rate: 0.0,
            bit_rate: 0,
            audio_codec: String::new(),
            video_codec: String::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct VideoPlayerDelegate {
    pub on_play: Option<Box<dyn Fn()>>,
    pub on_pause: Option<Box<dyn Fn()>>,
    pub on_stop: Option<Box<dyn Fn()>>,
    pub on_completed: Option<Box<dyn Fn()>>,
    pub on_seek: Option<Box<dyn Fn(Duration)>>,
    pub on_progress: Option<Box<dyn Fn(Duration, f64)>>,
    pub on_buffering: Option<Box<dyn Fn(f64)>>,
    pub on_ready: Option<Box<dyn Fn(&VideoInfo)>>,
    pub on_error: Option<Box<dyn Fn(&str)>>,
}

impl Default for VideoPlayerDelegate {
    fn default() -> Self {
        Self {
            on_play: None,
            on_pause: None,
            on_stop: None,
            on_completed: None,
            on_seek: None,
            on_progress: None,
            on_buffering: None,
            on_ready: None,
            on_error: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct VideoFrame {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub timestamp: Duration,
}

#[derive(Clone, Debug)]
pub struct VideoPlayer {
    source: String,
    source_type: VideoSourceType,
    state: Arc<Mutex<VideoPlayerState>>,
    current_time: Arc<Mutex<Duration>>,
    duration: Arc<Mutex<Duration>>,
    volume: Arc<Mutex<f32>>,
    is_looping: Arc<Mutex<bool>>,
    is_muted: Arc<Mutex<bool>>,
    playback_rate: Arc<Mutex<f32>>,
    scaling_mode: Arc<Mutex<ScalingMode>>,
    info: Arc<Mutex<VideoInfo>>,
    delegate: Rc<RefCell<VideoPlayerDelegate>>,
    frame_buffer: Arc<Mutex<VecDeque<VideoFrame>>>,
    max_buffer_size: usize,
    position: (f32, f32),
    size: (f32, f32),
    visible: bool,
    opacity: u8,
    background_color: Color4B,
    keep_aspect_ratio: bool,
    show_controls: bool,
    current_volume: f32,
}

impl VideoPlayer {
    pub fn new() -> Self {
        Self {
            source: String::new(),
            source_type: VideoSourceType::Unknown,
            state: Arc::new(Mutex::new(VideoPlayerState::Unknown)),
            current_time: Arc::new(Mutex::new(Duration::ZERO)),
            duration: Arc::new(Mutex::new(Duration::ZERO)),
            volume: Arc::new(Mutex::new(1.0)),
            is_looping: Arc::new(Mutex::new(false)),
            is_muted: Arc::new(Mutex::new(false)),
            playback_rate: Arc::new(Mutex::new(1.0)),
            scaling_mode: Arc::new(Mutex::new(ScalingMode::AspectFit)),
            info: Arc::new(Mutex::new(VideoInfo::default())),
            delegate: Rc::new(RefCell::new(VideoPlayerDelegate::default())),
            frame_buffer: Arc::new(Mutex::new(VecDeque::new())),
            max_buffer_size: 30,
            position: (0.0, 0.0),
            size: (320.0, 240.0),
            visible: true,
            opacity: 255,
            background_color: Color4B::new(0, 0, 0, 255),
            keep_aspect_ratio: true,
            show_controls: true,
            current_volume: 1.0,
        }
    }

    pub fn with_file(file: &str) -> Self {
        let mut player = Self::new();
        player.set_file(file);
        player
    }

    pub fn with_url(url: &str) -> Self {
        let mut player = Self::new();
        player.set_url(url);
        player
    }

    pub fn set_file(&mut self, file: &str) {
        self.source = file.to_string();
        self.source_type = VideoSourceType::File;
        self.set_state(VideoPlayerState::Loading);
    }

    pub fn set_url(&mut self, url: &str) {
        self.source = url.to_string();
        self.source_type = VideoSourceType::URL;
        self.set_state(VideoPlayerState::Loading);
    }

    pub fn set_asset(&mut self, asset: &str) {
        self.source = asset.to_string();
        self.source_type = VideoSourceType::Asset;
        self.set_state(VideoPlayerState::Loading);
    }

    pub fn get_source(&self) -> &str {
        &self.source
    }

    pub fn get_source_type(&self) -> VideoSourceType {
        self.source_type
    }

    pub fn play(&mut self) {
        self.set_state(VideoPlayerState::Playing);
        self.notify_play();
    }

    pub fn pause(&mut self) {
        self.set_state(VideoPlayerState::Paused);
        self.notify_pause();
    }

    pub fn stop(&mut self) {
        self.set_state(VideoPlayerState::Stoped);
        *self.current_time.lock().unwrap() = Duration::ZERO;
        self.notify_stop();
    }

    pub fn seek_to(&mut self, time: Duration) {
        let duration = *self.duration.lock().unwrap();
        let time = time.min(duration);
        *self.current_time.lock().unwrap() = time;
        self.notify_seek(time);
    }

    pub fn seek_to_percentage(&mut self, percentage: f64) {
        let duration = *self.duration.lock().unwrap();
        if duration.as_secs_f64() > 0.0 {
            let time = Duration::from_secs_f64(duration.as_secs_f64() * percentage / 100.0);
            self.seek_to(time);
        }
    }

    pub fn step_forward(&mut self, frames: u32) {
        let frame_time = Duration::from_secs_f64(1.0 / 30.0);
        let step = frame_time * frames as u32;
        let current = *self.current_time.lock().unwrap();
        let duration = *self.duration.lock().unwrap();
        let new_time = (current + step).min(duration);
        self.seek_to(new_time);
    }

    pub fn step_backward(&mut self, frames: u32) {
        let frame_time = Duration::from_secs_f64(1.0 / 30.0);
        let step = frame_time * frames as u32;
        let current = *self.current_time.lock().unwrap();
        let new_time = if current >= step { current - step } else { Duration::ZERO };
        self.seek_to(new_time);
    }

    pub fn set_volume(&mut self, volume: f32) {
        let volume = volume.clamp(0.0, 1.0);
        *self.volume.lock().unwrap() = volume;
        self.current_volume = volume;
    }

    pub fn get_volume(&self) -> f32 {
        *self.volume.lock().unwrap()
    }

    pub fn mute(&mut self) {
        *self.is_muted.lock().unwrap() = true;
    }

    pub fn unmute(&mut self) {
        *self.is_muted.lock().unwrap() = false;
    }

    pub fn is_muted(&self) -> bool {
        *self.is_muted.lock().unwrap()
    }

    pub fn toggle_mute(&mut self) {
        let mut muted = self.is_muted.lock().unwrap();
        *muted = !*muted;
    }

    pub fn set_looping(&mut self, looping: bool) {
        *self.is_looping.lock().unwrap() = looping;
    }

    pub fn is_looping(&self) -> bool {
        *self.is_looping.lock().unwrap()
    }

    pub fn set_playback_rate(&mut self, rate: f32) {
        let rate = rate.clamp(0.25, 4.0);
        *self.playback_rate.lock().unwrap() = rate;
    }

    pub fn get_playback_rate(&self) -> f32 {
        *self.playback_rate.lock().unwrap()
    }

    pub fn set_scaling_mode(&mut self, mode: ScalingMode) {
        *self.scaling_mode.lock().unwrap() = mode;
    }

    pub fn get_scaling_mode(&self) -> ScalingMode {
        *self.scaling_mode.lock().unwrap()
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.position = (x, y);
    }

    pub fn get_position(&self) -> (f32, f32) {
        self.position
    }

    pub fn set_size(&mut self, width: f32, height: f32) {
        self.size = (width, height);
    }

    pub fn get_size(&self) -> (f32, f32) {
        self.size
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn set_opacity(&mut self, opacity: u8) {
        self.opacity = opacity;
    }

    pub fn get_opacity(&self) -> u8 {
        self.opacity
    }

    pub fn set_background_color(&mut self, color: Color4B) {
        self.background_color = color;
    }

    pub fn get_background_color(&self) -> Color4B {
        self.background_color
    }

    pub fn set_keep_aspect_ratio(&mut self, keep: bool) {
        self.keep_aspect_ratio = keep;
    }

    pub fn is_keeping_aspect_ratio(&self) -> bool {
        self.keep_aspect_ratio
    }

    pub fn set_show_controls(&mut self, show: bool) {
        self.show_controls = show;
    }

    pub fn is_showing_controls(&self) -> bool {
        self.show_controls
    }

    pub fn get_state(&self) -> VideoPlayerState {
        *self.state.lock().unwrap()
    }

    pub fn get_current_time(&self) -> Duration {
        *self.current_time.lock().unwrap()
    }

    pub fn get_current_time_string(&self) -> String {
        let time = self.get_current_time();
        let secs = time.as_secs() as u64;
        let mins = secs / 60;
        let secs = secs % 60;
        format!("{:02}:{:02}", mins, secs)
    }

    pub fn get_duration(&self) -> Duration {
        *self.duration.lock().unwrap()
    }

    pub fn get_duration_string(&self) -> String {
        let duration = self.get_duration();
        let secs = duration.as_secs() as u64;
        let mins = secs / 60;
        let secs = secs % 60;
        format!("{:02}:{:02}", mins, secs)
    }

    pub fn get_progress(&self) -> f64 {
        let duration = *self.duration.lock().unwrap();
        if duration.as_secs_f64() == 0.0 {
            0.0
        } else {
            self.get_current_time().as_secs_f64() / duration.as_secs_f64() * 100.0
        }
    }

    pub fn get_video_info(&self) -> VideoInfo {
        *self.info.lock().unwrap()
    }

    pub fn get_video_width(&self) -> u32 {
        self.info.lock().unwrap().width
    }

    pub fn get_video_height(&self) -> u32 {
        self.info.lock().unwrap().height
    }

    pub fn get_video_size(&self) -> (u32, u32) {
        let info = self.info.lock().unwrap();
        (info.width, info.height)
    }

    pub fn set_delegate(&mut self, delegate: Rc<RefCell<VideoPlayerDelegate>>) {
        self.delegate = delegate;
    }

    pub fn get_delegate(&self) -> Rc<RefCell<VideoPlayerDelegate>> {
        self.delegate.clone()
    }

    pub fn is_playing(&self) -> bool {
        *self.state.lock().unwrap() == VideoPlayerState::Playing
    }

    pub fn is_paused(&self) -> bool {
        *self.state.lock().unwrap() == VideoPlayerState::Paused
    }

    pub fn is_loaded(&self) -> bool {
        matches!(
            *self.state.lock().unwrap(),
            VideoPlayerState::ReadyToPlay | VideoPlayerState::Playing | VideoPlayerState::Paused
        )
    }

    pub fn add_frame(&mut self, frame: VideoFrame) {
        let mut buffer = self.frame_buffer.lock().unwrap();
        if buffer.len() >= self.max_buffer_size {
            buffer.pop_front();
        }
        buffer.push_back(frame);
    }

    pub fn get_next_frame(&self) -> Option<VideoFrame> {
        self.frame_buffer.lock().unwrap().pop_front()
    }

    pub fn clear_buffer(&mut self) {
        self.frame_buffer.lock().unwrap().clear();
    }

    fn set_state(&mut self, state: VideoPlayerState) {
        *self.state.lock().unwrap() = state;
    }

    fn notify_play(&self) {
        if let Some(cb) = &self.delegate.borrow().on_play {
            cb();
        }
    }

    fn notify_pause(&self) {
        if let Some(cb) = &self.delegate.borrow().on_pause {
            cb();
        }
    }

    fn notify_stop(&self) {
        if let Some(cb) = &self.delegate.borrow().on_stop {
            cb();
        }
    }

    fn notify_completed(&self) {
        if let Some(cb) = &self.delegate.borrow().on_completed {
            cb();
        }
    }

    fn notify_seek(&self, time: Duration) {
        if let Some(cb) = &self.delegate.borrow().on_seek {
            cb(time);
        }
    }

    fn notify_progress(&self, current: Duration, percentage: f64) {
        if let Some(cb) = &self.delegate.borrow().on_progress {
            cb(current, percentage);
        }
    }

    fn notify_buffering(&self, percentage: f64) {
        if let Some(cb) = &self.delegate.borrow().on_buffering {
            cb(percentage);
        }
    }

    fn notify_ready(&self, info: &VideoInfo) {
        if let Some(cb) = &self.delegate.borrow().on_ready {
            cb(info);
        }
    }

    fn notify_error(&self, message: &str) {
        self.set_state(VideoPlayerState::Error(message.to_string()));
        if let Some(cb) = &self.delegate.borrow().on_error {
            cb(message);
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.is_playing() {
            let rate = *self.playback_rate.lock().unwrap();
            let frame_time = Duration::from_secs_f64(delta_time as f64 * rate);
            let mut current = self.current_time.lock().unwrap();
            *current += frame_time;

            let duration = *self.duration.lock().unwrap();
            if *current >= duration {
                if *self.is_looping.lock().unwrap() {
                    *current = Duration::ZERO;
                    self.notify_completed();
                } else {
                    *current = duration;
                    self.set_state(VideoPlayerState::Completed);
                    self.notify_completed();
                }
            }

            let progress = if duration.as_secs_f64() > 0.0 {
                current.as_secs_f64() / duration.as_secs_f64() * 100.0
            } else {
                0.0
            };
            self.notify_progress(*current, progress);
        }
    }

    pub fn fast_forward(&mut self, seconds: f64) {
        let current = *self.current_time.lock().unwrap();
        let duration = *self.duration.lock().unwrap();
        let new_time = (current + Duration::from_secs_f64(seconds)).min(duration);
        self.seek_to(new_time);
    }

    pub fn rewind(&mut self, seconds: f64) {
        let current = *self.current_time.lock().unwrap();
        let new_time = if current.as_secs_f64() >= seconds {
            current - Duration::from_secs_f64(seconds)
        } else {
            Duration::ZERO
        };
        self.seek_to(new_time);
    }

    pub fn skip_intro(&mut self, seconds: f64) {
        self.fast_forward(seconds);
    }

    pub fn replay(&mut self) {
        self.stop();
        self.play();
    }

    pub fn get_formatted_time(&self, time: Duration) -> String {
        let total_secs = time.as_secs() as u64;
        let hours = total_secs / 3600;
        let mins = (total_secs % 3600) / 60;
        let secs = total_secs % 60;

        if hours > 0 {
            format!("{:02}:{:02}:{:02}", hours, mins, secs)
        } else {
            format!("{:02}:{:02}", mins, secs)
        }
    }

    pub fn get_time_remaining(&self) -> Duration {
        let duration = *self.duration.lock().unwrap();
        let current = *self.current_time.lock().unwrap();
        duration - current
    }

    pub fn get_time_remaining_string(&self) -> String {
        self.get_formatted_time(self.get_time_remaining())
    }

    pub fn generate_report(&self) -> String {
        format!(
            "=== VideoPlayer Report ===\n\
             Source: {}\n\
             Source Type: {:?}\n\
             State: {:?}\n\
             Current Time: {}\n\
             Duration: {}\n\
             Progress: {:.1}%\n\
             Volume: {:.1}\n\
             Loop: {}\n\
             Muted: {}\n\
             Playback Rate: {:.1}x\n\
             Video Size: {}x{}\n\
             Is Playing: {}",
            self.source,
            self.source_type,
            self.get_state(),
            self.get_current_time_string(),
            self.get_duration_string(),
            self.get_progress(),
            self.get_volume(),
            self.is_looping(),
            self.is_muted(),
            self.get_playback_rate(),
            self.get_video_width(),
            self.get_video_height(),
            self.is_playing()
        )
    }
}

impl Default for VideoPlayer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_videoplayer_creation() {
        let player = VideoPlayer::new();
        assert!(player.get_source().is_empty());
        assert_eq!(player.get_state(), VideoPlayerState::Unknown);
        assert!(player.is_visible());
        assert_eq!(player.get_volume(), 1.0);
    }

    #[test]
    fn test_videoplayer_file() {
        let mut player = VideoPlayer::new();
        player.set_file("test.mp4");
        assert_eq!(player.get_source(), "test.mp4");
        assert_eq!(player.get_source_type(), VideoSourceType::File);
        assert_eq!(player.get_state(), VideoPlayerState::Loading);
    }

    #[test]
    fn test_videoplayer_url() {
        let mut player = VideoPlayer::new();
        player.set_url("https://example.com/video.mp4");
        assert_eq!(player.get_source(), "https://example.com/video.mp4");
        assert_eq!(player.get_source_type(), VideoSourceType::URL);
    }

    #[test]
    fn test_videoplayer_volume() {
        let mut player = VideoPlayer::new();
        player.set_volume(0.5);
        assert_eq!(player.get_volume(), 0.5);

        player.set_volume(1.5);
        assert_eq!(player.get_volume(), 1.0);
    }

    #[test]
    fn test_videoplayer_mute() {
        let mut player = VideoPlayer::new();
        assert!(!player.is_muted());

        player.mute();
        assert!(player.is_muted());

        player.unmute();
        assert!(!player.is_muted());

        player.toggle_mute();
        assert!(player.is_muted());
    }

    #[test]
    fn test_videoplayer_looping() {
        let mut player = VideoPlayer::new();
        assert!(!player.is_looping());

        player.set_looping(true);
        assert!(player.is_looping());
    }

    #[test]
    fn test_videoplayer_playback_rate() {
        let mut player = VideoPlayer::new();
        assert_eq!(player.get_playback_rate(), 1.0);

        player.set_playback_rate(2.0);
        assert_eq!(player.get_playback_rate(), 2.0);

        player.set_playback_rate(10.0);
        assert_eq!(player.get_playback_rate(), 4.0);
    }

    #[test]
    fn test_videoplayer_scaling() {
        let mut player = VideoPlayer::new();
        player.set_scaling_mode(ScalingMode::Fill);
        assert_eq!(player.get_scaling_mode(), ScalingMode::Fill);
    }

    #[test]
    fn test_videoplayer_position() {
        let mut player = VideoPlayer::new();
        player.set_position(100.0, 200.0);
        assert_eq!(player.get_position(), (100.0, 200.0));
    }

    #[test]
    fn test_videoplayer_size() {
        let mut player = VideoPlayer::new();
        player.set_size(640.0, 480.0);
        assert_eq!(player.get_size(), (640.0, 480.0));
    }

    #[test]
    fn test_videoplayer_visibility() {
        let mut player = VideoPlayer::new();
        player.set_visible(false);
        assert!(!player.is_visible());

        player.set_visible(true);
        assert!(player.is_visible());
    }

    #[test]
    fn test_videoplayer_opacity() {
        let mut player = VideoPlayer::new();
        player.set_opacity(128);
        assert_eq!(player.get_opacity(), 128);
    }

    #[test]
    fn test_videoplayer_time_format() {
        let player = VideoPlayer::new();

        let time = Duration::from_secs(65);
        assert_eq!(player.get_formatted_time(time), "01:05");

        let time = Duration::from_secs(3665);
        assert_eq!(player.get_formatted_time(time), "01:01:05");
    }

    #[test]
    fn test_videoplayer_report() {
        let mut player = VideoPlayer::new();
        player.set_file("test.mp4");
        let report = player.generate_report();
        assert!(report.contains("VideoPlayer Report"));
        assert!(report.contains("test.mp4"));
    }

    #[test]
    fn test_videoplayer_frame_buffer() {
        let mut player = VideoPlayer::new();

        let frame = VideoFrame {
            data: vec![0u8; 100],
            width: 640,
            height: 480,
            timestamp: Duration::from_millis(33),
        };

        player.add_frame(frame.clone());

        let retrieved = player.get_next_frame();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().width, 640);
    }
}

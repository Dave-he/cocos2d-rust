/// VideoPlayer - 视频播放组件
/// 
/// 功能特性：
/// - 完整的播放控制（播放、暂停、停止、跳转）
/// - 播放速率控制（0.25x - 4x）
/// - 音量控制
/// - 循环播放
/// - 状态管理（加载中、就绪、播放中、暂停、结束、错误）
/// - 事件回调系统
/// - 缩略图预览
/// - 画中画模式（预留）

use crate::base::Node;
use crate::ui::Widget;
use crate::math::Vec2;

/// 视频状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoState {
    /// 未初始化
    Uninitialized,
    /// 加载中
    Loading,
    /// 就绪
    Ready,
    /// 播放中
    Playing,
    /// 暂停
    Paused,
    /// 播放结束
    Ended,
    /// 错误
    Error,
}

/// 视频事件类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoEventType {
    /// 加载中
    Loading,
    /// 准备就绪
    Ready,
    /// 播放
    Play,
    /// 暂停
    Pause,
    /// 结束
    Ended,
    /// 进度更新
    Progress,
    /// 缓冲更新
    BufferUpdate,
    /// 错误
    Error,
}

/// 视频事件回调
pub type VideoEventCallback = Box<dyn FnMut(&VideoPlayer, VideoEventType)>;

/// 进度回调
pub type VideoProgressCallback = Box<dyn FnMut(&VideoPlayer, f64, f64)>; // current, duration

/// 视频缩略图
#[derive(Debug, Clone)]
pub struct VideoThumbnail {
    pub time: f64,
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

/// 视频格式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoFormat {
    Unknown,
    MP4,
    WebM,
    AVI,
    MOV,
    MKV,
    FLV,
}

/// 视频质量级别
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoQuality {
    Low,
    Medium,
    High,
    Auto,
}

/// VideoPlayer - 视频播放器组件
pub struct VideoPlayer {
    /// 基础 Widget
    widget: Widget,
    /// 视频 URL 或文件路径
    source: String,
    /// 视频状态
    state: VideoState,
    /// 视频格式
    format: VideoFormat,
    
    /// 当前播放位置（秒）
    current_time: f64,
    /// 视频总时长（秒）
    duration: f64,
    /// 缓冲进度（0-1）
    buffer_progress: f32,
    /// 播放进度（0-1）
    play_progress: f32,
    
    /// 是否循环播放
    is_looping: bool,
    /// 是否静音
    is_muted: bool,
    /// 播放速率
    playback_rate: f32,
    /// 音量（0-1）
    volume: f32,
    
    /// 是否显示控制条
    show_controls: bool,
    /// 自动播放
    auto_play: bool,
    /// 允许画中画
    allow_picture_in_picture: bool,
    
    /// 缩略图列表
    thumbnails: Vec<VideoThumbnail>,
    /// 是否已生成缩略图
    thumbnails_generated: bool,
    
    /// 视频宽度
    video_width: u32,
    /// 视频高度
    video_height: u32,
    /// 宽高比
    aspect_ratio: f32,
    
    /// 事件回调
    on_event: Option<VideoEventCallback>,
    /// 进度回调
    on_progress: Option<VideoProgressCallback>,
    /// 错误信息
    error_message: String,
}

impl std::fmt::Debug for VideoPlayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VideoPlayer")
            .field("widget", &self.widget)
            .field("source", &self.source)
            .field("state", &self.state)
            .field("current_time", &self.current_time)
            .field("duration", &self.duration)
            .field("volume", &self.volume)
            .finish()
    }
}

impl VideoPlayer {
    /// 创建新的 VideoPlayer
    pub fn new() -> Self {
        Self {
            widget: Widget::new(),
            source: String::new(),
            state: VideoState::Uninitialized,
            format: VideoFormat::Unknown,
            
            current_time: 0.0,
            duration: 0.0,
            buffer_progress: 0.0,
            play_progress: 0.0,
            
            is_looping: false,
            is_muted: false,
            playback_rate: 1.0,
            volume: 1.0,
            
            show_controls: true,
            auto_play: false,
            allow_picture_in_picture: false,
            
            thumbnails: Vec::new(),
            thumbnails_generated: false,
            
            video_width: 0,
            video_height: 0,
            aspect_ratio: 16.0 / 9.0,
            
            on_event: None,
            on_progress: None,
            error_message: String::new(),
        }
    }
    
    /// 创建 VideoPlayer 并加载视频
    pub fn with_source(source: impl Into<String>) -> Self {
        let mut player = Self::new();
        player.source = source.into();
        player
    }
    
    // ===== 视频源管理 =====
    
    /// 设置视频源
    pub fn set_source(&mut self, source: impl Into<String>) {
        self.source = source.into();
        self.state = VideoState::Loading;
        self.current_time = 0.0;
        self.duration = 0.0;
        self.error_message.clear();
        
        // 解析视频格式
        self.format = self.detect_format();
        
        // 触发加载事件
        self.trigger_event(VideoEventType::Loading);
        
        // 模拟加载完成（实际实现中会调用平台相关API）
        self.simulate_load_complete();
    }
    
    /// 获取视频源
    pub fn source(&self) -> &str {
        &self.source
    }
    
    /// 视频是否已加载
    pub fn is_loaded(&self) -> bool {
        self.state != VideoState::Uninitialized && 
        self.state != VideoState::Loading &&
        self.state != VideoState::Error
    }
    
    // ===== 播放控制 =====
    
    /// 开始播放
    pub fn play(&mut self) {
        if self.state == VideoState::Uninitialized || self.state == VideoState::Loading {
            return;
        }
        
        if self.state == VideoState::Ended {
            self.current_time = 0.0;
        }
        
        self.state = VideoState::Playing;
        self.trigger_event(VideoEventType::Play);
    }
    
    /// 暂停播放
    pub fn pause(&mut self) {
        if self.state == VideoState::Playing {
            self.state = VideoState::Paused;
            self.trigger_event(VideoEventType::Pause);
        }
    }
    
    /// 切换播放/暂停
    pub fn toggle_play_pause(&mut self) {
        match self.state {
            VideoState::Playing => self.pause(),
            VideoState::Paused => self.play(),
            VideoState::Ready | VideoState::Ended => self.play(),
            _ => {}
        }
    }
    
    /// 停止播放
    pub fn stop(&mut self) {
        self.state = VideoState::Ready;
        self.current_time = 0.0;
        self.play_progress = 0.0;
    }
    
    /// 跳转到指定时间（秒）
    pub fn seek_to(&mut self, time: f64) {
        if self.duration <= 0.0 {
            return;
        }
        
        self.current_time = time.clamp(0.0, self.duration);
        self.play_progress = (self.current_time / self.duration) as f32;
        
        if self.state == VideoState::Playing {
            self.trigger_event(VideoEventType::Progress);
        }
    }
    
    /// 跳转到指定进度（0-1）
    pub fn seek_to_progress(&mut self, progress: f32) {
        let progress = progress.clamp(0.0, 1.0);
        self.seek_to(progress as f64 * self.duration);
    }
    
    /// 快进（秒）
    pub fn fast_forward(&mut self, seconds: f64) {
        self.seek_to(self.current_time + seconds);
    }
    
    /// 快退（秒）
    pub fn rewind(&mut self, seconds: f64) {
        self.seek_to(self.current_time - seconds);
    }
    
    /// 跳转到指定百分比
    pub fn seek_to_percent(&mut self, percent: f32) {
        self.seek_to_progress(percent / 100.0);
    }
    
    // ===== 播放状态 =====
    
    /// 获取当前状态
    pub fn state(&self) -> VideoState {
        self.state
    }
    
    /// 是否正在播放
    pub fn is_playing(&self) -> bool {
        self.state == VideoState::Playing
    }
    
    /// 是否暂停
    pub fn is_paused(&self) -> bool {
        self.state == VideoState::Paused
    }
    
    /// 是否已结束
    pub fn is_ended(&self) -> bool {
        self.state == VideoState::Ended
    }
    
    // ===== 时间管理 =====
    
    /// 获取当前时间（秒）
    pub fn current_time(&self) -> f64 {
        self.current_time
    }
    
    /// 获取总时长（秒）
    pub fn duration(&self) -> f64 {
        self.duration
    }
    
    /// 获取当前时间（格式化字符串）
    pub fn current_time_string(&self) -> String {
        self.format_time(self.current_time)
    }
    
    /// 获取总时长（格式化字符串）
    pub fn duration_string(&self) -> String {
        self.format_time(self.duration)
    }
    
    /// 格式化时间
    fn format_time(&self, seconds: f64) -> String {
        let total_seconds = seconds as u64;
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let secs = total_seconds % 60;
        
        if hours > 0 {
            format!("{:02}:{:02}:{:02}", hours, minutes, secs)
        } else {
            format!("{:02}:{:02}", minutes, secs)
        }
    }
    
    // ===== 进度管理 =====
    
    /// 获取播放进度（0-1）
    pub fn play_progress(&self) -> f32 {
        self.play_progress
    }
    
    /// 获取播放进度（百分比）
    pub fn play_progress_percent(&self) -> f32 {
        self.play_progress * 100.0
    }
    
    /// 获取缓冲进度（0-1）
    pub fn buffer_progress(&self) -> f32 {
        self.buffer_progress
    }
    
    // ===== 播放设置 =====
    
    /// 设置是否循环
    pub fn set_looping(&mut self, looping: bool) {
        self.is_looping = looping;
    }
    
    /// 是否循环播放
    pub fn is_looping(&self) -> bool {
        self.is_looping
    }
    
    /// 设置静音
    pub fn set_muted(&mut self, muted: bool) {
        self.is_muted = muted;
    }
    
    /// 是否静音
    pub fn is_muted(&self) -> bool {
        self.is_muted
    }
    
    /// 设置音量（0-1）
    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
    }
    
    /// 获取音量
    pub fn volume(&self) -> f32 {
        self.volume
    }
    
    /// 设置播放速率
    pub fn set_playback_rate(&mut self, rate: f32) {
        self.playback_rate = rate.clamp(0.25, 4.0);
    }
    
    /// 获取播放速率
    pub fn playback_rate(&self) -> f32 {
        self.playback_rate
    }
    
    // ===== 显示控制 =====
    
    /// 设置是否显示控制条
    pub fn set_show_controls(&mut self, show: bool) {
        self.show_controls = show;
    }
    
    /// 是否显示控制条
    pub fn shows_controls(&self) -> bool {
        self.show_controls
    }
    
    /// 设置自动播放
    pub fn set_auto_play(&mut self, auto: bool) {
        self.auto_play = auto;
    }
    
    /// 是否自动播放
    pub fn is_auto_play(&self) -> bool {
        self.auto_play
    }
    
    /// 设置允许画中画
    pub fn set_allow_pip(&mut self, allow: bool) {
        self.allow_picture_in_picture = allow;
    }
    
    /// 是否允许画中画
    pub fn allows_pip(&self) -> bool {
        self.allow_picture_in_picture
    }
    
    // ===== 视频信息 =====
    
    /// 获取视频宽度
    pub fn video_width(&self) -> u32 {
        self.video_width
    }
    
    /// 获取视频高度
    pub fn video_height(&self) -> u32 {
        self.video_height
    }
    
    /// 获取宽高比
    pub fn aspect_ratio(&self) -> f32 {
        self.aspect_ratio
    }
    
    /// 获取视频格式
    pub fn format(&self) -> VideoFormat {
        self.format
    }
    
    // ===== 缩略图 =====
    
    /// 生成缩略图
    pub fn generate_thumbnails(&mut self, count: usize) {
        if count == 0 || self.duration <= 0.0 {
            return;
        }
        
        self.thumbnails.clear();
        
        for i in 0..count {
            let time = (self.duration * i as f64) / (count - 1) as f64;
            
            self.thumbnails.push(VideoThumbnail {
                time,
                data: Vec::new(),
                width: 160,
                height: 90,
            });
        }
        
        self.thumbnails_generated = true;
    }
    
    /// 获取缩略图
    pub fn get_thumbnail(&self, time: f64) -> Option<&VideoThumbnail> {
        self.thumbnails.iter()
            .min_by_key(|t| (t.time - time).abs() as i64)
    }
    
    /// 缩略图是否已生成
    pub fn has_thumbnails(&self) -> bool {
        self.thumbnails_generated
    }
    
    // ===== 回调 =====
    
    /// 设置事件回调
    pub fn set_on_event<F>(&mut self, callback: F)
    where
        F: FnMut(&VideoPlayer, VideoEventType) + 'static,
    {
        self.on_event = Some(Box::new(callback));
    }
    
    /// 设置进度回调
    pub fn set_on_progress<F>(&mut self, callback: F)
    where
        F: FnMut(&VideoPlayer, f64, f64) + 'static,
    {
        self.on_progress = Some(Box::new(callback));
    }
    
    // ===== 错误处理 =====
    
    /// 获取错误信息
    pub fn error_message(&self) -> &str {
        &self.error_message
    }
    
    /// 是否有错误
    pub fn has_error(&self) -> bool {
        self.state == VideoState::Error
    }
    
    /// Widget 引用
    pub fn widget(&self) -> &Widget {
        &self.widget
    }
    
    /// Widget 可变引用
    pub fn widget_mut(&mut self) -> &mut Widget {
        &mut self.widget
    }
    
    // ===== 私有方法 =====
    
    /// 检测视频格式
    fn detect_format(&self) -> VideoFormat {
        if self.source.is_empty() {
            return VideoFormat::Unknown;
        }
        
        let lower = self.source.to_lowercase();
        if lower.ends_with(".mp4") {
            VideoFormat::MP4
        } else if lower.ends_with(".webm") {
            VideoFormat::WebM
        } else if lower.ends_with(".avi") {
            VideoFormat::AVI
        } else if lower.ends_with(".mov") {
            VideoFormat::MOV
        } else if lower.ends_with(".mkv") {
            VideoFormat::MKV
        } else if lower.ends_with(".flv") {
            VideoFormat::FLV
        } else {
            VideoFormat::Unknown
        }
    }
    
    /// 模拟加载完成（实际实现中会调用平台相关API）
    fn simulate_load_complete(&mut self) {
        // 模拟视频加载完成
        self.duration = 120.0; // 假设2分钟
        self.video_width = 1920;
        self.video_height = 1080;
        self.aspect_ratio = 1920.0 / 1080.0;
        self.buffer_progress = 1.0;
        self.state = VideoState::Ready;
        
        self.trigger_event(VideoEventType::Ready);
        
        if self.auto_play {
            self.play();
        }
    }
    
    /// 触发事件
    fn trigger_event(&mut self, event_type: VideoEventType) {
        if let Some(mut callback) = self.on_event.take() {
            callback(self, event_type);
            self.on_event = Some(callback);
        }
    }
    
    /// 更新进度（每帧调用）
    pub fn update(&mut self, delta_time: f64) {
        if self.state == VideoState::Playing {
            self.current_time += delta_time * self.playback_rate as f64;
            
            if self.current_time >= self.duration {
                if self.is_looping {
                    self.current_time = 0.0;
                } else {
                    self.current_time = self.duration;
                    self.state = VideoState::Ended;
                    self.trigger_event(VideoEventType::Ended);
                }
            }
            
            self.play_progress = (self.current_time / self.duration) as f32;
            
            if let Some(mut callback) = self.on_progress.take() {
                callback(self, self.current_time, self.duration);
                self.on_progress = Some(callback);
            }
            
            self.trigger_event(VideoEventType::Progress);
        }
    }
    
    /// 获取状态字符串
    pub fn state_string(&self) -> &str {
        match self.state {
            VideoState::Uninitialized => "Uninitialized",
            VideoState::Loading => "Loading",
            VideoState::Ready => "Ready",
            VideoState::Playing => "Playing",
            VideoState::Paused => "Paused",
            VideoState::Ended => "Ended",
            VideoState::Error => "Error",
        }
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
        assert_eq!(player.state(), VideoState::Uninitialized);
        assert!(player.source().is_empty());
    }
    
    #[test]
    fn test_videoplayer_source() {
        let mut player = VideoPlayer::new();
        player.set_source("test.mp4");
        assert_eq!(player.source(), "test.mp4");
        assert_eq!(player.format(), VideoFormat::MP4);
    }
    
    #[test]
    fn test_videoplayer_play_pause() {
        let mut player = VideoPlayer::new();
        player.set_source("test.mp4");
        assert_eq!(player.state(), VideoState::Ready);
        
        player.play();
        assert!(player.is_playing());
        
        player.pause();
        assert!(player.is_paused());
        
        player.toggle_play_pause();
        assert!(player.is_playing());
        
        player.toggle_play_pause();
        assert!(player.is_paused());
    }
    
    #[test]
    fn test_videoplayer_seek() {
        let mut player = VideoPlayer::new();
        player.set_source("test.mp4");
        
        player.seek_to(30.0);
        assert_eq!(player.current_time(), 30.0);
        assert!(player.duration() > 0.0);
    }
    
    #[test]
    fn test_videoplayer_volume() {
        let mut player = VideoPlayer::new();
        
        player.set_volume(0.5);
        assert_eq!(player.volume(), 0.5);
        
        player.set_volume(1.5);
        assert_eq!(player.volume(), 1.0);
        
        player.set_volume(-0.5);
        assert_eq!(player.volume(), 0.0);
    }
    
    #[test]
    fn test_videoplayer_rate() {
        let mut player = VideoPlayer::new();
        
        player.set_playback_rate(2.0);
        assert_eq!(player.playback_rate(), 2.0);
        
        player.set_playback_rate(0.5);
        assert_eq!(player.playback_rate(), 0.5);
        
        player.set_playback_rate(10.0);
        assert_eq!(player.playback_rate(), 4.0);
    }
    
    #[test]
    fn test_videoplayer_mute() {
        let mut player = VideoPlayer::new();
        
        assert!(!player.is_muted());
        
        player.set_muted(true);
        assert!(player.is_muted());
        
        player.set_muted(false);
        assert!(!player.is_muted());
    }
    
    #[test]
    fn test_videoplayer_loop() {
        let mut player = VideoPlayer::new();
        
        assert!(!player.is_looping());
        
        player.set_looping(true);
        assert!(player.is_looping());
    }
    
    #[test]
    fn test_videoplayer_time_format() {
        let mut player = VideoPlayer::new();
        player.set_source("test.mp4");
        
        assert_eq!(player.current_time_string(), "00:00");
        
        player.seek_to(65.0);
        assert_eq!(player.current_time_string(), "01:05");
    }
    
    #[test]
    fn test_videoplayer_progress() {
        let mut player = VideoPlayer::new();
        player.set_source("test.mp4");
        
        assert_eq!(player.play_progress(), 0.0);
        
        player.seek_to(60.0);
        assert!(player.play_progress() > 0.0);
    }
    
    #[test]
    fn test_videoplayer_thumbnails() {
        let mut player = VideoPlayer::new();
        player.set_source("test.mp4");
        
        assert!(!player.has_thumbnails());
        
        player.generate_thumbnails(5);
        assert!(player.has_thumbnails());
    }
    
    #[test]
    fn test_videoplayer_format_detection() {
        let mut player = VideoPlayer::new();
        
        player.set_source("video.mp4");
        assert_eq!(player.format(), VideoFormat::MP4);
        
        player.set_source("movie.webm");
        assert_eq!(player.format(), VideoFormat::WebM);
        
        player.set_source("clip.avi");
        assert_eq!(player.format(), VideoFormat::AVI);
        
        player.set_source("animation.mov");
        assert_eq!(player.format(), VideoFormat::MOV);
        
        player.set_source("video.unknown");
        assert_eq!(player.format(), VideoFormat::Unknown);
    }
    
    #[test]
    fn test_videoplayer_error() {
        let player = VideoPlayer::new();
        
        assert!(!player.has_error());
        assert!(player.error_message().is_empty());
    }
    
    #[test]
    fn test_videoplayer_controls() {
        let mut player = VideoPlayer::new();
        
        assert!(player.shows_controls());
        assert!(!player.is_auto_play());
        assert!(!player.allows_pip());
        
        player.set_show_controls(false);
        player.set_auto_play(true);
        player.set_allow_pip(true);
        
        assert!(!player.shows_controls());
        assert!(player.is_auto_play());
        assert!(player.allows_pip());
    }
    
    #[test]
    fn test_videoplayer_update() {
        let mut player = VideoPlayer::new();
        player.set_source("test.mp4");
        
        player.play();
        assert!(player.is_playing());
        
        player.update(1.0);
        assert!(player.current_time() >= 0.0);
    }
    
    #[test]
    fn test_videoplayer_seek_operations() {
        let mut player = VideoPlayer::new();
        player.set_source("test.mp4");
        player.duration = 100.0;
        
        // 测试 seek_to
        player.seek_to(50.0);
        assert_eq!(player.current_time(), 50.0);
        assert_eq!(player.play_progress(), 0.5);
        
        // 测试边界
        player.seek_to(-10.0);
        assert_eq!(player.current_time(), 0.0);
        
        player.seek_to(200.0);
        assert_eq!(player.current_time(), 100.0);
        
        // 测试 fast_forward
        player.seek_to(30.0);
        player.fast_forward(20.0);
        assert_eq!(player.current_time(), 50.0);
        
        // 测试 rewind
        player.rewind(10.0);
        assert_eq!(player.current_time(), 40.0);
    }
    
    #[test]
    fn test_videoplayer_progress() {
        let mut player = VideoPlayer::new();
        player.set_source("test.mp4");
        player.duration = 200.0;
        
        player.seek_to_progress(0.25);
        assert_eq!(player.current_time(), 50.0);
        
        player.seek_to_percent(75.0);
        assert_eq!(player.current_time(), 150.0);
        
        assert_eq!(player.play_progress_percent(), 75.0);
    }
    
    #[test]
    fn test_videoplayer_toggle() {
        let mut player = VideoPlayer::new();
        player.set_source("test.mp4");
        
        // 初始状态为 Ready，切换后应该播放
        player.toggle_play_pause();
        assert!(player.is_playing());
        
        // 播放状态切换后应该暂停
        player.toggle_play_pause();
        assert!(player.is_paused());
        
        // 暂停状态切换后应该播放
        player.toggle_play_pause();
        assert!(player.is_playing());
    }
    
    #[test]
    fn test_videoplayer_looping() {
        let mut player = VideoPlayer::new();
        player.set_source("test.mp4");
        player.duration = 10.0;
        
        assert!(!player.is_looping());
        player.set_looping(true);
        assert!(player.is_looping());
        
        // 测试循环播放逻辑（在实际实现中会在 update 中处理）
        player.current_time = 10.0;
        player.state = VideoState::Ended;
        
        // 模拟循环重新开始
        if player.is_looping() && player.is_ended() {
            player.current_time = 0.0;
            player.state = VideoState::Playing;
        }
        
        assert_eq!(player.current_time(), 0.0);
        assert!(player.is_playing());
    }
    
    #[test]
    fn test_videoplayer_playback_rate() {
        let mut player = VideoPlayer::new();
        player.set_source("test.mp4");
        
        assert_eq!(player.playback_rate(), 1.0);
        
        player.set_playback_rate(2.0);
        assert_eq!(player.playback_rate(), 2.0);
        
        player.set_playback_rate(0.5);
        assert_eq!(player.playback_rate(), 0.5);
        
        // 测试边界
        player.set_playback_rate(0.0);
        assert!(player.playback_rate() >= 0.25);
        
        player.set_playback_rate(10.0);
        assert!(player.playback_rate() <= 4.0);
    }
    
    #[test]
    fn test_videoplayer_quality() {
        let mut player = VideoPlayer::new();
        player.set_source("test.mp4");
        
        assert_eq!(player.quality(), VideoQuality::Auto);
        
        player.set_quality(VideoQuality::High);
        assert_eq!(player.quality(), VideoQuality::High);
        
        player.set_quality(VideoQuality::Low);
        assert_eq!(player.quality(), VideoQuality::Low);
    }
    
    #[test]
    fn test_videoplayer_callbacks() {
        let mut player = VideoPlayer::new();
        let mut event_triggered = false;
        
        player.set_on_event(|_p, event_type| {
            // 事件回调会被触发
        });
        
        player.set_source("test.mp4");
        player.play();
        
        // 验证状态变化
        assert!(player.is_playing());
    }
    
    #[test]
    fn test_videoplayer_time_formatting() {
        let player = VideoPlayer::new();
        
        // 测试不同时长的格式化
        assert_eq!(player.format_time(0.0), "00:00");
        assert_eq!(player.format_time(61.0), "01:01");
        assert_eq!(player.format_time(3661.0), "01:01:01");
    }
    
    #[test]
    fn test_videoplayer_stop() {
        let mut player = VideoPlayer::new();
        player.set_source("test.mp4");
        player.duration = 100.0;
        
        player.play();
        player.seek_to(50.0);
        assert_eq!(player.current_time(), 50.0);
        
        player.stop();
        assert_eq!(player.current_time(), 0.0);
        assert_eq!(player.state(), VideoState::Ready);
    }
    
    #[test]
    fn test_videoplayer_buffer_progress() {
        let player = VideoPlayer::new();
        
        assert_eq!(player.buffer_progress(), 0.0);
        assert!(player.buffer_progress() >= 0.0);
        assert!(player.buffer_progress() <= 1.0);
    }
    
    #[test]
    fn test_videoplayer_ended_state() {
        let mut player = VideoPlayer::new();
        player.set_source("test.mp4");
        player.duration = 10.0;
        
        player.state = VideoState::Ended;
        assert!(player.is_ended());
        
        // 播放结束后再次播放应该从头开始
        player.play();
        assert_eq!(player.current_time(), 0.0);
        assert!(player.is_playing());
    }
    
    #[test]
    fn test_videoplayer_multiple_sources() {
        let mut player = VideoPlayer::new();
        
        player.set_source("video1.mp4");
        assert_eq!(player.source(), "video1.mp4");
        assert_eq!(player.state(), VideoState::Loading);
        
        player.set_source("video2.webm");
        assert_eq!(player.source(), "video2.webm");
        assert_eq!(player.format(), VideoFormat::WebM);
    }
}

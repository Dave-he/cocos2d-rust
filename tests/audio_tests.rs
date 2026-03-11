use cocos2d_rust::audio::AudioEngine;

#[test]
fn test_audio_engine_creation() {
    // AudioEngine::new() 在测试中可能因为音频设备不可用而 panic
    // 这里我们只测试静态方法
    // 注意：AudioEngine::new() 需要系统有可用的音频输出设备
}

#[test]
fn test_audio_preload() {
    // 测试预加载（无返回值）
    AudioEngine::preload("sound.wav");
}

#[test]
fn test_audio_play2d_missing_file() {
    // 测试播放不存在的文件，应该返回 -1
    let id = AudioEngine::play2d("nonexistent_sound.wav", false, 1.0);
    assert_eq!(id, -1, "不存在的文件应该返回 -1");
}

#[test]
fn test_audio_stop_all() {
    // 测试停止所有音频（安全方法）
    AudioEngine::stop_all();
}

#[test]
fn test_audio_global_volume() {
    // 测试全局音量
    AudioEngine::set_global_volume(0.5);
    let vol = AudioEngine::get_volume();
    assert!((vol - 0.5).abs() < 0.01, "全局音量应为 0.5");
    
    // 恢复默认
    AudioEngine::set_global_volume(1.0);
}

#[test]
fn test_audio_mute() {
    // 测试静音
    AudioEngine::set_mute(true);
    assert!(AudioEngine::is_mute(), "应该处于静音状态");
    
    // 取消静音
    AudioEngine::set_mute(false);
    assert!(!AudioEngine::is_mute(), "应该取消静音");
}

#[test]
fn test_audio_max_sources() {
    let max = AudioEngine::get_max_audio_sources();
    assert!(max > 0, "最大音频源数量应大于 0");
}

#[test]
fn test_audio_is_playing_nonexistent() {
    // 不存在的音频 ID 不应该处于播放状态
    let playing = AudioEngine::is_playing(-999);
    assert!(!playing, "不存在的音频不应该处于播放状态");
}

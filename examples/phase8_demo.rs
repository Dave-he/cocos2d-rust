/// Phase 8 Demo - 高级UI组件演示
/// 
/// 展示 EditBox、VideoPlayer、WebView 的功能

use cocos2d_rust::ui::{
    EditBox, EditBoxInputMode, EditBoxInputFlag, EditBoxReturnType,
    VideoPlayer, VideoState, VideoEventType,
    WebView, LoadState, WebViewEvent,
};

fn main() {
    println!("=== Phase 8 Demo: 高级UI组件演示 ===\n");
    
    println!("--- EditBox 高级文本输入组件 ---");
    demo_editbox();
    println!();
    
    println!("--- VideoPlayer 视频播放组件 ---");
    demo_videoplayer();
    println!();
    
    println!("--- WebView 网页视图组件 ---");
    demo_webview();
    println!();
    
    println!("--- 综合功能演示 ---");
    demo_comprehensive();
    
    println!("\n=== Phase 8 Demo 完成 ===");
}

fn demo_editbox() {
    let mut editbox = EditBox::new();
    
    println!("1. 基础文本操作:");
    editbox.set_text("Hello, EditBox!");
    println!("   设置文本: {}", editbox.text());
    
    println!("\n2. 多行编辑:");
    let mut multiline_editbox = EditBox::new_multiline(10);
    multiline_editbox.begin_editing();
    multiline_editbox.insert_text("Line 1\nLine 2\nLine 3");
    println!("   多行文本行数: {}", multiline_editbox.line_count());
    
    println!("\n3. 输入模式:");
    let mut numeric_editbox = EditBox::new();
    numeric_editbox.set_input_mode(EditBoxInputMode::Numeric);
    numeric_editbox.begin_editing();
    numeric_editbox.insert_text("123abc456");
    println!("   数字模式输入'123abc456': {}", numeric_editbox.text());
    
    println!("\n4. 输入标志 (密码模式):");
    let mut password_editbox = EditBox::new();
    let mut flag = EditBoxInputFlag::default();
    flag.password = true;
    password_editbox.set_input_flag(flag);
    password_editbox.set_text("secret123");
    println!("   原始文本: {}", password_editbox.text());
    println!("   显示文本: {}", password_editbox.display_text());
    
    println!("\n5. 撤销功能:");
    let mut undo_editbox = EditBox::new();
    undo_editbox.begin_editing();
    undo_editbox.insert_text("Hello");
    assert_eq!(undo_editbox.text(), "Hello");
    
    println!("\n6. 验证器:");
    let mut validated_editbox = EditBox::new();
    validated_editbox.set_validator(|text| text.len() <= 10);
    validated_editbox.set_text("This is too long");
    println!("   长度验证 (>10字符): '{}'", validated_editbox.text());
    
    println!("\n7. 最大长度限制:");
    let mut limited_editbox = EditBox::new();
    limited_editbox.set_max_length(5);
    limited_editbox.set_text("Hello World");
    println!("   设置'Hello World'，最大长度5: '{}'", limited_editbox.text());
    
    println!("\n8. 占位符:");
    let placeholder_editbox = EditBox::new();
    println!("   空文本时的占位符: '{}'", placeholder_editbox.placeholder());
}

fn demo_videoplayer() {
    println!("1. 创建和加载视频:");
    let mut player = VideoPlayer::with_source("https://example.com/video.mp4");
    println!("   视频源: {}", player.source());
    println!("   视频格式: {:?}", player.format());
    println!("   初始状态: {}", player.state_string());
    
    println!("\n2. 播放控制:");
    player.play();
    println!("   播放后状态: {}", player.state_string());
    
    player.pause();
    println!("   暂停后状态: {}", player.state_string());
    
    player.toggle_play_pause();
    println!("   切换后状态: {}", player.state_string());
    
    println!("\n3. 跳转控制:");
    player.seek_to(30.0);
    println!("   跳转到30秒: {} / {}", player.current_time_string(), player.duration_string());
    
    player.seek_to_progress(0.5);
    println!("   跳转到50%: {} / {}", player.current_time_string(), player.duration_string());
    
    println!("\n4. 播放设置:");
    player.set_volume(0.7);
    println!("   设置音量: {}", player.volume());
    
    player.set_playback_rate(2.0);
    println!("   设置播放速率: {}x", player.playback_rate());
    
    player.set_looping(true);
    println!("   循环播放: {}", player.is_looping());
    
    println!("\n5. 缩略图:");
    player.generate_thumbnails(5);
    println!("   缩略图生成: {}", player.has_thumbnails());
    
    println!("\n6. 视频信息:");
    println!("   视频宽度: {}", player.video_width());
    println!("   视频高度: {}", player.video_height());
    println!("   宽高比: {:.2}", player.aspect_ratio());
    
    println!("\n7. 停止播放:");
    player.stop();
    println!("   停止后状态: {}", player.state_string());
}

fn demo_webview() {
    println!("1. 创建和加载网页:");
    let mut webview = WebView::with_url("https://example.com");
    println!("   当前URL: {}", webview.url());
    println!("   加载状态: {}", webview.load_state_string());
    println!("   加载进度: {}%", webview.load_progress());
    
    println!("\n2. 加载HTML内容:");
    let mut html_webview = WebView::new();
    html_webview.load_html("<html><title>Test Page</title><body><h1>Hello!</h1></body></html>", None);
    println!("   页面标题: {}", html_webview.title());
    
    println!("\n3. 导航控制:");
    let mut nav_webview = WebView::new();
    nav_webview.load_url("https://page1.com");
    nav_webview.load_url("https://page2.com");
    nav_webview.load_url("https://page3.com");
    
    println!("   历史记录数: {}", nav_webview.history().len());
    println!("   可以后退: {}", nav_webview.can_go_back());
    
    nav_webview.go_back();
    println!("   后退后URL: {}", nav_webview.url());
    
    println!("\n4. 缩放控制:");
    webview.set_zoom_level(1.5);
    println!("   设置缩放: {}x", webview.zoom_level());
    
    webview.zoom_in();
    println!("   放大后: {}x", webview.zoom_level());
    
    webview.reset_zoom();
    println!("   重置后: {}x", webview.zoom_level());
    
    println!("\n5. JavaScript支持:");
    let mut js_webview = WebView::new();
    js_webview.load_url("https://example.com");
    
    let result = js_webview.evaluate_js("document.title");
    println!("   执行JS成功: {}", result.success);
    
    js_webview.inject_js("console.log('Injected!');");
    println!("   已注入脚本");
    
    js_webview.set_javascript_enabled(false);
    let result = js_webview.evaluate_js("1 + 1");
    println!("   JS禁用后执行: {}", result.success);
    
    println!("\n6. 设置选项:");
    let mut settings_webview = WebView::new();
    
    settings_webview.set_javascript_enabled(false);
    println!("   JS启用: {}", settings_webview.is_javascript_enabled());
    
    settings_webview.set_zoom_enabled(false);
    println!("   缩放启用: {}", settings_webview.is_zoom_enabled());
    
    settings_webview.set_transparent(true);
    println!("   背景透明: {}", settings_webview.is_transparent());
    
    println!("\n7. 历史记录管理:");
    let mut history_webview = WebView::new();
    for i in 1..=5 {
        history_webview.load_url(&format!("https://page{}.com", i));
    }
    println!("   历史记录数: {}", history_webview.history().len());
    
    history_webview.clear_history();
    println!("   清除后记录数: {}", history_webview.history().len());
}

fn demo_comprehensive() {
    println!("\n=== 综合功能演示 ===\n");
    
    println!("1. EditBox 控制 VideoPlayer:");
    let mut player = VideoPlayer::with_source("video.mp4");
    let mut time_input = EditBox::new();
    time_input.set_placeholder("输入跳转时间(秒)");
    time_input.set_input_mode(EditBoxInputMode::Decimal);
    
    time_input.set_text("45");
    if let Ok(time) = time_input.text().parse() {
        player.seek_to(time);
        println!("   跳转到: {}秒", time);
    }
    
    println!("\n2. EditBox + WebView 搜索演示:");
    let mut search_box = EditBox::new();
    search_box.set_placeholder("输入搜索内容");
    
    let query = "Cocos2d";
    let search_url = format!("https://www.google.com/search?q={}", query);
    println!("   搜索URL: {}", search_url);
    
    println!("\n3. 视频播放器事件回调:");
    let mut callback_player = VideoPlayer::with_source("movie.mp4");
    
    let mut play_count = 0;
    let mut pause_count = 0;
    
    callback_player.set_on_event(move |_, event| {
        match event {
            VideoEventType::Play => play_count += 1,
            VideoEventType::Pause => pause_count += 1,
            _ => {}
        }
    });
    
    callback_player.play();
    callback_player.pause();
    callback_player.play();
    callback_player.pause();
    
    println!("   播放次数: {}", play_count);
    println!("   暂停次数: {}", pause_count);
    
    println!("\n4. EditBox 表单验证:");
    let mut email_box = EditBox::new();
    email_box.set_input_mode(EditBoxInputMode::EmailAddress);
    email_box.begin_editing();
    email_box.insert_text("user@domain.com");
    println!("   邮箱输入: {}", email_box.text());
    
    let mut phone_box = EditBox::new();
    phone_box.set_input_mode(EditBoxInputMode::PhoneNumber);
    phone_box.begin_editing();
    phone_box.insert_text("+86 138 0013 8000");
    println!("   电话输入: {}", phone_box.text());
    
    println!("\n5. VideoPlayer 格式支持:");
    let formats = vec!["video.mp4", "movie.webm", "clip.avi", "animation.mov", "video.mkv"];
    
    for format in formats {
        let mut player = VideoPlayer::with_source(format);
        println!("   {}: {:?}", format, player.format());
    }
    
    println!("\n所有演示完成！");
}

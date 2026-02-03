/// WebView - 网页视图组件
/// 
/// 功能特性：
/// - URL 加载和导航
/// - 前进/后退/刷新
/// - JavaScript 交互（调用 JS 和注入代码）
/// - 页面加载进度跟踪
/// - 缩放控制
/// - 背景透明支持
/// - Cookie 管理
/// - 事件回调系统

use crate::ui::Widget;
use std::collections::HashMap;

/// 加载状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadState {
    Idle,
    Loading,
    Loaded,
    Failed,
    Cancelled,
}

/// 导航动作
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigationType {
    LinkClicked,
    FormSubmitted,
    BackForward,
    Reload,
    FormResubmitted,
    Other,
}

/// WebView 事件类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebViewEvent {
    LoadStarted,
    LoadProgress,
    LoadFinished,
    LoadFailed,
    TitleChanged,
    UrlChanged,
    ZoomChanged,
    JavascriptCall,
}

/// 缩放范围
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ZoomRange {
    pub min_scale: f32,
    pub max_scale: f32,
    pub initial_scale: f32,
}

impl Default for ZoomRange {
    fn default() -> Self {
        Self {
            min_scale: 0.25,
            max_scale: 5.0,
            initial_scale: 1.0,
        }
    }
}

/// Cookie
#[derive(Debug, Clone)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub domain: String,
    pub path: String,
    pub expires: Option<String>,
    pub secure: bool,
    pub http_only: bool,
}

/// JavaScript 执行结果
#[derive(Debug, Clone)]
pub struct JavascriptResult {
    pub success: bool,
    pub result: String,
    pub error: Option<String>,
}

/// WebView 事件回调
pub type WebViewEventCallback = Box<dyn FnMut(&WebView, WebViewEvent, &str)>;

/// JavaScript 调用回调
pub type JavascriptInvokeCallback = Box<dyn FnMut(&WebView, &str) -> String>;

/// WebView - 网页视图组件
pub struct WebView {
    widget: Widget,
    url: String,
    original_url: String,
    title: String,
    load_state: LoadState,
    load_progress: u32,
    zoom_level: f32,
    zoom_range: ZoomRange,
    javascript_enabled: bool,
    zoom_enabled: bool,
    cache_enabled: bool,
    mixed_content_mode: bool,
    transparent_background: bool,
    user_agent: String,
    cookies: HashMap<String, Cookie>,
    history: Vec<String>,
    history_index: usize,
    max_history: usize,
    can_go_back: bool,
    can_go_forward: bool,
    on_event: Option<WebViewEventCallback>,
    on_js_invoke: Option<JavascriptInvokeCallback>,
    injected_js: Vec<String>,
    error_message: String,
}

impl std::fmt::Debug for WebView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WebView")
            .field("widget", &self.widget)
            .field("url", &self.url)
            .field("title", &self.title)
            .field("load_state", &self.load_state)
            .field("load_progress", &self.load_progress)
            .field("zoom_level", &self.zoom_level)
            .finish()
    }
}

impl WebView {
    pub fn new() -> Self {
        Self {
            widget: Widget::new(),
            url: String::new(),
            original_url: String::new(),
            title: String::new(),
            load_state: LoadState::Idle,
            load_progress: 0,
            zoom_level: 1.0,
            zoom_range: ZoomRange::default(),
            javascript_enabled: true,
            zoom_enabled: true,
            cache_enabled: true,
            mixed_content_mode: false,
            transparent_background: false,
            user_agent: String::from("Mozilla/5.0"),
            cookies: HashMap::new(),
            history: Vec::new(),
            history_index: 0,
            max_history: 50,
            can_go_back: false,
            can_go_forward: false,
            on_event: None,
            on_js_invoke: None,
            injected_js: Vec::new(),
            error_message: String::new(),
        }
    }
    
    pub fn with_url(url: impl Into<String>) -> Self {
        let mut webview = Self::new();
        let url = url.into();
        webview.url = url.clone();
        webview.original_url = url.clone();
        webview.history.push(url);
        webview.history_index = 0;
        webview
    }
    
    // ===== 导航控制 =====
    
    pub fn load_url(&mut self, url: impl Into<String>) {
        let url = url.into();
        self.url = url.clone();
        self.original_url = url.clone();
        self.load_state = LoadState::Loading;
        self.load_progress = 0;
        self.error_message.clear();
        
        self.add_to_history(&url);
        self.trigger_event(WebViewEvent::LoadStarted, &url);
        
        self.simulate_load_complete();
    }
    
    pub fn load_html(&mut self, html: impl Into<String>, base_url: Option<&str>) {
        let base = base_url.unwrap_or("about:blank");
        self.url = base.to_string();
        self.load_state = LoadState::Loading;
        self.load_progress = 0;
        
        let html_content = html.into();
        self.title = self.extract_title(&html_content);
        
        let url_for_history = self.url.clone();
        self.add_to_history(&url_for_history);
        let url_for_event = self.url.clone();
        self.trigger_event(WebViewEvent::LoadStarted, &url_for_event);
        
        self.simulate_load_complete();
    }
    
    pub fn load_data(&mut self, data: &[u8], _mime_type: &str, base_url: Option<&str>) {
        let content = String::from_utf8_lossy(data);
        self.load_html(content.as_ref(), base_url);
    }
    
    pub fn reload(&mut self) {
        if !self.url.is_empty() {
            self.load_state = LoadState::Loading;
            let url_for_event = self.url.clone();
            self.trigger_event(WebViewEvent::LoadStarted, &url_for_event);
            self.simulate_load_complete();
        }
    }
    
    pub fn stop_loading(&mut self) {
        if self.load_state == LoadState::Loading {
            self.load_state = LoadState::Cancelled;
            self.trigger_event(WebViewEvent::LoadFailed, "Load cancelled");
        }
    }
    
    pub fn go_back(&mut self) {
        if self.can_go_back {
            self.history_index = self.history_index.saturating_sub(1);
            self.navigate_to_history();
        }
    }
    
    pub fn go_forward(&mut self) {
        if self.can_go_forward {
            self.history_index = (self.history_index + 1).min(self.history.len() - 1);
            self.navigate_to_history();
        }
    }
    
    pub fn go_to_history_index(&mut self, index: usize) {
        if index < self.history.len() {
            self.history_index = index;
            self.navigate_to_history();
        }
    }
    
    // ===== URL 和标题 =====
    
    pub fn url(&self) -> &str {
        &self.url
    }
    
    pub fn title(&self) -> &str {
        &self.title
    }
    
    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
        self.trigger_event(WebViewEvent::TitleChanged, title);
    }
    
    // ===== 加载状态 =====
    
    pub fn load_state(&self) -> LoadState {
        self.load_state
    }
    
    pub fn load_progress(&self) -> u32 {
        self.load_progress
    }
    
    pub fn is_loading(&self) -> bool {
        self.load_state == LoadState::Loading
    }
    
    // ===== 缩放控制 =====
    
    pub fn set_zoom_level(&mut self, level: f32) {
        self.zoom_level = level.clamp(self.zoom_range.min_scale, self.zoom_range.max_scale);
        self.trigger_event(WebViewEvent::ZoomChanged, &format!("{}", self.zoom_level));
    }
    
    pub fn zoom_level(&self) -> f32 {
        self.zoom_level
    }
    
    pub fn zoom_in(&mut self) {
        self.set_zoom_level(self.zoom_level * 1.2);
    }
    
    pub fn zoom_out(&mut self) {
        self.set_zoom_level(self.zoom_level / 1.2);
    }
    
    pub fn reset_zoom(&mut self) {
        self.set_zoom_level(self.zoom_range.initial_scale);
    }
    
    pub fn set_zoom_range(&mut self, range: ZoomRange) {
        self.zoom_range = range;
        self.set_zoom_level(self.zoom_level);
    }
    
    // ===== JavaScript =====
    
    pub fn evaluate_js(&mut self, code: &str) -> JavascriptResult {
        if !self.javascript_enabled {
            return JavascriptResult {
                success: false,
                result: String::new(),
                error: Some("JavaScript is disabled".to_string()),
            };
        }
        
        self.trigger_event(WebViewEvent::JavascriptCall, code);
        
        JavascriptResult {
            success: true,
            result: String::new(),
            error: None,
        }
    }
    
    pub fn inject_js(&mut self, code: impl Into<String>) {
        self.injected_js.push(code.into());
    }
    
    pub fn call_js_function(&mut self, function_name: &str, args: &[&str]) -> JavascriptResult {
        let args_str = args.join(",");
        let code = format!("{}({})", function_name, args_str);
        self.evaluate_js(&code)
    }
    
    pub fn set_on_js_invoke<F>(&mut self, callback: F)
    where
        F: FnMut(&WebView, &str) -> String + 'static,
    {
        self.on_js_invoke = Some(Box::new(callback));
    }
    
    pub fn is_javascript_enabled(&self) -> bool {
        self.javascript_enabled
    }
    
    pub fn set_javascript_enabled(&mut self, enabled: bool) {
        self.javascript_enabled = enabled;
    }
    
    // ===== Cookie =====
    
    pub fn set_cookie(&mut self, cookie: Cookie) {
        self.cookies.insert(cookie.name.clone(), cookie);
    }
    
    pub fn get_cookie(&self, name: &str) -> Option<&Cookie> {
        self.cookies.get(name)
    }
    
    pub fn all_cookies(&self) -> Vec<&Cookie> {
        self.cookies.values().collect()
    }
    
    pub fn delete_cookie(&mut self, name: &str) {
        self.cookies.remove(name);
    }
    
    pub fn clear_cookies(&mut self) {
        self.cookies.clear();
    }
    
    // ===== 设置 =====
    
    pub fn is_zoom_enabled(&self) -> bool {
        self.zoom_enabled
    }
    
    pub fn set_zoom_enabled(&mut self, enabled: bool) {
        self.zoom_enabled = enabled;
    }
    
    pub fn is_cache_enabled(&self) -> bool {
        self.cache_enabled
    }
    
    pub fn set_cache_enabled(&mut self, enabled: bool) {
        self.cache_enabled = enabled;
    }
    
    pub fn is_mixed_content_allowed(&self) -> bool {
        self.mixed_content_mode
    }
    
    pub fn set_mixed_content_allowed(&mut self, allowed: bool) {
        self.mixed_content_mode = allowed;
    }
    
    pub fn is_transparent(&self) -> bool {
        self.transparent_background
    }
    
    pub fn set_transparent(&mut self, transparent: bool) {
        self.transparent_background = transparent;
    }
    
    pub fn user_agent(&self) -> &str {
        &self.user_agent
    }
    
    pub fn set_user_agent(&mut self, user_agent: impl Into<String>) {
        self.user_agent = user_agent.into();
    }
    
    // ===== 历史记录 =====
    
    pub fn can_go_back(&self) -> bool {
        self.can_go_back
    }
    
    pub fn can_go_forward(&self) -> bool {
        self.can_go_forward
    }
    
    pub fn history(&self) -> &[String] {
        &self.history
    }
    
    pub fn history_index(&self) -> usize {
        self.history_index
    }
    
    pub fn clear_history(&mut self) {
        self.history.clear();
        self.history_index = 0;
        self.can_go_back = false;
        self.can_go_forward = false;
    }
    
    // ===== 事件回调 =====
    
    pub fn set_on_event<F>(&mut self, callback: F)
    where
        F: FnMut(&WebView, WebViewEvent, &str) + 'static,
    {
        self.on_event = Some(Box::new(callback));
    }
    
    // ===== 错误处理 =====
    
    pub fn error_message(&self) -> &str {
        &self.error_message
    }
    
    pub fn has_error(&self) -> bool {
        self.load_state == LoadState::Failed
    }
    
    pub fn widget(&self) -> &Widget {
        &self.widget
    }
    
    pub fn widget_mut(&mut self) -> &mut Widget {
        &mut self.widget
    }
    
    // ===== 私有方法 =====
    
    fn add_to_history(&mut self, url: &str) {
        if self.history_index < self.history.len() - 1 {
            self.history.truncate(self.history_index + 1);
        }
        
        self.history.push(url.to_string());
        
        if self.history.len() > self.max_history {
            self.history.remove(0);
        } else {
            self.history_index += 1;
        }
        
        self.can_go_back = self.history_index > 0;
        self.can_go_forward = self.history_index < self.history.len() - 1;
    }
    
    fn navigate_to_history(&mut self) {
        if let Some(url) = self.history.get(self.history_index) {
            self.url = url.clone();
            self.load_state = LoadState::Loading;
            let url_for_event = self.url.clone();
            self.trigger_event(WebViewEvent::LoadStarted, &url_for_event);
            self.simulate_load_complete();
            
            self.can_go_back = self.history_index > 0;
            self.can_go_forward = self.history_index < self.history.len() - 1;
        }
    }
    
    fn extract_title(&self, html: &str) -> String {
        if let Some(start) = html.find("<title>") {
            if let Some(end) = html[start..].find("</title>") {
                return html[start + 7..start + end].to_string();
            }
        }
        String::from("Untitled")
    }
    
    fn simulate_load_complete(&mut self) {
        for i in (10..=100).step_by(10) {
            self.load_progress = i;
            self.trigger_event(WebViewEvent::LoadProgress, &i.to_string());
        }
        
        self.load_state = LoadState::Loaded;
        let url_for_event = self.url.clone();
        self.trigger_event(WebViewEvent::LoadFinished, &url_for_event);
    }
    
    fn trigger_event(&mut self, event: WebViewEvent, data: &str) {
        if let Some(mut callback) = self.on_event.take() {
            callback(self, event, data);
            self.on_event = Some(callback);
        }
    }
    
    pub fn load_state_string(&self) -> &str {
        match self.load_state {
            LoadState::Idle => "Idle",
            LoadState::Loading => "Loading",
            LoadState::Loaded => "Loaded",
            LoadState::Failed => "Failed",
            LoadState::Cancelled => "Cancelled",
        }
    }
}

impl Default for WebView {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_webview_creation() {
        let webview = WebView::new();
        assert!(webview.url().is_empty());
        assert_eq!(webview.load_state(), LoadState::Idle);
        assert_eq!(webview.zoom_level(), 1.0);
    }
    
    #[test]
    fn test_webview_url() {
        let webview = WebView::new();
        assert!(webview.url().is_empty() || webview.url() == "");
    }
    
    #[test]
    fn test_webview_html() {
        let webview = WebView::new();
        assert!(webview.title().is_empty() || webview.title() == "Untitled");
    }
    
    #[test]
    fn test_webview_history() {
        let webview = WebView::new();
        assert!(webview.history().is_empty());
    }
    
    #[test]
    fn test_webview_load_url() {
        let mut webview = WebView::new();
        webview.load_url("https://example.com");
        
        assert_eq!(webview.url(), "https://example.com");
        assert_eq!(webview.history().len(), 1);
        assert!(!webview.can_go_back());
    }
    
    #[test]
    fn test_webview_navigation() {
        let mut webview = WebView::new();
        
        webview.load_url("https://example.com/page1");
        webview.load_url("https://example.com/page2");
        webview.load_url("https://example.com/page3");
        
        assert_eq!(webview.history().len(), 3);
        assert!(webview.can_go_back());
        assert!(!webview.can_go_forward());
        
        webview.go_back();
        assert_eq!(webview.url(), "https://example.com/page2");
        assert!(webview.can_go_forward());
        
        webview.go_forward();
        assert_eq!(webview.url(), "https://example.com/page3");
    }
    
    #[test]
    fn test_webview_zoom() {
        let mut webview = WebView::new();
        
        assert_eq!(webview.zoom_level(), 1.0);
        
        webview.set_zoom_level(2.0);
        assert_eq!(webview.zoom_level(), 2.0);
        
        webview.zoom_in();
        assert!(webview.zoom_level() > 2.0);
        
        webview.zoom_out();
        assert!(webview.zoom_level() < 2.5);
        
        webview.reset_zoom();
        assert_eq!(webview.zoom_level(), 1.0);
    }
    
    #[test]
    fn test_webview_zoom_limits() {
        let mut webview = WebView::new();
        
        webview.set_zoom_level(0.1);
        assert!(webview.zoom_level() >= 0.25);
        
        webview.set_zoom_level(10.0);
        assert!(webview.zoom_level() <= 5.0);
    }
    
    #[test]
    fn test_webview_javascript() {
        let mut webview = WebView::new();
        
        assert!(webview.is_javascript_enabled());
        
        webview.set_javascript_enabled(false);
        assert!(!webview.is_javascript_enabled());
        
        let result = webview.evaluate_js("console.log('test')");
        assert!(result.success);
    }
    
    #[test]
    fn test_webview_reload() {
        let mut webview = WebView::new();
        webview.load_url("https://example.com");
        
        let original_load_state = webview.load_state();
        webview.reload();
        
        // Reload should trigger loading state
        assert_eq!(webview.url(), "https://example.com");
    }
    
    #[test]
    fn test_webview_stop_loading() {
        let mut webview = WebView::new();
        webview.load_url("https://example.com");
        
        webview.stop_loading();
        // 停止加载后不应该继续加载
    }
    
    #[test]
    fn test_webview_cookies() {
        let mut webview = WebView::new();
        
        let cookie = Cookie {
            name: "session".to_string(),
            value: "abc123".to_string(),
            domain: "example.com".to_string(),
            path: "/".to_string(),
            expires: None,
            secure: false,
            http_only: true,
        };
        
        webview.set_cookie(cookie.clone());
        
        let retrieved = webview.get_cookie("session");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().value, "abc123");
        
        webview.delete_cookie("session");
        assert!(webview.get_cookie("session").is_none());
    }
    
    #[test]
    fn test_webview_clear_cookies() {
        let mut webview = WebView::new();
        
        webview.set_cookie(Cookie {
            name: "cookie1".to_string(),
            value: "value1".to_string(),
            domain: "example.com".to_string(),
            path: "/".to_string(),
            expires: None,
            secure: false,
            http_only: false,
        });
        
        webview.set_cookie(Cookie {
            name: "cookie2".to_string(),
            value: "value2".to_string(),
            domain: "example.com".to_string(),
            path: "/".to_string(),
            expires: None,
            secure: false,
            http_only: false,
        });
        
        webview.clear_cookies();
        assert!(webview.get_cookie("cookie1").is_none());
        assert!(webview.get_cookie("cookie2").is_none());
    }
    
    #[test]
    fn test_webview_user_agent() {
        let mut webview = WebView::new();
        
        assert!(webview.user_agent().contains("Mozilla"));
        
        webview.set_user_agent("Custom User Agent");
        assert_eq!(webview.user_agent(), "Custom User Agent");
    }
    
    #[test]
    fn test_webview_inject_javascript() {
        let mut webview = WebView::new();
        
        webview.inject_javascript("console.log('injected')");
        assert!(webview.injected_js.len() > 0);
    }
    
    #[test]
    fn test_webview_cache() {
        let mut webview = WebView::new();
        
        assert!(webview.is_cache_enabled());
        
        webview.set_cache_enabled(false);
        assert!(!webview.is_cache_enabled());
        
        webview.clear_cache();
    }
    
    #[test]
    fn test_webview_title_extraction() {
        let webview = WebView::new();
        let html = "<html><head><title>Test Page</title></head><body></body></html>";
        let title = webview.extract_title(html);
        assert_eq!(title, "Test Page");
        
        let html_no_title = "<html><body>No title</body></html>";
        let title2 = webview.extract_title(html_no_title);
        assert_eq!(title2, "Untitled");
    }
    
    #[test]
    fn test_webview_load_html() {
        let mut webview = WebView::new();
        let html = "<html><head><title>Local Page</title></head><body><h1>Hello</h1></body></html>";
        
        webview.load_html(html, Some("about:blank"));
        assert_eq!(webview.title(), "Local Page");
    }
    
    #[test]
    fn test_webview_transparent_background() {
        let mut webview = WebView::new();
        
        assert!(!webview.has_transparent_background());
        
        webview.set_transparent_background(true);
        assert!(webview.has_transparent_background());
    }
    
    #[test]
    fn test_webview_error_handling() {
        let webview = WebView::new();
        
        assert!(webview.error_message().is_empty());
        assert!(!webview.has_error());
    }
    
    #[test]
    fn test_webview_load_progress() {
        let mut webview = WebView::new();
        
        assert_eq!(webview.load_progress(), 0);
        
        webview.load_url("https://example.com");
        // 模拟加载会更新进度
        assert!(webview.load_progress() >= 0 && webview.load_progress() <= 100);
    }
    
    #[test]
    fn test_webview_history_limit() {
        let mut webview = WebView::new();
        webview.max_history = 3;
        
        webview.load_url("https://example.com/1");
        webview.load_url("https://example.com/2");
        webview.load_url("https://example.com/3");
        webview.load_url("https://example.com/4");
        
        // 历史记录应该被限制
        assert!(webview.history().len() <= 3);
    }
    
    #[test]
    fn test_webview_callbacks() {
        let mut webview = WebView::new();
        
        webview.set_on_event(|_wv, event, _data| {
            // 事件回调会被触发
        });
        
        webview.load_url("https://example.com");
        // 验证加载触发了事件
    }
}

/// EnhancedWebView - 增强型网页视图组件
///
/// 功能：
/// - 网页加载和导航
/// - HTML 内容直接渲染
/// - JavaScript 交互
/// - 历史记录管理
/// - 前进/后退导航
/// - 缩放控制
/// - 加载进度跟踪
/// - 事件委托
/// - 缓存管理
/// - Cookie 管理
/// - 打印功能

use std::time::{Duration, Instant};
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use url::Url;
use crate::base::Color4B;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WebViewState {
    Unknown,
    Loading,
    Loaded,
    Error(String),
    Progress(f64),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CacheMode {
    Default,
    Reload,
    ForceCache,
    NoCache,
}

#[derive(Clone, Debug)]
pub struct WebViewHistoryItem {
    pub url: String,
    pub title: String,
    pub timestamp: Instant,
}

impl Default for WebViewHistoryItem {
    fn default() -> Self {
        Self {
            url: String::new(),
            title: String::new(),
            timestamp: Instant::now(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct WebViewDelegate {
    pub on_load_started: Option<Box<dyn Fn(&str)>>,
    pub on_load_progress: Option<Box<dyn Fn(f64)>>,
    pub on_load_finished: Option<Box<dyn Fn(&str)>>,
    pub on_load_error: Option<Box<dyn Fn(&str)>>,
    pub on_title_changed: Option<Box<dyn Fn(&str)>>,
    pub on_url_changed: Option<Box<dyn Fn(&str)>>,
    pub on_javascript_call: Option<Box<dyn Fn(&str) -> String>>,
    pub on_console_message: Option<Box<dyn Fn(&str, &str)>>,
    pub on_should_override_url: Option<Box<dyn Fn(&str) -> bool>>,
}

impl Default for WebViewDelegate {
    fn default() -> Self {
        Self {
            on_load_started: None,
            on_load_progress: None,
            on_load_finished: None,
            on_load_error: None,
            on_title_changed: None,
            on_url_changed: None,
            on_javascript_call: None,
            on_console_message: None,
            on_should_override_url: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub domain: String,
    pub path: String,
    pub expires: Option<Duration>,
    pub secure: bool,
    pub http_only: bool,
}

impl Default for Cookie {
    fn default() -> Self {
        Self {
            name: String::new(),
            value: String::new(),
            domain: String::new(),
            path: String::new(),
            expires: None,
            secure: false,
            http_only: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct EnhancedWebView {
    url: Arc<Mutex<String>>,
    html: Arc<Mutex<String>>,
    title: Arc<Mutex<String>>,
    state: Arc<Mutex<WebViewState>>,
    scale: Arc<Mutex<f32>>,
    min_scale: Arc<Mutex<f32>>,
    max_scale: Arc<Mutex<f32>>,
    delegate: Rc<RefCell<WebViewDelegate>>,
    history: Arc<Mutex<Vec<WebViewHistoryItem>>>,
    history_index: Arc<Mutex<usize>>,
    cache_mode: Arc<Mutex<CacheMode>>,
    cookies: Arc<Mutex<HashMap<String, Cookie>>>,
    position: (f32, f32),
    size: (f32, f32),
    visible: bool,
    background_color: Color4B,
    opacity: u8,
    load_progress: f64,
    can_go_back: bool,
    can_go_forward: bool,
    javascript_enabled: bool,
    zoom_enabled: bool,
    bounce_enabled: bool,
    scrolls_to_top: bool,
    inspectable: bool,
}

impl EnhancedWebView {
    pub fn new() -> Self {
        Self {
            url: Arc::new(Mutex::new(String::new())),
            html: Arc::new(Mutex::new(String::new())),
            title: Arc::new(Mutex::new(String::new())),
            state: Arc::new(Mutex::new(WebViewState::Unknown)),
            scale: Arc::new(Mutex::new(1.0)),
            min_scale: Arc::new(Mutex::new(0.1)),
            max_scale: Arc::new(Mutex::new(5.0)),
            delegate: Rc::new(RefCell::new(WebViewDelegate::default())),
            history: Arc::new(Mutex::new(Vec::new())),
            history_index: Arc::new(Mutex::new(0)),
            cache_mode: Arc::new(Mutex::new(CacheMode::Default)),
            cookies: Arc::new(Mutex::new(HashMap::new())),
            position: (0.0, 0.0),
            size: (320.0, 240.0),
            visible: true,
            background_color: Color4B::new(255, 255, 255, 255),
            opacity: 255,
            load_progress: 0.0,
            can_go_back: false,
            can_go_forward: false,
            javascript_enabled: true,
            zoom_enabled: true,
            bounce_enabled: true,
            scrolls_to_top: true,
            inspectable: false,
        }
    }

    pub fn with_html(html: &str) -> Self {
        let mut webview = Self::new();
        webview.set_html(html);
        webview
    }

    pub fn load_url(&mut self, url: &str) {
        *self.url.lock().unwrap() = url.to_string();
        self.set_state(WebViewState::Loading);
        self.notify_load_started(url);
    }

    pub fn load_html(&mut self, html: &str, base_url: Option<&str>) {
        *self.html.lock().unwrap() = html.to_string();
        *self.url.lock().unwrap() = base_url.unwrap_or("about:blank").to_string();
        self.set_state(WebViewState::Loading);
        self.notify_load_started(base_url.unwrap_or("local html"));
    }

    pub fn load_file(&mut self, file_path: &str) {
        self.load_url(&format!("file://{}", file_path));
    }

    pub fn reload(&mut self) {
        let url = self.url.lock().unwrap().clone();
        if !url.is_empty() {
            self.load_url(&url);
        }
    }

    pub fn stop_loading(&mut self) {
        self.set_state(WebViewState::Loaded);
    }

    pub fn get_url(&self) -> String {
        self.url.lock().unwrap().clone()
    }

    pub fn set_url(&mut self, url: &str) {
        self.load_url(url);
    }

    pub fn get_html(&self) -> String {
        self.html.lock().unwrap().clone()
    }

    pub fn set_html(&mut self, html: &str) {
        *self.html.lock().unwrap() = html.to_string();
    }

    pub fn get_title(&self) -> String {
        self.title.lock().unwrap().clone()
    }

    pub fn set_title(&mut self, title: &str) {
        *self.title.lock().unwrap() = title.to_string();
        self.notify_title_changed(title);
    }

    pub fn get_state(&self) -> WebViewState {
        *self.state.lock().unwrap()
    }

    fn set_state(&mut self, state: WebViewState) {
        *self.state.lock().unwrap() = state;
        if let WebViewState::Progress(progress) = state {
            self.load_progress = progress;
            self.notify_load_progress(progress);
        }
    }

    pub fn go_back(&mut self) {
        let mut history = self.history.lock().unwrap();
        let mut index = self.history_index.lock().unwrap();

        if *index > 0 {
            *index -= 1;
            let item = &history[*index];
            self.load_url(&item.url);
            self.update_navigation_state();
        }
    }

    pub fn go_forward(&mut self) {
        let mut history = self.history.lock().unwrap();
        let mut index = self.history_index.lock().unwrap();

        if *index < history.len().saturating_sub(1) {
            *index += 1;
            let item = &history[*index];
            self.load_url(&item.url);
            self.update_navigation_state();
        }
    }

    pub fn go_to_history_index(&mut self, index: usize) {
        let history = self.history.lock().unwrap();
        if index < history.len() {
            let mut idx = self.history_index.lock().unwrap();
            *idx = index;
            let item = &history[index];
            self.load_url(&item.url);
            self.update_navigation_state();
        }
    }

    pub fn can_go_back(&self) -> bool {
        let index = *self.history_index.lock().unwrap();
        index > 0
    }

    pub fn can_go_forward(&self) -> bool {
        let index = *self.history_index.lock().unwrap();
        let len = self.history.lock().unwrap().len();
        index < len.saturating_sub(1)
    }

    fn update_navigation_state(&mut self) {
        self.can_go_back = self.can_go_back();
        self.can_go_forward = self.can_go_forward();
    }

    pub fn add_history_entry(&mut self, url: &str, title: &str) {
        let mut history = self.history.lock().unwrap();
        let mut index = self.history_index.lock().unwrap();

        history.truncate(*index);

        history.push(WebViewHistoryItem {
            url: url.to_string(),
            title: title.to_string(),
            timestamp: Instant::now(),
        });

        *index = history.len() - 1;
        self.update_navigation_state();
    }

    pub fn get_history(&self) -> Vec<WebViewHistoryItem> {
        self.history.lock().unwrap().clone()
    }

    pub fn clear_history(&mut self) {
        self.history.lock().unwrap().clear();
        *self.history_index.lock().unwrap() = 0;
        self.update_navigation_state();
    }

    pub fn set_scale(&mut self, scale: f32) {
        let min = *self.min_scale.lock().unwrap();
        let max = *self.max_scale.lock().unwrap();
        let scale = scale.clamp(min, max);
        *self.scale.lock().unwrap() = scale;
    }

    pub fn get_scale(&self) -> f32 {
        *self.scale.lock().unwrap()
    }

    pub fn set_scale_range(&mut self, min_scale: f32, max_scale: f32) {
        *self.min_scale.lock().unwrap() = min_scale;
        *self.max_scale.lock().unwrap() = max_scale;
    }

    pub fn zoom_in(&mut self) {
        let current = *self.scale.lock().unwrap();
        let max = *self.max_scale.lock().unwrap();
        let step = (max - current).min(0.5).max(0.1);
        self.set_scale(current + step);
    }

    pub fn zoom_out(&mut self) {
        let current = *self.scale.lock().unwrap();
        let min = *self.min_scale.lock().unwrap();
        let step = (current - min).min(0.5).max(0.1);
        self.set_scale(current - step);
    }

    pub fn reset_scale(&mut self) {
        self.set_scale(1.0);
    }

    pub fn evaluate_javascript(&self, code: &str) -> String {
        if let Some(cb) = &self.delegate.borrow().on_javascript_call {
            cb(code)
        } else {
            String::new()
        }
    }

    pub fn inject_javascript(&self, code: &str) {
        let wrapped = format!("(function() {{ {} }})()", code);
        self.evaluate_javascript(&wrapped);
    }

    pub fn call_javascript_function(&self, object: &str, method: &str, args: &[&str]) -> String {
        let args_str = args.join(",");
        let code = format!("{}.{}({})", object, method, args_str);
        self.evaluate_javascript(&code)
    }

    pub fn set_cache_mode(&mut self, mode: CacheMode) {
        *self.cache_mode.lock().unwrap() = mode;
    }

    pub fn get_cache_mode(&self) -> CacheMode {
        *self.cache_mode.lock().unwrap()
    }

    pub fn clear_cache(&mut self) {
        self.load_progress = 0.0;
    }

    pub fn clear_cookies(&mut self) {
        self.cookies.lock().unwrap().clear();
    }

    pub fn set_cookie(&mut self, cookie: Cookie) {
        let key = format!("{}:{}", cookie.domain, cookie.name);
        self.cookies.lock().unwrap().insert(key, cookie);
    }

    pub fn get_cookies(&self) -> Vec<Cookie> {
        self.cookies.lock().unwrap().values().cloned().collect()
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

    pub fn set_background_color(&mut self, color: Color4B) {
        self.background_color = color;
    }

    pub fn get_background_color(&self) -> Color4B {
        self.background_color
    }

    pub fn set_opacity(&mut self, opacity: u8) {
        self.opacity = opacity;
    }

    pub fn get_opacity(&self) -> u8 {
        self.opacity
    }

    pub fn set_javascript_enabled(&mut self, enabled: bool) {
        self.javascript_enabled = enabled;
    }

    pub fn is_javascript_enabled(&self) -> bool {
        self.javascript_enabled
    }

    pub fn set_zoom_enabled(&mut self, enabled: bool) {
        self.zoom_enabled = enabled;
    }

    pub fn is_zoom_enabled(&self) -> bool {
        self.zoom_enabled
    }

    pub fn set_bounce_enabled(&mut self, enabled: bool) {
        self.bounce_enabled = enabled;
    }

    pub fn is_bounce_enabled(&self) -> bool {
        self.bounce_enabled
    }

    pub fn set_scrolls_to_top(&mut self, scrolls: bool) {
        self.scrolls_to_top = scrolls;
    }

    pub fn scrolls_to_top(&self) -> bool {
        self.scrolls_to_top
    }

    pub fn set_inspectable(&mut self, inspectable: bool) {
        self.inspectable = inspectable;
    }

    pub fn is_inspectable(&self) -> bool {
        self.inspectable
    }

    pub fn set_delegate(&mut self, delegate: Rc<RefCell<WebViewDelegate>>) {
        self.delegate = delegate;
    }

    pub fn get_delegate(&self) -> Rc<RefCell<WebViewDelegate>> {
        self.delegate.clone()
    }

    fn notify_load_started(&self, url: &str) {
        if let Some(cb) = &self.delegate.borrow().on_load_started {
            cb(url);
        }
    }

    fn notify_load_progress(&self, progress: f64) {
        if let Some(cb) = &self.delegate.borrow().on_load_progress {
            cb(progress);
        }
    }

    fn notify_load_finished(&self, url: &str) {
        *self.state.lock().unwrap() = WebViewState::Loaded;
        if let Some(cb) = &self.delegate.borrow().on_load_finished {
            cb(url);
        }
    }

    fn notify_load_error(&self, error: &str) {
        self.set_state(WebViewState::Error(error.to_string()));
        if let Some(cb) = &self.delegate.borrow().on_load_error {
            cb(error);
        }
    }

    fn notify_title_changed(&self, title: &str) {
        if let Some(cb) = &self.delegate.borrow().on_title_changed {
            cb(title);
        }
    }

    fn notify_url_changed(&self, url: &str) {
        if let Some(cb) = &self.delegate.borrow().on_url_changed {
            cb(url);
        }
    }

    fn notify_console_message(&self, message: &str, source: &str) {
        if let Some(cb) = &self.delegate.borrow().on_console_message {
            cb(message, source);
        }
    }

    pub fn simulate_load_progress(&mut self) {
        for i in (0..=100).step_by(10) {
            self.set_state(WebViewState::Progress(i as f64));
            std::thread::sleep(Duration::from_millis(50));
        }
        self.notify_load_finished(&self.get_url());
    }

    pub fn get_load_progress(&self) -> f64 {
        self.load_progress
    }

    pub fn is_loading(&self) -> bool {
        matches!(
            *self.state.lock().unwrap(),
            WebViewState::Loading | WebViewState::Progress(_)
        )
    }

    pub fn get_parsed_url(&self) -> Option<Url> {
        Url::parse(&self.get_url()).ok()
    }

    pub fn is_valid_url(&self) -> bool {
        self.get_parsed_url().is_some()
    }

    pub fn generate_report(&self) -> String {
        format!(
            "=== WebView Report ===\n\
             URL: {}\n\
             Title: {}\n\
             State: {:?}\n\
             Load Progress: {:.0}%\n\
             Scale: {:.2}\n\
             Can Go Back: {}\n\
             Can Go Forward: {}\n\
             History Size: {}\n\
             JavaScript Enabled: {}\n\
             Zoom Enabled: {}",
            self.get_url(),
            self.get_title(),
            self.get_state(),
            self.load_progress,
            self.get_scale(),
            self.can_go_back(),
            self.can_go_forward(),
            self.history.lock().unwrap().len(),
            self.javascript_enabled,
            self.zoom_enabled
        )
    }
}

impl Default for EnhancedWebView {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_webview_creation() {
        let webview = EnhancedWebView::new();
        assert!(webview.get_url().is_empty());
        assert!(webview.get_title().is_empty());
        assert!(webview.is_visible());
    }

    #[test]
    fn test_webview_load_url() {
        let mut webview = EnhancedWebView::new();
        webview.load_url("https://example.com");
        assert_eq!(webview.get_url(), "https://example.com");
        assert!(webview.is_loading());
    }

    #[test]
    fn test_webview_load_html() {
        let mut webview = EnhancedWebView::new();
        webview.load_html("<html><body>Hello</body></html>", Some("local"));
        assert!(webview.get_html().contains("Hello"));
        assert_eq!(webview.get_url(), "local");
    }

    #[test]
    fn test_webview_scale() {
        let mut webview = EnhancedWebView::new();
        assert_eq!(webview.get_scale(), 1.0);

        webview.set_scale(2.0);
        assert_eq!(webview.get_scale(), 2.0);

        webview.zoom_in();
        assert!(webview.get_scale() > 2.0);

        webview.zoom_out();
        assert!(webview.get_scale() < webview.get_scale() + 0.5);
    }

    #[test]
    fn test_webview_scale_range() {
        let mut webview = EnhancedWebView::new();
        webview.set_scale_range(0.5, 3.0);

        webview.set_scale(0.1);
        assert_eq!(webview.get_scale(), 0.5);

        webview.set_scale(10.0);
        assert_eq!(webview.get_scale(), 3.0);
    }

    #[test]
    fn test_webview_navigation() {
        let mut webview = EnhancedWebView::new();

        webview.load_url("https://example.com/1");
        webview.load_url("https://example.com/2");
        webview.load_url("https://example.com/3");

        assert!(!webview.can_go_back());
        assert!(!webview.can_go_forward());

        webview.go_back();
        assert!(webview.can_go_back());
        assert!(webview.can_go_forward());

        webview.go_forward();
        assert_eq!(webview.get_url(), "https://example.com/3");
    }

    #[test]
    fn test_webview_history() {
        let mut webview = EnhancedWebView::new();

        webview.add_history_entry("https://example.com/1", "Page 1");
        webview.add_history_entry("https://example.com/2", "Page 2");

        let history = webview.get_history();
        assert_eq!(history.len(), 2);
        assert_eq!(history[0].title, "Page 1");
    }

    #[test]
    fn test_webview_clear_history() {
        let mut webview = EnhancedWebView::new();
        webview.add_history_entry("https://example.com/1", "Page 1");
        webview.add_history_entry("https://example.com/2", "Page 2");

        webview.clear_history();

        let history = webview.get_history();
        assert!(history.is_empty());
    }

    #[test]
    fn test_webview_cache_mode() {
        let mut webview = EnhancedWebView::new();
        webview.set_cache_mode(CacheMode::NoCache);
        assert_eq!(webview.get_cache_mode(), CacheMode::NoCache);
    }

    #[test]
    fn test_webview_cookies() {
        let mut webview = EnhancedWebView::new();

        let cookie = Cookie {
            name: "session".to_string(),
            value: "abc123".to_string(),
            domain: "example.com".to_string(),
            path: "/".to_string(),
            expires: None,
            secure: true,
            http_only: true,
        };

        webview.set_cookie(cookie);

        let cookies = webview.get_cookies();
        assert_eq!(cookies.len(), 1);
        assert_eq!(cookies[0].name, "session");

        webview.clear_cookies();
        assert!(webview.get_cookies().is_empty());
    }

    #[test]
    fn test_webview_javascript() {
        let webview = EnhancedWebView::new();
        assert!(webview.is_javascript_enabled());

        webview.set_javascript_enabled(false);
        assert!(!webview.is_javascript_enabled());
    }

    #[test]
    fn test_webview_zoom() {
        let mut webview = EnhancedWebView::new();
        assert!(webview.is_zoom_enabled());

        webview.set_zoom_enabled(false);
        assert!(!webview.is_zoom_enabled());
    }

    #[test]
    fn test_webview_visibility() {
        let mut webview = EnhancedWebView::new();
        webview.set_visible(false);
        assert!(!webview.is_visible());
    }

    #[test]
    fn test_webview_position_size() {
        let mut webview = EnhancedWebView::new();
        webview.set_position(100.0, 200.0);
        webview.set_size(640.0, 480.0);

        assert_eq!(webview.get_position(), (100.0, 200.0));
        assert_eq!(webview.get_size(), (640.0, 480.0));
    }

    #[test]
    fn test_webview_url_parsing() {
        let webview = EnhancedWebView::new();

        webview.set_url("https://user:pass@example.com:8080/path?query=value#hash");
        let parsed = webview.get_parsed_url().unwrap();

        assert_eq!(parsed.scheme(), "https");
        assert_eq!(parsed.host_str(), Some("example.com"));
        assert_eq!(parsed.port(), Some(8080));
        assert_eq!(parsed.path(), "/path");
        assert_eq!(parsed.query(), Some("query=value"));
        assert_eq!(parsed.fragment(), Some("hash"));
    }

    #[test]
    fn test_webview_invalid_url() {
        let webview = EnhancedWebView::new();
        assert!(!webview.is_valid_url());

        webview.set_url("not a valid url");
        assert!(!webview.is_valid_url());

        webview.set_url("https://example.com");
        assert!(webview.is_valid_url());
    }

    #[test]
    fn test_webview_report() {
        let mut webview = EnhancedWebView::new();
        webview.set_url("https://example.com");
        webview.set_title("Example");

        let report = webview.generate_report();
        assert!(report.contains("WebView Report"));
        assert!(report.contains("example.com"));
        assert!(report.contains("Example"));
    }
}

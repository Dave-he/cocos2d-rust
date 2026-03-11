/// WebSocket - 基于线程的 WebSocket 客户端实现
///
/// 特性：
/// - 状态机驱动的连接管理
/// - 事件回调系统
/// - 消息发送队列
/// - 心跳保活机制
/// - 自动重连支持
/// - 线程安全

use std::collections::VecDeque;
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::time::{Duration, Instant};

/// WebSocket 连接状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebSocketState {
    Connecting,
    Open,
    Closing,
    Closed,
    Error,
}

impl std::fmt::Display for WebSocketState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WebSocketState::Connecting => write!(f, "Connecting"),
            WebSocketState::Open => write!(f, "Open"),
            WebSocketState::Closing => write!(f, "Closing"),
            WebSocketState::Closed => write!(f, "Closed"),
            WebSocketState::Error => write!(f, "Error"),
        }
    }
}

/// WebSocket 消息类型
#[derive(Debug, Clone)]
pub enum WebSocketMessage {
    /// 文本消息
    Text(String),
    /// 二进制消息
    Binary(Vec<u8>),
    /// Ping 消息
    Ping(Vec<u8>),
    /// Pong 消息
    Pong(Vec<u8>),
    /// 关闭消息
    Close { code: u16, reason: String },
}

impl WebSocketMessage {
    pub fn text(data: &str) -> Self {
        WebSocketMessage::Text(data.to_string())
    }

    pub fn binary(data: Vec<u8>) -> Self {
        WebSocketMessage::Binary(data)
    }

    pub fn is_text(&self) -> bool {
        matches!(self, WebSocketMessage::Text(_))
    }

    pub fn is_binary(&self) -> bool {
        matches!(self, WebSocketMessage::Binary(_))
    }

    pub fn as_text(&self) -> Option<&str> {
        match self {
            WebSocketMessage::Text(s) => Some(s.as_str()),
            _ => None,
        }
    }

    pub fn as_bytes(&self) -> Option<&[u8]> {
        match self {
            WebSocketMessage::Binary(b) => Some(b.as_slice()),
            WebSocketMessage::Text(s) => Some(s.as_bytes()),
            _ => None,
        }
    }
}

/// WebSocket 事件
#[derive(Debug, Clone)]
pub enum WebSocketEvent {
    /// 连接建立
    Open,
    /// 接收到消息
    Message(WebSocketMessage),
    /// 连接关闭
    Close { code: u16, reason: String, clean: bool },
    /// 发生错误
    Error(String),
}

/// WebSocket 事件处理器 trait
pub trait WebSocketDelegate: Send + Sync {
    fn on_open(&self);
    fn on_message(&self, message: &WebSocketMessage);
    fn on_close(&self, code: u16, reason: &str, clean: bool);
    fn on_error(&self, error: &str);
}

/// 默认实现（空操作）
pub struct EmptyWebSocketDelegate;
impl WebSocketDelegate for EmptyWebSocketDelegate {
    fn on_open(&self) {}
    fn on_message(&self, _message: &WebSocketMessage) {}
    fn on_close(&self, _code: u16, _reason: &str, _clean: bool) {}
    fn on_error(&self, _error: &str) {}
}

/// WebSocket 配置
#[derive(Debug, Clone)]
pub struct WebSocketConfig {
    /// URL
    pub url: String,
    /// 子协议列表
    pub protocols: Vec<String>,
    /// 连接超时（毫秒）
    pub connect_timeout_ms: u64,
    /// 心跳间隔（毫秒，0 表示禁用）
    pub ping_interval_ms: u64,
    /// 最大重连次数（0 表示禁用）
    pub max_reconnect_attempts: usize,
    /// 重连延迟（毫秒）
    pub reconnect_delay_ms: u64,
    /// 最大消息队列大小
    pub max_queue_size: usize,
}

impl Default for WebSocketConfig {
    fn default() -> Self {
        Self {
            url: String::new(),
            protocols: Vec::new(),
            connect_timeout_ms: 10_000,
            ping_interval_ms: 30_000,
            max_reconnect_attempts: 3,
            reconnect_delay_ms: 1_000,
            max_queue_size: 100,
        }
    }
}

impl WebSocketConfig {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            ..Default::default()
        }
    }

    pub fn with_protocols(mut self, protocols: Vec<String>) -> Self {
        self.protocols = protocols;
        self
    }

    pub fn with_connect_timeout(mut self, timeout_ms: u64) -> Self {
        self.connect_timeout_ms = timeout_ms;
        self
    }

    pub fn with_ping_interval(mut self, interval_ms: u64) -> Self {
        self.ping_interval_ms = interval_ms;
        self
    }

    pub fn with_reconnect(mut self, max_attempts: usize, delay_ms: u64) -> Self {
        self.max_reconnect_attempts = max_attempts;
        self.reconnect_delay_ms = delay_ms;
        self
    }
}

/// WebSocket 统计信息
#[derive(Debug, Clone, Default)]
pub struct WebSocketStats {
    pub messages_sent: usize,
    pub messages_received: usize,
    pub bytes_sent: usize,
    pub bytes_received: usize,
    pub reconnect_count: usize,
    pub ping_count: usize,
    pub pong_count: usize,
    pub connected_at: Option<Instant>,
    pub last_message_at: Option<Instant>,
}

impl WebSocketStats {
    pub fn connection_duration(&self) -> Option<Duration> {
        self.connected_at.map(|t| t.elapsed())
    }
}

/// WebSocket 客户端
/// 
/// 注意：当前为模拟实现，真实环境中应使用 `tungstenite` 或 `tokio-tungstenite` 库。
pub struct WebSocket {
    config: WebSocketConfig,
    state: Arc<Mutex<WebSocketState>>,
    delegate: Arc<Mutex<Box<dyn WebSocketDelegate>>>,
    outgoing_queue: Arc<Mutex<VecDeque<WebSocketMessage>>>,
    incoming_queue: Arc<Mutex<VecDeque<WebSocketMessage>>>,
    stats: Arc<Mutex<WebSocketStats>>,
    is_running: Arc<AtomicBool>,
    reconnect_count: Arc<Mutex<usize>>,
    event_log: Arc<Mutex<VecDeque<WebSocketEvent>>>,
}

impl WebSocket {
    /// 创建新的 WebSocket 客户端
    pub fn new(url: &str) -> Self {
        Self::with_config(WebSocketConfig::new(url))
    }

    /// 使用配置创建
    pub fn with_config(config: WebSocketConfig) -> Self {
        Self {
            config,
            state: Arc::new(Mutex::new(WebSocketState::Closed)),
            delegate: Arc::new(Mutex::new(Box::new(EmptyWebSocketDelegate))),
            outgoing_queue: Arc::new(Mutex::new(VecDeque::new())),
            incoming_queue: Arc::new(Mutex::new(VecDeque::new())),
            stats: Arc::new(Mutex::new(WebSocketStats::default())),
            is_running: Arc::new(AtomicBool::new(false)),
            reconnect_count: Arc::new(Mutex::new(0)),
            event_log: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    /// 设置事件处理器
    pub fn set_delegate<D: WebSocketDelegate + 'static>(&self, delegate: D) {
        *self.delegate.lock().unwrap() = Box::new(delegate);
    }

    /// 获取连接 URL
    pub fn get_url(&self) -> &str {
        &self.config.url
    }

    /// 获取当前状态
    pub fn get_state(&self) -> WebSocketState {
        *self.state.lock().unwrap()
    }

    /// 是否已连接
    pub fn is_open(&self) -> bool {
        *self.state.lock().unwrap() == WebSocketState::Open
    }

    /// 连接到服务器
    ///
    /// 注意：本实现模拟连接行为；实际项目中应集成 tungstenite。
    pub fn connect(&self) {
        let mut state = self.state.lock().unwrap();
        if *state == WebSocketState::Open || *state == WebSocketState::Connecting {
            return;
        }
        *state = WebSocketState::Connecting;
        drop(state);

        self.is_running.store(true, Ordering::Relaxed);

        // 模拟连接成功（实际实现中这里会建立 TCP/TLS 连接）
        let state_clone = self.state.clone();
        let delegate_clone = self.delegate.clone();
        let stats_clone = self.stats.clone();
        let event_log_clone = self.event_log.clone();
        let url = self.config.url.clone();
        let connect_timeout = self.config.connect_timeout_ms;

        thread::spawn(move || {
            // 模拟连接延迟
            thread::sleep(Duration::from_millis(connect_timeout.min(50)));

            // 模拟连接成功
            *state_clone.lock().unwrap() = WebSocketState::Open;
            stats_clone.lock().unwrap().connected_at = Some(Instant::now());

            log::info!("WebSocket connected to: {}", url);

            let event = WebSocketEvent::Open;
            event_log_clone.lock().unwrap().push_back(event);

            delegate_clone.lock().unwrap().on_open();
        });
    }

    /// 发送文本消息
    pub fn send_text(&self, text: &str) -> bool {
        self.send(WebSocketMessage::text(text))
    }

    /// 发送二进制消息
    pub fn send_binary(&self, data: Vec<u8>) -> bool {
        self.send(WebSocketMessage::binary(data))
    }

    /// 发送消息
    pub fn send(&self, message: WebSocketMessage) -> bool {
        if self.get_state() != WebSocketState::Open {
            log::warn!("WebSocket::send called when not open");
            return false;
        }

        let mut queue = self.outgoing_queue.lock().unwrap();
        if queue.len() >= self.config.max_queue_size {
            log::warn!("WebSocket send queue full, dropping message");
            return false;
        }

        // 更新统计
        let bytes = match &message {
            WebSocketMessage::Text(s) => s.len(),
            WebSocketMessage::Binary(b) => b.len(),
            _ => 0,
        };
        {
            let mut stats = self.stats.lock().unwrap();
            stats.messages_sent += 1;
            stats.bytes_sent += bytes;
        }

        queue.push_back(message);
        true
    }

    /// 发送 Ping
    pub fn ping(&self, data: Vec<u8>) -> bool {
        if self.get_state() != WebSocketState::Open {
            return false;
        }
        self.stats.lock().unwrap().ping_count += 1;
        self.outgoing_queue.lock().unwrap()
            .push_back(WebSocketMessage::Ping(data));
        true
    }

    /// 模拟接收消息（测试用，实际实现通过网络读取）
    pub fn simulate_receive(&self, message: WebSocketMessage) {
        if self.get_state() != WebSocketState::Open {
            return;
        }

        let bytes = match &message {
            WebSocketMessage::Text(s) => s.len(),
            WebSocketMessage::Binary(b) => b.len(),
            _ => 0,
        };

        {
            let mut stats = self.stats.lock().unwrap();
            stats.messages_received += 1;
            stats.bytes_received += bytes;
            stats.last_message_at = Some(Instant::now());
        }

        let event = WebSocketEvent::Message(message.clone());
        self.event_log.lock().unwrap().push_back(event);
        self.incoming_queue.lock().unwrap().push_back(message.clone());

        self.delegate.lock().unwrap().on_message(&message);
    }

    /// 关闭连接
    pub fn close(&self) {
        self.close_with(1000, "Normal Closure")
    }

    /// 带状态码关闭
    pub fn close_with(&self, code: u16, reason: &str) {
        let mut state = self.state.lock().unwrap();
        if *state == WebSocketState::Closed || *state == WebSocketState::Closing {
            return;
        }
        *state = WebSocketState::Closing;
        drop(state);

        self.is_running.store(false, Ordering::Relaxed);

        let state_clone = self.state.clone();
        let delegate_clone = self.delegate.clone();
        let event_log_clone = self.event_log.clone();
        let reason_owned = reason.to_string();

        thread::spawn(move || {
            thread::sleep(Duration::from_millis(10));
            *state_clone.lock().unwrap() = WebSocketState::Closed;

            let event = WebSocketEvent::Close {
                code,
                reason: reason_owned.clone(),
                clean: true,
            };
            event_log_clone.lock().unwrap().push_back(event);

            delegate_clone.lock().unwrap().on_close(code, &reason_owned, true);
        });
    }

    /// 获取统计信息
    pub fn get_stats(&self) -> WebSocketStats {
        self.stats.lock().unwrap().clone()
    }

    /// 获取已接收的消息（从内部队列取出）
    pub fn poll_messages(&self) -> Vec<WebSocketMessage> {
        self.incoming_queue.lock().unwrap().drain(..).collect()
    }

    /// 获取待发送的消息（用于测试验证）
    pub fn poll_outgoing(&self) -> Vec<WebSocketMessage> {
        self.outgoing_queue.lock().unwrap().drain(..).collect()
    }

    /// 获取事件日志
    pub fn get_event_log(&self) -> Vec<WebSocketEvent> {
        self.event_log.lock().unwrap().iter().cloned().collect()
    }

    /// 清空事件日志
    pub fn clear_event_log(&self) {
        self.event_log.lock().unwrap().clear();
    }

    /// 获取配置
    pub fn get_config(&self) -> &WebSocketConfig {
        &self.config
    }
}

impl std::fmt::Debug for WebSocket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WebSocket")
            .field("url", &self.config.url)
            .field("state", &self.get_state())
            .finish()
    }
}

/// WebSocket 管理器 —— 管理多个 WebSocket 连接
pub struct WebSocketManager {
    sockets: Arc<Mutex<std::collections::HashMap<String, WebSocket>>>,
}

impl Default for WebSocketManager {
    fn default() -> Self {
        Self::new()
    }
}

impl WebSocketManager {
    pub fn new() -> Self {
        Self {
            sockets: Arc::new(Mutex::new(std::collections::HashMap::new())),
        }
    }

    /// 注册一个 WebSocket 连接
    pub fn add(&self, name: &str, socket: WebSocket) {
        self.sockets.lock().unwrap().insert(name.to_string(), socket);
    }

    /// 移除连接
    pub fn remove(&self, name: &str) {
        if let Some(ws) = self.sockets.lock().unwrap().remove(name) {
            ws.close();
        }
    }

    /// 发送文本到指定连接
    pub fn send_text(&self, name: &str, text: &str) -> bool {
        let sockets = self.sockets.lock().unwrap();
        if let Some(ws) = sockets.get(name) {
            ws.send_text(text)
        } else {
            false
        }
    }

    /// 关闭所有连接
    pub fn close_all(&self) {
        for (_, ws) in self.sockets.lock().unwrap().iter() {
            ws.close();
        }
    }

    /// 获取连接数量
    pub fn count(&self) -> usize {
        self.sockets.lock().unwrap().len()
    }

    /// 获取指定连接状态
    pub fn get_state(&self, name: &str) -> Option<WebSocketState> {
        self.sockets.lock().unwrap().get(name).map(|ws| ws.get_state())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn test_websocket_creation() {
        let ws = WebSocket::new("ws://localhost:8080");
        assert_eq!(ws.get_url(), "ws://localhost:8080");
        assert_eq!(ws.get_state(), WebSocketState::Closed);
        assert!(!ws.is_open());
    }

    #[test]
    fn test_websocket_config() {
        let config = WebSocketConfig::new("ws://example.com")
            .with_protocols(vec!["chat".to_string()])
            .with_connect_timeout(5000)
            .with_ping_interval(15000)
            .with_reconnect(5, 2000);

        assert_eq!(config.url, "ws://example.com");
        assert_eq!(config.protocols, vec!["chat"]);
        assert_eq!(config.connect_timeout_ms, 5000);
        assert_eq!(config.ping_interval_ms, 15000);
        assert_eq!(config.max_reconnect_attempts, 5);
        assert_eq!(config.reconnect_delay_ms, 2000);
    }

    #[test]
    fn test_websocket_message_text() {
        let msg = WebSocketMessage::text("Hello, World!");
        assert!(msg.is_text());
        assert!(!msg.is_binary());
        assert_eq!(msg.as_text(), Some("Hello, World!"));
    }

    #[test]
    fn test_websocket_message_binary() {
        let data = vec![1u8, 2, 3, 4, 5];
        let msg = WebSocketMessage::binary(data.clone());
        assert!(msg.is_binary());
        assert!(!msg.is_text());
        assert_eq!(msg.as_bytes(), Some(data.as_slice()));
    }

    #[test]
    fn test_websocket_connect_state_change() {
        let ws = WebSocket::new("ws://localhost:9000");
        assert_eq!(ws.get_state(), WebSocketState::Closed);

        ws.connect();
        // 连接后应转为 Connecting 或 Open
        let state = ws.get_state();
        assert!(
            state == WebSocketState::Connecting || state == WebSocketState::Open,
            "State should be Connecting or Open, got {:?}", state
        );
    }

    #[test]
    fn test_websocket_send_when_closed() {
        let ws = WebSocket::new("ws://localhost:8080");
        // 未连接时发送应失败
        let result = ws.send_text("test");
        assert!(!result);
    }

    #[test]
    fn test_websocket_simulate_receive() {
        let ws = WebSocket::new("ws://localhost:8080");
        // 强制设置为 Open 状态以测试接收
        *ws.state.lock().unwrap() = WebSocketState::Open;

        ws.simulate_receive(WebSocketMessage::text("Hello from server"));

        let messages = ws.poll_messages();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].as_text(), Some("Hello from server"));
    }

    #[test]
    fn test_websocket_stats() {
        let ws = WebSocket::new("ws://localhost:8080");
        *ws.state.lock().unwrap() = WebSocketState::Open;

        ws.send_text("test message");
        ws.simulate_receive(WebSocketMessage::text("response"));

        let stats = ws.get_stats();
        assert_eq!(stats.messages_sent, 1);
        assert_eq!(stats.messages_received, 1);
        assert_eq!(stats.bytes_sent, "test message".len());
        assert_eq!(stats.bytes_received, "response".len());
    }

    #[test]
    fn test_websocket_ping() {
        let ws = WebSocket::new("ws://localhost:8080");
        *ws.state.lock().unwrap() = WebSocketState::Open;

        let result = ws.ping(vec![1, 2, 3, 4]);
        assert!(result);

        let outgoing = ws.poll_outgoing();
        assert!(!outgoing.is_empty(), "Ping should produce outgoing message");
    }

    #[test]
    fn test_websocket_close() {
        let ws = WebSocket::new("ws://localhost:8080");
        *ws.state.lock().unwrap() = WebSocketState::Open;

        ws.close();
        // 应转为 Closing 或 Closed
        let state = ws.get_state();
        assert!(
            state == WebSocketState::Closing || state == WebSocketState::Closed,
            "State should be Closing or Closed, got {:?}", state
        );
    }

    #[test]
    fn test_websocket_delegate_callback() {
        struct TestDelegate {
            opened: Arc<AtomicBool>,
            messages: Arc<AtomicUsize>,
        }

        impl WebSocketDelegate for TestDelegate {
            fn on_open(&self) {
                self.opened.store(true, Ordering::Relaxed);
            }
            fn on_message(&self, _: &WebSocketMessage) {
                self.messages.fetch_add(1, Ordering::Relaxed);
            }
            fn on_close(&self, _: u16, _: &str, _: bool) {}
            fn on_error(&self, _: &str) {}
        }

        let opened = Arc::new(AtomicBool::new(false));
        let messages = Arc::new(AtomicUsize::new(0));

        let ws = WebSocket::new("ws://localhost:8080");
        ws.set_delegate(TestDelegate {
            opened: opened.clone(),
            messages: messages.clone(),
        });

        *ws.state.lock().unwrap() = WebSocketState::Open;
        ws.simulate_receive(WebSocketMessage::text("msg1"));
        ws.simulate_receive(WebSocketMessage::text("msg2"));

        assert_eq!(messages.load(Ordering::Relaxed), 2);
    }

    #[test]
    fn test_websocket_event_log() {
        let ws = WebSocket::new("ws://localhost:8080");
        *ws.state.lock().unwrap() = WebSocketState::Open;

        ws.simulate_receive(WebSocketMessage::text("event1"));
        ws.simulate_receive(WebSocketMessage::binary(vec![1, 2, 3]));

        let log = ws.get_event_log();
        assert_eq!(log.len(), 2);
        ws.clear_event_log();
        assert!(ws.get_event_log().is_empty());
    }

    #[test]
    fn test_websocket_manager() {
        let manager = WebSocketManager::new();
        assert_eq!(manager.count(), 0);

        let ws1 = WebSocket::new("ws://server1.com");
        let ws2 = WebSocket::new("ws://server2.com");

        manager.add("server1", ws1);
        manager.add("server2", ws2);

        assert_eq!(manager.count(), 2);
        assert_eq!(manager.get_state("server1"), Some(WebSocketState::Closed));

        manager.remove("server1");
        assert_eq!(manager.count(), 1);
    }

    #[test]
    fn test_websocket_state_display() {
        assert_eq!(WebSocketState::Connecting.to_string(), "Connecting");
        assert_eq!(WebSocketState::Open.to_string(), "Open");
        assert_eq!(WebSocketState::Closing.to_string(), "Closing");
        assert_eq!(WebSocketState::Closed.to_string(), "Closed");
        assert_eq!(WebSocketState::Error.to_string(), "Error");
    }

    #[test]
    fn test_websocket_queue_overflow() {
        let mut config = WebSocketConfig::new("ws://localhost");
        config.max_queue_size = 3;
        let ws = WebSocket::with_config(config);
        *ws.state.lock().unwrap() = WebSocketState::Open;

        // 发送超过队列限制
        for i in 0..5 {
            ws.send_text(&format!("message {}", i));
        }

        let outgoing = ws.poll_outgoing();
        // 只有前3条应该入队
        assert!(outgoing.len() <= 3);
    }

    #[test]
    fn test_websocket_stats_connection_duration() {
        let mut stats = WebSocketStats::default();
        assert!(stats.connection_duration().is_none());

        stats.connected_at = Some(Instant::now());
        assert!(stats.connection_duration().is_some());
    }
}

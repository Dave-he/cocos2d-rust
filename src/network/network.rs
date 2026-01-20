use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

#[derive(Debug)]
pub struct HttpRequest {
    url: String,
    method: HttpMethod,
    headers: HashMap<String, String>,
    body: Vec<u8>,
    timeout: Duration,
}

impl HttpRequest {
    pub fn new(url: &str, method: HttpMethod) -> HttpRequest {
        HttpRequest {
            url: url.to_string(),
            method,
            headers: HashMap::new(),
            body: Vec::new(),
            timeout: Duration::from_secs(30),
        }
    }

    pub fn get_url(&self) -> &str {
        &self.url
    }

    pub fn get_method(&self) -> HttpMethod {
        self.method
    }

    pub fn set_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }

    pub fn get_header(&self, key: &str) -> Option<&str> {
        self.headers.get(key).map(|s| s.as_str())
    }

    pub fn set_body(&mut self, body: Vec<u8>) {
        self.body = body;
    }

    pub fn get_body(&self) -> &Vec<u8> {
        &self.body
    }

    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout;
    }

    pub fn get_timeout(&self) -> Duration {
        self.timeout
    }
}

#[derive(Debug)]
pub struct HttpResponse {
    code: i32,
    headers: HashMap<String, String>,
    body: Vec<u8>,
    error: Option<String>,
}

impl HttpResponse {
    pub fn new() -> HttpResponse {
        HttpResponse {
            code: 0,
            headers: HashMap::new(),
            body: Vec::new(),
            error: None,
        }
    }

    pub fn get_code(&self) -> i32 {
        self.code
    }

    pub fn set_code(&mut self, code: i32) {
        self.code = code;
    }

    pub fn get_header(&self, key: &str) -> Option<&str> {
        self.headers.get(key).map(|s| s.as_str())
    }

    pub fn set_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }

    pub fn get_body(&self) -> &Vec<u8> {
        &self.body
    }

    pub fn set_body(&mut self, body: Vec<u8>) {
        self.body = body;
    }

    pub fn get_error(&self) -> Option<&str> {
        self.error.as_deref()
    }

    pub fn set_error(&mut self, error: &str) {
        self.error = Some(error.to_string());
    }

    pub fn is_succeeded(&self) -> bool {
        self.code >= 200 && self.code < 300
    }
}

pub type HttpCallback = Arc<dyn Fn(HttpResponse) + Send + Sync>;

#[derive(Debug)]
pub struct HttpClient {
    requests: HashMap<i32, HttpRequest>,
    response_callbacks: HashMap<i32, HttpCallback>,
    current_request_id: i32,
}

impl HttpClient {
    pub fn new() -> HttpClient {
        HttpClient {
            requests: HashMap::new(),
            response_callbacks: HashMap::new(),
            current_request_id: 0,
        }
    }

    pub fn get_instance() -> &'static mut HttpClient {
        static mut HTTP_CLIENT: Option<HttpClient> = None;
        unsafe {
            if HTTP_CLIENT.is_none() {
                HTTP_CLIENT = Some(HttpClient::new());
            }
            HTTP_CLIENT.as_mut().unwrap()
        }
    }

    pub fn send(&mut self, request: HttpRequest, callback: HttpCallback) -> i32 {
        self.current_request_id += 1;
        let id = self.current_request_id;
        self.requests.insert(id, request);
        self.response_callbacks.insert(id, callback);
        id
    }

    pub fn get(&mut self, url: &str, callback: HttpCallback) -> i32 {
        let request = HttpRequest::new(url, HttpMethod::GET);
        self.send(request, callback)
    }

    pub fn post(&mut self, url: &str, body: Vec<u8>, callback: HttpCallback) -> i32 {
        let mut request = HttpRequest::new(url, HttpMethod::POST);
        request.set_body(body);
        self.send(request, callback)
    }

    pub fn cancel(&mut self, request_id: i32) {
        self.requests.remove(&request_id);
        self.response_callbacks.remove(&request_id);
    }

    pub fn cancel_all(&mut self) {
        self.requests.clear();
        self.response_callbacks.clear();
    }
}

#[derive(Debug)]
pub struct Network {
    reachability: NetworkReachability,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkReachability {
    NONE,
    WIFI,
    WAN,
}

impl Network {
    pub fn new() -> Network {
        Network {
            reachability: NetworkReachability::NONE,
        }
    }

    pub fn get_instance() -> &'static mut Network {
        static mut NETWORK: Option<Network> = None;
        unsafe {
            if NETWORK.is_none() {
                NETWORK = Some(Network::new());
            }
            NETWORK.as_mut().unwrap()
        }
    }

    pub fn get_network_reachability(&self) -> NetworkReachability {
        self.reachability
    }

    pub fn set_network_reachability(&mut self, reachability: NetworkReachability) {
        self.reachability = reachability;
    }

    pub fn is_internet_reachable(&self) -> bool {
        self.reachability != NetworkReachability::NONE
    }
}

#[derive(Debug)]
pub struct WebSocket {
    url: String,
    state: WebSocketState,
    message_queue: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebSocketState {
    CONNECTING,
    OPEN,
    CLOSING,
    CLOSED,
}

impl WebSocket {
    pub fn new(url: &str) -> WebSocket {
        WebSocket {
            url: url.to_string(),
            state: WebSocketState::CONNECTING,
            message_queue: Vec::new(),
        }
    }

    pub fn get_url(&self) -> &str {
        &self.url
    }

    pub fn get_state(&self) -> WebSocketState {
        self.state
    }

    pub fn send(&mut self, message: &str) {
        self.message_queue.push(message.to_string());
    }

    pub fn close(&mut self) {
        self.state = WebSocketState::CLOSING;
    }
}

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Unknown,
}

#[derive(Debug)]
pub struct HttpRequest {
    url: String,
    method: HttpMethod,
    headers: HashMap<String, String>,
    body: Vec<u8>,
    tag: String,
    timeout: u64,
}

impl HttpRequest {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            method: HttpMethod::Get,
            headers: HashMap::new(),
            body: Vec::new(),
            tag: String::new(),
            timeout: 30,
        }
    }

    pub fn with_method(mut self, method: HttpMethod) -> Self {
        self.method = method;
        self
    }

    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_body(mut self, body: Vec<u8>) -> Self {
        self.body = body;
        self
    }

    pub fn with_body_str(mut self, body: &str) -> Self {
        self.body = body.as_bytes().to_vec();
        self
    }

    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tag = tag.to_string();
        self
    }

    pub fn with_timeout(mut self, timeout: u64) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn get_url(&self) -> &str {
        &self.url
    }

    pub fn get_method(&self) -> HttpMethod {
        self.method
    }

    pub fn get_headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    pub fn get_body(&self) -> &[u8] {
        &self.body
    }

    pub fn get_tag(&self) -> &str {
        &self.tag
    }

    pub fn get_timeout(&self) -> u64 {
        self.timeout
    }

    pub fn set_url(&mut self, url: &str) {
        self.url = url.to_string();
    }

    pub fn set_method(&mut self, method: HttpMethod) {
        self.method = method;
    }

    pub fn add_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }

    pub fn set_body(&mut self, body: Vec<u8>) {
        self.body = body;
    }

    pub fn set_tag(&mut self, tag: &str) {
        self.tag = tag.to_string();
    }

    pub fn set_timeout(&mut self, timeout: u64) {
        self.timeout = timeout;
    }
}

#[derive(Debug)]
pub struct HttpResponse {
    request: HttpRequest,
    status_code: u16,
    headers: HashMap<String, String>,
    body: Vec<u8>,
    error_message: String,
    success: bool,
}

impl HttpResponse {
    pub fn new(request: HttpRequest) -> Self {
        Self {
            request,
            status_code: 0,
            headers: HashMap::new(),
            body: Vec::new(),
            error_message: String::new(),
            success: false,
        }
    }

    pub fn with_status(mut self, status_code: u16) -> Self {
        self.status_code = status_code;
        self.success = (200..300).contains(&status_code);
        self
    }

    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_body(mut self, body: Vec<u8>) -> Self {
        self.body = body;
        self
    }

    pub fn with_error(mut self, error_message: &str) -> Self {
        self.error_message = error_message.to_string();
        self.success = false;
        self
    }

    pub fn get_request(&self) -> &HttpRequest {
        &self.request
    }

    pub fn get_status_code(&self) -> u16 {
        self.status_code
    }

    pub fn get_headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    pub fn get_body(&self) -> &[u8] {
        &self.body
    }

    pub fn get_body_str(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(&self.body)
    }

    pub fn get_error_message(&self) -> &str {
        &self.error_message
    }

    pub fn is_success(&self) -> bool {
        self.success
    }
}

pub struct HttpClient {
    max_concurrent_requests: usize,
}

impl HttpClient {
    pub fn new() -> Self {
        Self {
            max_concurrent_requests: 5,
        }
    }

    pub fn send(&self, request: HttpRequest) -> HttpResponse {
        HttpResponse::new(request)
            .with_status(200)
            .with_body(Vec::new())
    }

    pub fn set_max_concurrent_requests(&mut self, max: usize) {
        self.max_concurrent_requests = max;
    }

    pub fn get_max_concurrent_requests(&self) -> usize {
        self.max_concurrent_requests
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_request_creation() {
        let request = HttpRequest::new("https://example.com");
        assert_eq!(request.get_url(), "https://example.com");
        assert_eq!(request.get_method(), HttpMethod::Get);
        assert_eq!(request.get_timeout(), 30);
    }

    #[test]
    fn test_http_request_builder() {
        let request = HttpRequest::new("https://api.example.com")
            .with_method(HttpMethod::Post)
            .with_header("Content-Type", "application/json")
            .with_body_str("{\"key\":\"value\"}")
            .with_tag("test_request")
            .with_timeout(60);

        assert_eq!(request.get_method(), HttpMethod::Post);
        assert_eq!(request.get_headers().get("Content-Type").unwrap(), "application/json");
        assert_eq!(request.get_body(), b"{\"key\":\"value\"}");
        assert_eq!(request.get_tag(), "test_request");
        assert_eq!(request.get_timeout(), 60);
    }

    #[test]
    fn test_http_request_setters() {
        let mut request = HttpRequest::new("https://example.com");
        request.set_method(HttpMethod::Put);
        request.add_header("Authorization", "Bearer token");
        request.set_body(vec![1, 2, 3, 4]);
        request.set_tag("custom_tag");
        request.set_timeout(120);

        assert_eq!(request.get_method(), HttpMethod::Put);
        assert_eq!(request.get_headers().get("Authorization").unwrap(), "Bearer token");
        assert_eq!(request.get_body(), &[1, 2, 3, 4]);
        assert_eq!(request.get_tag(), "custom_tag");
        assert_eq!(request.get_timeout(), 120);
    }

    #[test]
    fn test_http_response_creation() {
        let request = HttpRequest::new("https://example.com");
        let response = HttpResponse::new(request);

        assert_eq!(response.get_status_code(), 0);
        assert!(!response.is_success());
    }

    #[test]
    fn test_http_response_builder() {
        let request = HttpRequest::new("https://example.com");
        let response = HttpResponse::new(request)
            .with_status(200)
            .with_header("Content-Type", "text/html")
            .with_body(b"Hello World".to_vec());

        assert_eq!(response.get_status_code(), 200);
        assert!(response.is_success());
        assert_eq!(response.get_headers().get("Content-Type").unwrap(), "text/html");
        assert_eq!(response.get_body(), b"Hello World");
    }

    #[test]
    fn test_http_response_body_str() {
        let request = HttpRequest::new("https://example.com");
        let response = HttpResponse::new(request)
            .with_status(200)
            .with_body(b"Test Response".to_vec());

        assert_eq!(response.get_body_str().unwrap(), "Test Response");
    }

    #[test]
    fn test_http_response_error() {
        let request = HttpRequest::new("https://example.com");
        let response = HttpResponse::new(request)
            .with_error("Network timeout");

        assert!(!response.is_success());
        assert_eq!(response.get_error_message(), "Network timeout");
    }

    #[test]
    fn test_http_response_status_ranges() {
        let request1 = HttpRequest::new("https://example.com");
        let response1 = HttpResponse::new(request1).with_status(201);
        assert!(response1.is_success());

        let request2 = HttpRequest::new("https://example.com");
        let response2 = HttpResponse::new(request2).with_status(404);
        assert!(!response2.is_success());

        let request3 = HttpRequest::new("https://example.com");
        let response3 = HttpResponse::new(request3).with_status(500);
        assert!(!response3.is_success());
    }

    #[test]
    fn test_http_client_creation() {
        let client = HttpClient::new();
        assert_eq!(client.get_max_concurrent_requests(), 5);
    }

    #[test]
    fn test_http_client_send() {
        let client = HttpClient::new();
        let request = HttpRequest::new("https://example.com")
            .with_method(HttpMethod::Get);

        let response = client.send(request);
        assert_eq!(response.get_status_code(), 200);
    }

    #[test]
    fn test_http_client_max_concurrent() {
        let mut client = HttpClient::new();
        client.set_max_concurrent_requests(10);
        assert_eq!(client.get_max_concurrent_requests(), 10);
    }

    #[test]
    fn test_http_method_equality() {
        assert_eq!(HttpMethod::Get, HttpMethod::Get);
        assert_ne!(HttpMethod::Get, HttpMethod::Post);
    }

    #[test]
    fn test_multiple_headers() {
        let request = HttpRequest::new("https://example.com")
            .with_header("Accept", "application/json")
            .with_header("User-Agent", "TestClient/1.0")
            .with_header("Authorization", "Bearer token");

        assert_eq!(request.get_headers().len(), 3);
        assert_eq!(request.get_headers().get("Accept").unwrap(), "application/json");
        assert_eq!(request.get_headers().get("User-Agent").unwrap(), "TestClient/1.0");
    }

    #[test]
    fn test_empty_body() {
        let request = HttpRequest::new("https://example.com");
        assert!(request.get_body().is_empty());
    }

    #[test]
    fn test_binary_body() {
        let binary_data = vec![0xFF, 0xFE, 0xFD, 0xFC];
        let request = HttpRequest::new("https://example.com")
            .with_body(binary_data.clone());

        assert_eq!(request.get_body(), &binary_data[..]);
    }
}

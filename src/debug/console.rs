/// Console - 控制台系统
///
/// 提供实时日志输出、命令行接口和历史记录

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};

/// 日志级别
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warning = 3,
    Error = 4,
    Fatal = 5,
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Trace => "TRACE",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warning => "WARN",
            LogLevel::Error => "ERROR",
            LogLevel::Fatal => "FATAL",
        }
    }
}

/// 日志条目
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: f64,
    pub level: LogLevel,
    pub message: String,
    pub file: String,
    pub line: u32,
}

impl LogEntry {
    pub fn new(timestamp: f64, level: LogLevel, message: String, file: String, line: u32) -> Self {
        Self {
            timestamp,
            level,
            message,
            file,
            line,
        }
    }
}

/// 命令处理器
pub type CommandHandler = Arc<dyn Fn(&[&str]) -> Result<String, String> + Send + Sync>;

/// 控制台系统
pub struct Console {
    logs: VecDeque<LogEntry>,
    max_logs: usize,
    commands: HashMap<String, CommandHandler>,
    history: VecDeque<String>,
    max_history: usize,
    visible: bool,
    filter_level: LogLevel,
    start_time: std::time::Instant,
}

impl Console {
    pub fn new() -> Self {
        Self {
            logs: VecDeque::new(),
            max_logs: 1000,
            commands: HashMap::new(),
            history: VecDeque::new(),
            max_history: 100,
            visible: false,
            filter_level: LogLevel::Trace,
            start_time: std::time::Instant::now(),
        }
    }

    pub fn with_capacity(max_logs: usize, max_history: usize) -> Self {
        Self {
            logs: VecDeque::with_capacity(max_logs),
            max_logs,
            commands: HashMap::new(),
            history: VecDeque::with_capacity(max_history),
            max_history,
            visible: false,
            filter_level: LogLevel::Trace,
            start_time: std::time::Instant::now(),
        }
    }

    fn get_timestamp(&self) -> f64 {
        self.start_time.elapsed().as_secs_f64()
    }

    pub fn log(&mut self, level: LogLevel, message: impl Into<String>) {
        self.log_with_location(level, message, file!(), line!());
    }

    pub fn log_with_location(&mut self, level: LogLevel, message: impl Into<String>, file: &str, line: u32) {
        let entry = LogEntry::new(
            self.get_timestamp(),
            level,
            message.into(),
            file.to_string(),
            line,
        );

        self.logs.push_back(entry);

        if self.logs.len() > self.max_logs {
            self.logs.pop_front();
        }
    }

    pub fn trace(&mut self, message: impl Into<String>) {
        self.log(LogLevel::Trace, message);
    }

    pub fn debug(&mut self, message: impl Into<String>) {
        self.log(LogLevel::Debug, message);
    }

    pub fn info(&mut self, message: impl Into<String>) {
        self.log(LogLevel::Info, message);
    }

    pub fn warning(&mut self, message: impl Into<String>) {
        self.log(LogLevel::Warning, message);
    }

    pub fn error(&mut self, message: impl Into<String>) {
        self.log(LogLevel::Error, message);
    }

    pub fn fatal(&mut self, message: impl Into<String>) {
        self.log(LogLevel::Fatal, message);
    }

    pub fn register_command<F>(&mut self, name: impl Into<String>, handler: F)
    where
        F: Fn(&[&str]) -> Result<String, String> + Send + Sync + 'static,
    {
        self.commands.insert(name.into(), Arc::new(handler));
    }

    pub fn execute_command(&mut self, command: &str) -> Result<String, String> {
        let command = command.trim();
        if command.is_empty() {
            return Err("Empty command".to_string());
        }

        self.history.push_back(command.to_string());
        if self.history.len() > self.max_history {
            self.history.pop_front();
        }

        let parts: Vec<&str> = command.split_whitespace().collect();
        let cmd_name = parts[0];
        let args = &parts[1..];

        if let Some(handler) = self.commands.get(cmd_name) {
            handler(args)
        } else {
            Err(format!("Unknown command: {}", cmd_name))
        }
    }

    pub fn get_logs(&self, level: Option<LogLevel>) -> Vec<&LogEntry> {
        match level {
            Some(min_level) => self
                .logs
                .iter()
                .filter(|entry| entry.level >= min_level)
                .collect(),
            None => self.logs.iter().collect(),
        }
    }

    pub fn get_filtered_logs(&self) -> Vec<&LogEntry> {
        self.get_logs(Some(self.filter_level))
    }

    pub fn get_history(&self) -> &VecDeque<String> {
        &self.history
    }

    pub fn clear(&mut self) {
        self.logs.clear();
    }

    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn toggle_visible(&mut self) {
        self.visible = !self.visible;
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn set_filter_level(&mut self, level: LogLevel) {
        self.filter_level = level;
    }

    pub fn get_filter_level(&self) -> LogLevel {
        self.filter_level
    }

    pub fn set_max_logs(&mut self, max_logs: usize) {
        self.max_logs = max_logs;
        while self.logs.len() > max_logs {
            self.logs.pop_front();
        }
    }

    pub fn get_log_count(&self) -> usize {
        self.logs.len()
    }

    pub fn get_command_names(&self) -> Vec<&str> {
        self.commands.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for Console {
    fn default() -> Self {
        let mut console = Self::new();

        console.register_command("help", |_args| {
            Ok("Available commands: help, clear, history, level, quit".to_string())
        });

        console.register_command("clear", |_args| {
            Ok("Logs cleared".to_string())
        });

        console.register_command("history", |_args| {
            Ok("Command history (use console.get_history())".to_string())
        });

        console
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_console_creation() {
        let console = Console::new();
        assert_eq!(console.get_log_count(), 0);
        assert!(!console.is_visible());
    }

    #[test]
    fn test_log_levels() {
        let mut console = Console::new();
        
        console.trace("trace message");
        console.debug("debug message");
        console.info("info message");
        console.warning("warning message");
        console.error("error message");
        console.fatal("fatal message");

        assert_eq!(console.get_log_count(), 6);
    }

    #[test]
    fn test_log_filtering() {
        let mut console = Console::new();
        
        console.trace("trace");
        console.debug("debug");
        console.info("info");
        console.warning("warning");
        console.error("error");

        let warnings_and_above = console.get_logs(Some(LogLevel::Warning));
        assert_eq!(warnings_and_above.len(), 2);

        console.set_filter_level(LogLevel::Info);
        let filtered = console.get_filtered_logs();
        assert_eq!(filtered.len(), 3);
    }

    #[test]
    fn test_max_logs() {
        let mut console = Console::with_capacity(5, 10);
        
        for i in 0..10 {
            console.info(format!("Message {}", i));
        }

        assert_eq!(console.get_log_count(), 5);
        
        let logs = console.get_logs(None);
        assert_eq!(logs[0].message, "Message 5");
    }

    #[test]
    fn test_command_registration() {
        let mut console = Console::new();
        
        console.register_command("test", |_args| {
            Ok("Test command executed".to_string())
        });

        let result = console.execute_command("test");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Test command executed");
    }

    #[test]
    fn test_command_with_args() {
        let mut console = Console::new();
        
        console.register_command("echo", |args| {
            Ok(args.join(" "))
        });

        let result = console.execute_command("echo hello world");
        assert_eq!(result.unwrap(), "hello world");
    }

    #[test]
    fn test_unknown_command() {
        let mut console = Console::new();
        let result = console.execute_command("unknown");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unknown command"));
    }

    #[test]
    fn test_empty_command() {
        let mut console = Console::new();
        let result = console.execute_command("");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Empty command"));
    }

    #[test]
    fn test_command_history() {
        let mut console = Console::new();
        
        console.register_command("test", |_| Ok("ok".to_string()));
        
        console.execute_command("test").ok();
        console.execute_command("test arg1").ok();
        console.execute_command("test arg2").ok();

        let history = console.get_history();
        assert_eq!(history.len(), 3);
        assert_eq!(history[0], "test");
        assert_eq!(history[2], "test arg2");
    }

    #[test]
    fn test_history_limit() {
        let mut console = Console::with_capacity(100, 3);
        
        console.register_command("cmd", |_| Ok("ok".to_string()));
        
        for i in 0..5 {
            console.execute_command(&format!("cmd {}", i)).ok();
        }

        assert_eq!(console.get_history().len(), 3);
        assert_eq!(console.get_history()[0], "cmd 2");
    }

    #[test]
    fn test_clear() {
        let mut console = Console::new();
        
        console.info("test");
        console.info("test2");
        assert_eq!(console.get_log_count(), 2);

        console.clear();
        assert_eq!(console.get_log_count(), 0);
    }

    #[test]
    fn test_visibility_toggle() {
        let mut console = Console::new();
        
        assert!(!console.is_visible());
        
        console.set_visible(true);
        assert!(console.is_visible());
        
        console.toggle_visible();
        assert!(!console.is_visible());
        
        console.toggle_visible();
        assert!(console.is_visible());
    }

    #[test]
    fn test_timestamp() {
        let mut console = Console::new();
        
        console.info("message1");
        std::thread::sleep(std::time::Duration::from_millis(10));
        console.info("message2");

        let logs = console.get_logs(None);
        assert!(logs[1].timestamp > logs[0].timestamp);
    }

    #[test]
    fn test_file_and_line() {
        let mut console = Console::new();
        console.log_with_location(LogLevel::Info, "test", "test.rs", 42);

        let logs = console.get_logs(None);
        assert_eq!(logs[0].file, "test.rs");
        assert_eq!(logs[0].line, 42);
    }

    #[test]
    fn test_default_commands() {
        let console = Console::default();
        let commands = console.get_command_names();
        
        assert!(commands.contains(&"help"));
        assert!(commands.contains(&"clear"));
        assert!(commands.contains(&"history"));
    }
}

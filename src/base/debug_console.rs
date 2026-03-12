#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
/// DebugConsole - 控制台组件
///
/// 功能：
/// - 在屏幕上显示调试信息
/// - 日志消息管理
/// - 命令输入和执行
/// - 滚动和历史记录
/// - 过滤和搜索

use std::collections::VecDeque;
use std::time::Instant;
use std::sync::mpsc;
use std::thread;

#[derive(Clone, Copy, Debug)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}

impl LogLevel {
    pub fn to_string(&self) -> &'static str {
        match self {
            LogLevel::Debug => "[DEBUG]",
            LogLevel::Info => "[INFO]",
            LogLevel::Warning => "[WARN]",
            LogLevel::Error => "[ERROR]",
            LogLevel::Critical => "[CRIT]",
        }
    }
}

#[derive(Clone)]
pub struct LogEntry {
    pub level: LogLevel,
    pub message: String,
    pub timestamp: Instant,
    pub tag: Option<String>,
}

impl LogEntry {
    pub fn new(level: LogLevel, message: String, tag: Option<String>) -> Self {
        Self {
            level,
            message,
            timestamp: Instant::now(),
            tag,
        }
    }

    pub fn formatted(&self) -> String {
        let elapsed = self.timestamp.elapsed().as_secs_f64();
        let tag_str = match &self.tag {
            Some(t) => format!("[{}]", t),
            None => String::new(),
        };
        format!("{:.3}s {} {} {}", elapsed, self.level.to_string(), tag_str, self.message)
    }
}

pub enum ConsoleCommand {
    Help,
    Clear,
    Stats,
    Profile(String),
    Set(String, String),
    ListVars,
    RunScript(String),
    Quit,
    Unknown(String),
}

pub struct CommandResult {
    pub output: String,
    pub success: bool,
}

impl CommandResult {
    pub fn new(output: String, success: bool) -> Self {
        Self { output, success }
    }
}

pub trait ConsoleHandler: Send {
    fn handle_command(&mut self, command: &str) -> CommandResult;
    fn get_help(&self) -> String;
}

struct DefaultHandler;

impl ConsoleHandler for DefaultHandler {
    fn handle_command(&mut self, command: &str) -> CommandResult {
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() {
            return CommandResult::new(String::new(), true);
        }

        match parts[0].to_lowercase().as_str() {
            "help" => CommandResult::new(self.get_help(), true),
            "clear" => CommandResult::new("Console cleared".to_string(), true),
            "stats" => {
                let stats = crate::base::DebugStats::new();
                CommandResult::new(stats.generate_report(), true)
            }
            "set" => {
                if parts.len() >= 3 {
                    CommandResult::new(format!("Set {} = {}", parts[1], parts[2]), true)
                } else {
                    CommandResult::new("Usage: set <var> <value>".to_string(), false)
                }
            }
            "list" | "vars" => CommandResult::new("Variables: (none)".to_string(), true),
            "quit" | "exit" => CommandResult::new("Quit command received".to_string(), true),
            _ => CommandResult::new(format!("Unknown command: {}", parts[0]), false),
        }
    }

    fn get_help(&self) -> String {
        ["=== Debug Console Commands ===",
            "help - Show this help message",
            "clear - Clear console output",
            "stats - Show debug statistics",
            "set <var> <value> - Set a variable",
            "list - List all variables",
            "quit - Exit console",
            "",
            "Custom commands can be registered via ConsoleHandler"].join("\n")
    }
}

#[derive(Clone)]
pub struct ConsoleStyle {
    pub background_color: [f32; 4],
    pub text_color: [f32; 4],
    pub font_size: f32,
    pub max_lines: usize,
    pub show_timestamps: bool,
    pub auto_scroll: bool,
}

impl Default for ConsoleStyle {
    fn default() -> Self {
        Self {
            background_color: [0.0, 0.0, 0.0, 0.8],
            text_color: [0.9, 0.9, 0.9, 1.0],
            font_size: 14.0,
            max_lines: 1000,
            show_timestamps: true,
            auto_scroll: true,
        }
    }
}

pub struct DebugConsole {
    logs: VecDeque<LogEntry>,
    style: ConsoleStyle,
    visible: bool,
    handler: Box<dyn ConsoleHandler>,
    command_history: VecDeque<String>,
    history_index: usize,
    current_input: String,
    show_command_prompt: bool,
    log_sender: mpsc::Sender<LogEntry>,
    log_receiver: mpsc::Receiver<LogEntry>,
    background_thread: Option<thread::JoinHandle<()>>,
}

impl DebugConsole {
    pub fn new() -> Self {
        let (log_sender, log_receiver) = mpsc::channel();
        Self {
            logs: VecDeque::with_capacity(1000),
            style: ConsoleStyle::default(),
            visible: true,
            handler: Box::new(DefaultHandler),
            command_history: VecDeque::new(),
            history_index: 0,
            current_input: String::new(),
            show_command_prompt: true,
            log_sender,
            log_receiver,
            background_thread: None,
        }
    }

    pub fn with_handler<H: ConsoleHandler + 'static>(handler: H) -> Self {
        let (log_sender, log_receiver) = mpsc::channel();
        Self {
            logs: VecDeque::with_capacity(1000),
            style: ConsoleStyle::default(),
            visible: true,
            handler: Box::new(handler),
            command_history: VecDeque::new(),
            history_index: 0,
            current_input: String::new(),
            show_command_prompt: true,
            log_sender,
            log_receiver,
            background_thread: None,
        }
    }

    pub fn set_handler<H: ConsoleHandler + 'static>(&mut self, handler: H) {
        self.handler = Box::new(handler);
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn set_style(&mut self, style: ConsoleStyle) {
        self.style = style;
    }

    pub fn get_style(&self) -> &ConsoleStyle {
        &self.style
    }

    pub fn log(&mut self, level: LogLevel, message: &str) {
        self.log_with_tag(level, message, None);
    }

    pub fn log_with_tag(&mut self, level: LogLevel, message: &str, tag: Option<&str>) {
        let entry = LogEntry::new(
            level,
            message.to_string(),
            tag.map(|s| s.to_string()),
        );

        self.logs.push_back(entry);
        if self.logs.len() > self.style.max_lines {
            self.logs.pop_front();
        }

        if self.log_sender.send(self.logs.back().unwrap().clone()).is_err() {
            // Receiver disconnected
        }
    }

    pub fn debug(&mut self, message: &str) {
        self.log(LogLevel::Debug, message);
    }

    pub fn info(&mut self, message: &str) {
        self.log(LogLevel::Info, message);
    }

    pub fn warning(&mut self, message: &str) {
        self.log(LogLevel::Warning, message);
    }

    pub fn error(&mut self, message: &str) {
        self.log(LogLevel::Error, message);
    }

    pub fn critical(&mut self, message: &str) {
        self.log(LogLevel::Critical, message);
    }

    pub fn log_fmt(&mut self, level: LogLevel, tag: Option<&str>, fmt: std::fmt::Arguments) {
        let message = fmt.to_string();
        self.log_with_tag(level, &message, tag);
    }

    pub fn get_logs(&self) -> &VecDeque<LogEntry> {
        &self.logs
    }

    pub fn get_recent_logs(&self, count: usize) -> Vec<&LogEntry> {
        self.logs.iter().rev().take(count).collect()
    }

    pub fn get_filtered_logs(&self, min_level: LogLevel) -> Vec<&LogEntry> {
        fn level_weight(level: &LogLevel) -> u8 {
            match level {
                LogLevel::Debug => 0,
                LogLevel::Info => 1,
                LogLevel::Warning => 2,
                LogLevel::Error => 3,
                LogLevel::Critical => 4,
            }
        }
        let min_weight = level_weight(&min_level);
        self.logs.iter()
            .filter(|e| level_weight(&e.level) >= min_weight)
            .collect()
    }

    pub fn clear(&mut self) {
        self.logs.clear();
    }

    pub fn set_input(&mut self, input: &str) {
        self.current_input = input.to_string();
        self.history_index = self.command_history.len();
    }

    pub fn get_input(&self) -> &str {
        &self.current_input
    }

    pub fn execute_command(&mut self, command: &str) -> CommandResult {
        if command.trim().is_empty() {
            return CommandResult::new(String::new(), true);
        }

        self.command_history.push_back(command.to_string());
        if self.command_history.len() > 100 {
            self.command_history.pop_front();
        }
        self.history_index = self.command_history.len();
        self.current_input.clear();

        let result = self.handler.handle_command(command);
        if result.success {
            self.log(LogLevel::Info, &format!("> {}", command));
        } else {
            self.log(LogLevel::Warning, &format!("> {}", command));
        }
        self.log(LogLevel::Info, &result.output);

        result
    }

    pub fn history_up(&mut self) -> Option<&String> {
        if !self.command_history.is_empty() && self.history_index > 0 {
            self.history_index -= 1;
            Some(&self.command_history[self.history_index])
        } else {
            None
        }
    }

    pub fn history_down(&mut self) -> Option<&String> {
        if self.history_index < self.command_history.len().saturating_sub(1) {
            self.history_index += 1;
            Some(&self.command_history[self.history_index])
        } else {
            self.history_index = self.command_history.len();
            None
        }
    }

    pub fn set_show_command_prompt(&mut self, show: bool) {
        self.show_command_prompt = show;
    }

    pub fn should_auto_scroll(&self) -> bool {
        self.style.auto_scroll
    }

    pub fn get_max_lines(&self) -> usize {
        self.style.max_lines
    }

    pub fn start_background_processing(&mut self) {
        // Note: mpsc::Receiver cannot be cloned, so we store the sender
        // and process logs synchronously or use a different approach for async
    }

    pub fn stop_background_processing(&mut self) {
        self.background_thread.take().map(|t| t.join());
    }

    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        report.push_str("=== Debug Console Report ===\n");
        report.push_str(&format!("Visible: {}\n", self.visible));
        report.push_str(&format!("Total logs: {}\n", self.logs.len()));
        report.push_str(&format!("Command history: {}\n", self.command_history.len()));

        let error_count = self.logs.iter().filter(|e| matches!(e.level, LogLevel::Error | LogLevel::Critical)).count();
        report.push_str(&format!("Errors: {}\n", error_count));

        report.push_str("\n=== Recent Logs ===\n");
        for entry in self.get_recent_logs(10) {
            report.push_str(&entry.formatted());
            report.push('\n');
        }

        report
    }
}

impl Default for DebugConsole {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for DebugConsole {
    fn drop(&mut self) {
        self.stop_background_processing();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestHandler {
        pub command_count: usize,
    }

    impl ConsoleHandler for TestHandler {
        fn handle_command(&mut self, command: &str) -> CommandResult {
            self.command_count += 1;
            match command {
                "test" => CommandResult::new("Test successful".to_string(), true),
                "fail" => CommandResult::new("Test failed".to_string(), false),
                _ => CommandResult::new("Unknown".to_string(), false),
            }
        }

        fn get_help(&self) -> String {
            "Test help".to_string()
        }
    }

    #[test]
    fn test_console_creation() {
        let console = DebugConsole::new();
        assert!(console.is_visible());
        assert!(console.get_input().is_empty());
    }

    #[test]
    fn test_console_logging() {
        let mut console = DebugConsole::new();
        console.debug("Debug message");
        console.info("Info message");
        console.warning("Warning message");
        console.error("Error message");
        console.critical("Critical message");

        assert_eq!(console.logs.len(), 5);
    }

    #[test]
    fn test_console_log_with_tag() {
        let mut console = DebugConsole::new();
        console.log_with_tag(LogLevel::Info, "Test message", Some("TAG"));

        let entry = console.logs.back().unwrap();
        assert_eq!(entry.tag, Some("TAG".to_string()));
        assert_eq!(entry.message, "Test message");
    }

    #[test]
    fn test_console_filtered_logs() {
        let mut console = DebugConsole::new();
        console.debug("Debug 1");
        console.info("Info 1");
        console.error("Error 1");
        console.debug("Debug 2");

        let filtered = console.get_filtered_logs(LogLevel::Warning);
        assert_eq!(filtered.len(), 1);
        assert!(matches!(filtered[0].level, LogLevel::Error));
    }

    #[test]
    fn test_console_clear() {
        let mut console = DebugConsole::new();
        console.info("Test");
        console.clear();
        assert!(console.logs.is_empty());
    }

    #[test]
    fn test_console_command_execution() {
        let mut console = DebugConsole::with_handler(TestHandler { command_count: 0 });

        let result = console.execute_command("test");
        assert!(result.success);
        assert_eq!(result.output, "Test successful");

        let result = console.execute_command("fail");
        assert!(!result.success);
    }

    #[test]
    fn test_console_command_history() {
        let mut console = DebugConsole::new();
        console.execute_command("cmd1");
        console.execute_command("cmd2");
        console.execute_command("cmd3");

        assert_eq!(console.command_history.len(), 3);

        let _ = console.history_up();
        let _ = console.history_up();
        let cmd = console.history_up();

        if let Some(s) = cmd {
            assert_eq!(s, "cmd1");
        }
    }

    #[test]
    fn test_console_input() {
        let mut console = DebugConsole::new();
        console.set_input("test input");
        assert_eq!(console.get_input(), "test input");
    }

    #[test]
    fn test_console_report() {
        let mut console = DebugConsole::new();
        console.info("Test 1");
        console.error("Error");
        console.info("Test 2");

        let report = console.generate_report();
        assert!(report.contains("Debug Console Report"));
        assert!(report.contains("Total logs: 3"));
        assert!(report.contains("Errors: 1"));
    }

    #[test]
    fn test_console_style() {
        let console = DebugConsole::new();
        let style = console.get_style();
        assert_eq!(style.max_lines, 1000);
        assert!(style.auto_scroll);
    }
}

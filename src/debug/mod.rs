/// Debug tools module - 调试工具模块
///
/// 提供开发者工具：控制台、统计、性能分析

pub mod console;
pub mod stats;
pub mod profiler;

pub use console::{Console, LogLevel, LogEntry};
pub use stats::{Stats, StatsSnapshot};
pub use profiler::{Profiler, FrameProfile, ScopeProfile};

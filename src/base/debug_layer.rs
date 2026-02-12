/// DebugLayer - 调试 UI 层
///
/// 功能：
/// - 集成 DebugStats、DebugConsole、DebugProfiler
/// - 可视化调试信息面板
/// - 可折叠/拖拽的调试窗口
/// - FPS 曲线绘制
/// - 性能图表显示

use std::time::{Instant, Duration};
use std::collections::VecDeque;

use crate::base::DebugStats;
use crate::base::debug_console::DebugConsole;
use crate::base::debug_profiler::DebugProfiler;
use crate::base::types::{Size, Color4B};

#[derive(Clone, Copy, Debug, PartialEq)]
#[derive(Default)]
pub enum DebugPanel {
    Stats,
    Console,
    Profiler,
    #[default]
    All,
}


#[derive(Clone, Copy, Debug)]
pub struct DebugPosition {
    pub x: f32,
    pub y: f32,
}

impl Default for DebugPosition {
    fn default() -> Self {
        Self { x: 10.0, y: 10.0 }
    }
}

#[derive(Clone, Debug)]
pub struct DebugPanelConfig {
    pub position: DebugPosition,
    pub size: Size,
    pub background_color: Color4B,
    pub text_color: Color4B,
    pub font_size: f32,
    pub opacity: u8,
    pub visible: bool,
    pub expanded: bool,
    pub title: String,
}

impl Default for DebugPanelConfig {
    fn default() -> Self {
        Self {
            position: DebugPosition::default(),
            size: Size::new(300.0, 200.0),
            background_color: Color4B::new(0, 0, 0, 180),
            text_color: Color4B::new(255, 255, 255, 255),
            font_size: 14.0,
            opacity: 180,
            visible: true,
            expanded: true,
            title: String::new(),
        }
    }
}

struct FpsHistory {
    values: VecDeque<f64>,
    max_samples: usize,
}

impl FpsHistory {
    fn new(max_samples: usize) -> Self {
        Self {
            values: VecDeque::with_capacity(max_samples),
            max_samples,
        }
    }

    fn push(&mut self, value: f64) {
        self.values.push_back(value);
        if self.values.len() > self.max_samples {
            self.values.pop_front();
        }
    }

    fn get_all(&self) -> Vec<f64> {
        self.values.iter().copied().collect()
    }

    fn avg(&self) -> f64 {
        if self.values.is_empty() {
            0.0
        } else {
            self.values.iter().sum::<f64>() / self.values.len() as f64
        }
    }

    fn min(&self) -> f64 {
        self.values.iter().copied().fold(f64::MAX, f64::min)
    }

    fn max(&self) -> f64 {
        self.values.iter().copied().fold(f64::MIN, f64::max)
    }
}

struct FrameTimeHistory {
    values: VecDeque<f64>,
    max_samples: usize,
}

impl FrameTimeHistory {
    fn new(max_samples: usize) -> Self {
        Self {
            values: VecDeque::with_capacity(max_samples),
            max_samples,
        }
    }

    fn push(&mut self, value: f64) {
        self.values.push_back(value);
        if self.values.len() > self.max_samples {
            self.values.pop_front();
        }
    }

    fn get_all(&self) -> Vec<f64> {
        self.values.iter().copied().collect()
    }
}

pub struct DebugLayer {
    stats: DebugStats,
    console: DebugConsole,
    profiler: DebugProfiler,
    config: DebugPanelConfig,
    fps_history: FpsHistory,
    frame_time_history: FrameTimeHistory,
    last_update: Instant,
    update_interval: Duration,
    active_panel: DebugPanel,
    show_fps_graph: bool,
    show_memory_graph: bool,
    show_call_graph: bool,
}

impl DebugLayer {
    pub fn new() -> Self {
        Self {
            stats: DebugStats::new(),
            console: DebugConsole::new(),
            profiler: DebugProfiler::new(),
            config: DebugPanelConfig::default(),
            fps_history: FpsHistory::new(120),
            frame_time_history: FrameTimeHistory::new(120),
            last_update: Instant::now(),
            update_interval: Duration::from_millis(100),
            active_panel: DebugPanel::All,
            show_fps_graph: true,
            show_memory_graph: false,
            show_call_graph: false,
        }
    }

    pub fn with_console<H: crate::base::debug_console::ConsoleHandler + 'static>(handler: H) -> Self {
        Self {
            stats: DebugStats::new(),
            console: DebugConsole::with_handler(handler),
            profiler: DebugProfiler::new(),
            config: DebugPanelConfig::default(),
            fps_history: FpsHistory::new(120),
            frame_time_history: FrameTimeHistory::new(120),
            last_update: Instant::now(),
            update_interval: Duration::from_millis(100),
            active_panel: DebugPanel::All,
            show_fps_graph: true,
            show_memory_graph: false,
            show_call_graph: false,
        }
    }

    pub fn get_stats(&mut self) -> &mut DebugStats {
        &mut self.stats
    }

    pub fn get_console(&mut self) -> &mut DebugConsole {
        &mut self.console
    }

    pub fn get_profiler(&mut self) -> &mut DebugProfiler {
        &mut self.profiler
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.config.visible = visible;
    }

    pub fn is_visible(&self) -> bool {
        self.config.visible
    }

    pub fn set_panel(&mut self, panel: DebugPanel) {
        self.active_panel = panel;
    }

    pub fn get_panel(&self) -> DebugPanel {
        self.active_panel
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.config.position.x = x;
        self.config.position.y = y;
    }

    pub fn get_position(&self) -> (f32, f32) {
        (self.config.position.x, self.config.position.y)
    }

    pub fn set_size(&mut self, width: f32, height: f32) {
        self.config.size.width = width;
        self.config.size.height = height;
    }

    pub fn get_size(&self) -> Size {
        self.config.size
    }

    pub fn set_show_fps_graph(&mut self, show: bool) {
        self.show_fps_graph = show;
    }

    pub fn set_show_memory_graph(&mut self, show: bool) {
        self.show_memory_graph = show;
    }

    pub fn set_show_call_graph(&mut self, show: bool) {
        self.show_call_graph = show;
    }

    pub fn set_background_color(&mut self, color: Color4B) {
        self.config.background_color = color;
        self.config.opacity = color.a;
    }

    pub fn set_text_color(&mut self, color: Color4B) {
        self.config.text_color = color;
    }

    pub fn set_font_size(&mut self, size: f32) {
        self.config.font_size = size;
    }

    pub fn set_opacity(&mut self, opacity: u8) {
        self.config.opacity = opacity;
        self.config.background_color.a = opacity;
    }

    pub fn toggle_console(&mut self) {
        if matches!(self.active_panel, DebugPanel::Console) {
            self.active_panel = DebugPanel::All;
        } else {
            self.active_panel = DebugPanel::Console;
        }
    }

    pub fn toggle_stats(&mut self) {
        if matches!(self.active_panel, DebugPanel::Stats) {
            self.active_panel = DebugPanel::All;
        } else {
            self.active_panel = DebugPanel::Stats;
        }
    }

    pub fn toggle_profiler(&mut self) {
        if matches!(self.active_panel, DebugPanel::Profiler) {
            self.active_panel = DebugPanel::All;
        } else {
            self.active_panel = DebugPanel::Profiler;
        }
    }

    pub fn toggle_expand(&mut self) {
        self.config.expanded = !self.config.expanded;
    }

    pub fn begin_frame(&mut self) {
        self.stats.begin_frame();
        self.profiler.begin_frame();
    }

    pub fn end_frame(&mut self) {
        self.stats.end_frame();
        self.profiler.end_frame();

        self.fps_history.push(self.stats.get_fps());
        self.frame_time_history.push(self.stats.get_frame_time() * 1000.0);
    }

    pub fn log(&mut self, message: &str) {
        self.console.log(crate::base::debug_console::LogLevel::Info, message);
    }

    pub fn log_debug(&mut self, message: &str) {
        self.console.log(crate::base::debug_console::LogLevel::Debug, message);
    }

    pub fn log_error(&mut self, message: &str) {
        self.console.log(crate::base::debug_console::LogLevel::Error, message);
    }

    pub fn log_warning(&mut self, message: &str) {
        self.console.log(crate::base::debug_console::LogLevel::Warning, message);
    }

    pub fn profile_scope(&mut self, name: &str) -> crate::base::debug_profiler::ProfilerScope {
        self.profiler.scope(name)
    }

    pub fn set_memory_usage(&mut self, bytes: u64) {
        self.stats.set_memory_usage(bytes);
    }

    pub fn set_draw_calls(&mut self, count: u32) {
        self.stats.set_draw_calls(count);
    }

    pub fn set_triangles(&mut self, count: u32) {
        self.stats.set_triangles(count);
    }

    pub fn set_vertices(&mut self, count: u32) {
        self.stats.set_vertices(count);
    }

    pub fn get_fps_history(&self) -> Vec<f64> {
        self.fps_history.get_all()
    }

    pub fn get_frame_time_history(&self) -> Vec<f64> {
        self.frame_time_history.get_all()
    }

    pub fn get_avg_fps(&self) -> f64 {
        self.fps_history.avg()
    }

    pub fn get_min_fps(&self) -> f64 {
        let min = self.fps_history.min();
        if min == f64::MAX { 0.0 } else { min }
    }

    pub fn get_max_fps(&self) -> f64 {
        let max = self.fps_history.max();
        if max == f64::MIN { 0.0 } else { max }
    }

    pub fn get_avg_frame_time(&self) -> f64 {
        self.frame_time_history.get_all().iter().sum::<f64>() / 
            self.frame_time_history.get_all().len().max(1) as f64
    }

    pub fn is_expanded(&self) -> bool {
        self.config.expanded
    }

    pub fn clear_console(&mut self) {
        self.console.clear();
    }

    pub fn clear_profiler(&mut self) {
        self.profiler.clear();
    }

    pub fn clear_all(&mut self) {
        self.clear_console();
        self.clear_profiler();
        self.stats.reset();
        self.fps_history = FpsHistory::new(120);
        self.frame_time_history = FrameTimeHistory::new(120);
    }

    pub fn generate_full_report(&self) -> String {
        let mut report = String::new();

        report.push_str("=== Debug Layer Full Report ===\n\n");

        report.push_str("=== Debug Stats ===\n");
        report.push_str(&self.stats.generate_report());
        report.push('\n');

        report.push_str("=== Debug Console ===\n");
        report.push_str(&self.console.generate_report());
        report.push('\n');

        report.push_str("=== Debug Profiler ===\n");
        report.push_str(&self.profiler.generate_report());

        report
    }

    pub fn generate_summary(&self) -> String {
        format!(
            "FPS: {:.1} (min: {:.1}, max: {:.1}) | Frame: {:.2}ms | DrawCalls: {} | Memory: {}",
            self.get_avg_fps(),
            self.get_min_fps(),
            self.get_max_fps(),
            self.get_avg_frame_time(),
            self.stats.get_draw_calls(),
            self.stats.get_memory_usage_string()
        )
    }
}

impl Default for DebugLayer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_debug_layer_creation() {
        let layer = DebugLayer::new();
        assert!(layer.is_visible());
        assert!(layer.is_expanded());
    }

    #[test]
    fn test_debug_layer_visibility() {
        let mut layer = DebugLayer::new();
        layer.set_visible(false);
        assert!(!layer.is_visible());

        layer.set_visible(true);
        assert!(layer.is_visible());
    }

    #[test]
    fn test_debug_layer_position() {
        let mut layer = DebugLayer::new();
        layer.set_position(100.0, 200.0);
        assert_eq!(layer.get_position(), (100.0, 200.0));
    }

    #[test]
    fn test_debug_layer_size() {
        let mut layer = DebugLayer::new();
        layer.set_size(400.0, 300.0);
        let size = layer.get_size();
        assert_eq!(size.width, 400.0);
        assert_eq!(size.height, 300.0);
    }

    #[test]
    fn test_debug_layer_frame() {
        let mut layer = DebugLayer::new();

        layer.begin_frame();
        thread::sleep(Duration::from_millis(10));
        layer.end_frame();

        assert!(layer.get_avg_fps() > 0.0);
    }

    #[test]
    fn test_debug_layer_logging() {
        let mut layer = DebugLayer::new();
        layer.log("Test message");
        layer.log_debug("Debug message");
        layer.log_error("Error message");
        layer.log_warning("Warning message");

        assert_eq!(layer.console.get_logs().len(), 4);
    }

    #[test]
    fn test_debug_layer_profiling() {
        let mut layer = DebugLayer::new();

        layer.begin_frame();
        {
            let _s = layer.profile_scope("test_scope");
            thread::sleep(Duration::from_millis(5));
        }
        layer.end_frame();

        let entry = layer.profiler.get_entry("test_scope");
        assert!(entry.is_some());
    }

    #[test]
    fn test_debug_layer_panel_toggle() {
        let mut layer = DebugLayer::new();

        assert_eq!(layer.get_panel(), DebugPanel::All);

        layer.set_panel(DebugPanel::Stats);
        assert_eq!(layer.get_panel(), DebugPanel::Stats);

        layer.toggle_console();
        assert_eq!(layer.get_panel(), DebugPanel::Console);
    }

    #[test]
    fn test_debug_layer_expand() {
        let mut layer = DebugLayer::new();
        assert!(layer.is_expanded());

        layer.toggle_expand();
        assert!(!layer.is_expanded());
    }

    #[test]
    fn test_debug_layer_fps_history() {
        let mut layer = DebugLayer::new();

        for _ in 0..10 {
            layer.begin_frame();
            layer.end_frame();
        }

        let history = layer.get_fps_history();
        assert_eq!(history.len(), 10);
        assert!(layer.get_avg_fps() > 0.0);
    }

    #[test]
    fn test_debug_layer_clear() {
        let mut layer = DebugLayer::new();

        layer.log("Test");
        layer.begin_frame();
        layer.profile_scope("test");
        layer.end_frame();

        layer.clear_all();

        assert!(layer.console.get_logs().is_empty());
        assert!(layer.profiler.get_all_entries().is_empty());
    }

    #[test]
    fn test_debug_layer_summary() {
        let mut layer = DebugLayer::new();

        layer.begin_frame();
        layer.end_frame();

        let summary = layer.generate_summary();
        assert!(summary.contains("FPS:"));
        assert!(summary.contains("Frame:"));
    }

    #[test]
    fn test_debug_layer_full_report() {
        let layer = DebugLayer::new();

        let report = layer.generate_full_report();
        assert!(report.contains("Debug Layer Full Report"));
        assert!(report.contains("Debug Stats"));
        assert!(report.contains("Debug Console"));
        assert!(report.contains("Debug Profiler"));
    }

    #[test]
    fn test_debug_layer_graph_settings() {
        let mut layer = DebugLayer::new();

        layer.set_show_fps_graph(false);
        layer.set_show_memory_graph(true);
        layer.set_show_call_graph(true);

        // These are just settings, verify they don't crash
        let _ = layer.get_fps_history();
        let _ = layer.get_frame_time_history();
    }
}

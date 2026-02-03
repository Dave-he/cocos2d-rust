/// DebugStats - 性能统计和调试信息显示
/// 
/// 功能：
/// - FPS 帧率统计
/// - 帧时间统计
/// - 渲染对象计数
/// - 内存使用估算
/// - 自定义统计项
/// - 控制台输出或 UI 显示

use std::time::{Instant, Duration};
use std::collections::HashMap;

/// 统计项类型
#[derive(Debug, Clone)]
pub enum StatValue {
    Integer(i64),
    Float(f64),
    String(String),
    Percentage(f64),
}

/// 调试统计信息
#[derive(Debug)]
pub struct DebugStats {
    // 帧统计
    frame_count: u64,
    fps: f64,
    fps_history: Vec<f64>,
    fps_history_size: usize,
    
    // 时间统计
    last_frame_time: Instant,
    frame_time: f64,
    frame_time_history: Vec<f64>,
    min_frame_time: f64,
    max_frame_time: f64,
    avg_frame_time: f64,
    
    // 渲染统计
    draw_calls: u32,
    triangles: u32,
    vertices: u32,
    
    // 内存统计
    memory_usage: u64,
    
    // 自定义统计
    custom_stats: HashMap<String, StatValue>,
    
    // 启用状态
    enabled: bool,
    visible: bool,
    
    // 更新间隔
    update_interval: Duration,
    last_update: Instant,
}

impl DebugStats {
    /// 创建新的调试统计
    pub fn new() -> Self {
        Self {
            frame_count: 0,
            fps: 0.0,
            fps_history: Vec::with_capacity(60),
            fps_history_size: 60,
            last_frame_time: Instant::now(),
            frame_time: 0.0,
            frame_time_history: Vec::with_capacity(60),
            min_frame_time: f64::MAX,
            max_frame_time: 0.0,
            avg_frame_time: 0.0,
            draw_calls: 0,
            triangles: 0,
            vertices: 0,
            memory_usage: 0,
            custom_stats: HashMap::new(),
            enabled: true,
            visible: true,
            update_interval: Duration::from_millis(500),
            last_update: Instant::now(),
        }
    }
    
    /// 启用/禁用统计
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        if enabled {
            self.last_frame_time = Instant::now();
            self.last_update = Instant::now();
        }
    }
    
    /// 是否启用
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    /// 显示/隐藏
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
    
    /// 是否可见
    pub fn is_visible(&self) -> bool {
        self.visible && self.enabled
    }
    
    /// 设置 FPS 历史记录大小
    pub fn set_fps_history_size(&mut self, size: usize) {
        self.fps_history_size = size;
        self.fps_history.truncate(size.min(self.fps_history.len()));
    }
    
    /// 设置更新间隔
    pub fn set_update_interval(&mut self, interval: Duration) {
        self.update_interval = interval;
    }
    
    /// 开始新帧
    pub fn begin_frame(&mut self) {
        if !self.enabled {
            return;
        }
        
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_frame_time);
        self.frame_time = elapsed.as_secs_f64();
        self.last_frame_time = now;
        
        // 更新 FPS
        self.frame_count += 1;
        if self.frame_time > 0.0 {
            let current_fps = 1.0 / self.frame_time;
            self.fps_history.push(current_fps);
            if self.fps_history.len() > self.fps_history_size {
                self.fps_history.remove(0);
            }
            
            // 计算平均 FPS
            if !self.fps_history.is_empty() {
                self.fps = self.fps_history.iter().sum::<f64>() / self.fps_history.len() as f64;
            }
        }
        
        // 更新帧时间统计
        self.frame_time_history.push(self.frame_time);
        if self.frame_time_history.len() > self.fps_history_size {
            self.frame_time_history.remove(0);
        }
        
        if self.frame_time > 0.0 {
            self.min_frame_time = self.min_frame_time.min(self.frame_time);
            self.max_frame_time = self.max_frame_time.max(self.frame_time);
            
            if !self.frame_time_history.is_empty() {
                self.avg_frame_time = self.frame_time_history.iter().sum::<f64>() / self.frame_time_history.len() as f64;
            }
        }
    }
    
    /// 结束帧
    pub fn end_frame(&mut self) {
        if !self.enabled {
            return;
        }
        
        // 检查是否需要更新
        let now = Instant::now();
        if now.duration_since(self.last_update) >= self.update_interval {
            self.last_update = now;
        }
    }
    
    // ===== 渲染统计 =====
    
    /// 增加绘制调用计数
    pub fn add_draw_call(&mut self, count: u32) {
        if self.enabled {
            self.draw_calls += count;
        }
    }
    
    /// 设置绘制调用计数
    pub fn set_draw_calls(&mut self, count: u32) {
        if self.enabled {
            self.draw_calls = count;
        }
    }
    
    /// 获取绘制调用计数
    pub fn get_draw_calls(&self) -> u32 {
        self.draw_calls
    }
    
    /// 增加三角形计数
    pub fn add_triangles(&mut self, count: u32) {
        if self.enabled {
            self.triangles += count;
        }
    }
    
    /// 设置三角形计数
    pub fn set_triangles(&mut self, count: u32) {
        if self.enabled {
            self.triangles = count;
        }
    }
    
    /// 获取三角形计数
    pub fn get_triangles(&self) -> u32 {
        self.triangles
    }
    
    /// 增加顶点计数
    pub fn add_vertices(&mut self, count: u32) {
        if self.enabled {
            self.vertices += count;
        }
    }
    
    /// 设置顶点计数
    pub fn set_vertices(&mut self, count: u32) {
        if self.enabled {
            self.vertices = count;
        }
    }
    
    /// 获取顶点计数
    pub fn get_vertices(&self) -> u32 {
        self.vertices
    }
    
    // ===== 内存统计 =====
    
    /// 设置内存使用量（字节）
    pub fn set_memory_usage(&mut self, bytes: u64) {
        if self.enabled {
            self.memory_usage = bytes;
        }
    }
    
    /// 获取内存使用量
    pub fn get_memory_usage(&self) -> u64 {
        self.memory_usage
    }
    
    /// 获取格式化内存大小
    pub fn get_memory_usage_string(&self) -> String {
        let bytes = self.memory_usage;
        if bytes < 1024 {
            format!("{} B", bytes)
        } else if bytes < 1024 * 1024 {
            format!("{:.1} KB", bytes as f64 / 1024.0)
        } else if bytes < 1024 * 1024 * 1024 {
            format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
        } else {
            format!("{:.2} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
        }
    }
    
    // ===== 自定义统计 =====
    
    /// 设置自定义统计值
    pub fn set_stat(&mut self, name: &str, value: StatValue) {
        if self.enabled {
            self.custom_stats.insert(name.to_string(), value);
        }
    }
    
    /// 设置整数统计
    pub fn set_int_stat(&mut self, name: &str, value: i64) {
        self.set_stat(name, StatValue::Integer(value));
    }
    
    /// 设置浮点数统计
    pub fn set_float_stat(&mut self, name: &str, value: f64) {
        self.set_stat(name, StatValue::Float(value));
    }
    
    /// 设置百分比统计
    pub fn set_percentage_stat(&mut self, name: &str, value: f64) {
        self.set_stat(name, StatValue::Percentage(value));
    }
    
    /// 获取自定义统计
    pub fn get_stat(&self, name: &str) -> Option<&StatValue> {
        self.custom_stats.get(name)
    }
    
    /// 获取所有自定义统计
    pub fn get_all_stats(&self) -> &HashMap<String, StatValue> {
        &self.custom_stats
    }
    
    // ===== 获取统计信息 =====
    
    /// 获取当前 FPS
    pub fn get_fps(&self) -> f64 {
        self.fps
    }
    
    /// 获取帧时间（秒）
    pub fn get_frame_time(&self) -> f64 {
        self.frame_time
    }
    
    /// 获取最小帧时间
    pub fn get_min_frame_time(&self) -> f64 {
        if self.min_frame_time == f64::MAX { 0.0 } else { self.min_frame_time }
    }
    
    /// 获取最大帧时间
    pub fn get_max_frame_time(&self) -> f64 {
        self.max_frame_time
    }
    
    /// 获取平均帧时间
    pub fn get_avg_frame_time(&self) -> f64 {
        self.avg_frame_time
    }
    
    /// 获取帧计数
    pub fn get_frame_count(&self) -> u64 {
        self.frame_count
    }
    
    /// 获取帧时间字符串（毫秒）
    pub fn get_frame_time_string(&self) -> String {
        format!("{:.2} ms", self.frame_time * 1000.0)
    }
    
    /// 获取 FPS 字符串
    pub fn get_fps_string(&self) -> String {
        format!("{:.1}", self.fps)
    }
    
    /// 重置统计
    pub fn reset(&mut self) {
        self.frame_count = 0;
        self.fps = 0.0;
        self.fps_history.clear();
        self.frame_time = 0.0;
        self.frame_time_history.clear();
        self.min_frame_time = f64::MAX;
        self.max_frame_time = 0.0;
        self.avg_frame_time = 0.0;
        self.draw_calls = 0;
        self.triangles = 0;
        self.vertices = 0;
        self.memory_usage = 0;
        self.custom_stats.clear();
        self.last_frame_time = Instant::now();
        self.last_update = Instant::now();
    }
    
    /// 生成报告字符串
    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        
        report.push_str("=== Debug Stats ===\n");
        report.push_str(&format!("FPS: {:.1}\n", self.fps));
        report.push_str(&format!("Frame Time: {:.2} ms\n", self.frame_time * 1000.0));
        report.push_str(&format!("Min/Max Frame Time: {:.2}/{:.2} ms\n", 
            self.get_min_frame_time() * 1000.0, 
            self.get_max_frame_time() * 1000.0));
        report.push_str(&format!("Avg Frame Time: {:.2} ms\n", self.avg_frame_time * 1000.0));
        report.push_str(&format!("Frame Count: {}\n", self.frame_count));
        report.push_str(&format!("Draw Calls: {}\n", self.draw_calls));
        report.push_str(&format!("Triangles: {}\n", self.triangles));
        report.push_str(&format!("Vertices: {}\n", self.vertices));
        report.push_str(&format!("Memory: {}\n", self.get_memory_usage_string()));
        
        if !self.custom_stats.is_empty() {
            report.push_str("\nCustom Stats:\n");
            for (name, value) in &self.custom_stats {
                match value {
                    StatValue::Integer(v) => report.push_str(&format!("{}: {}\n", name, v)),
                    StatValue::Float(v) => report.push_str(&format!("{}: {:.2}\n", name, v)),
                    StatValue::String(s) => report.push_str(&format!("{}: {}\n", name, s)),
                    StatValue::Percentage(v) => report.push_str(&format!("{}: {:.1}%\n", name, v)),
                }
            }
        }
        
        report
    }
}

impl Default for DebugStats {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_debug_stats_creation() {
        let stats = DebugStats::new();
        assert!(stats.is_enabled());
        assert!(stats.is_visible());
        assert_eq!(stats.get_fps(), 0.0);
    }

    #[test]
    fn test_debug_stats_enable_disable() {
        let mut stats = DebugStats::new();
        
        stats.set_enabled(false);
        assert!(!stats.is_enabled());
        
        stats.set_enabled(true);
        assert!(stats.is_enabled());
    }

    #[test]
    fn test_debug_stats_visible() {
        let mut stats = DebugStats::new();
        
        stats.set_visible(false);
        assert!(!stats.is_visible());
        
        stats.set_visible(true);
        assert!(stats.is_visible());
    }

    #[test]
    fn test_debug_stats_frame() {
        let mut stats = DebugStats::new();
        
        stats.begin_frame();
        thread::sleep(Duration::from_millis(16)); // ~60fps
        stats.end_frame();
        
        stats.begin_frame();
        thread::sleep(Duration::from_millis(16));
        stats.end_frame();
        
        assert_eq!(stats.get_frame_count(), 2);
        assert!(stats.get_fps() > 0.0);
    }

    #[test]
    fn test_debug_stats_draw_calls() {
        let mut stats = DebugStats::new();
        
        stats.add_draw_call(10);
        assert_eq!(stats.get_draw_calls(), 10);
        
        stats.add_draw_call(5);
        assert_eq!(stats.get_draw_calls(), 15);
        
        stats.set_draw_calls(100);
        assert_eq!(stats.get_draw_calls(), 100);
    }

    #[test]
    fn test_debug_stats_triangles() {
        let mut stats = DebugStats::new();
        
        stats.add_triangles(1000);
        assert_eq!(stats.get_triangles(), 1000);
        
        stats.set_triangles(500);
        assert_eq!(stats.get_triangles(), 500);
    }

    #[test]
    fn test_debug_stats_memory() {
        let mut stats = DebugStats::new();
        
        stats.set_memory_usage(1024 * 1024); // 1 MB
        assert_eq!(stats.get_memory_usage(), 1024 * 1024);
        assert_eq!(stats.get_memory_usage_string(), "1.0 MB");
        
        stats.set_memory_usage(1024 * 1024 * 1024); // 1 GB
        assert_eq!(stats.get_memory_usage_string(), "1.00 GB");
    }

    #[test]
    fn test_debug_stats_custom() {
        let mut stats = DebugStats::new();
        
        stats.set_int_stat("score", 100);
        stats.set_float_stat("health", 99.5);
        stats.set_percentage_stat("battery", 75.0);
        
        match stats.get_stat("score").unwrap() {
            StatValue::Integer(v) => assert_eq!(*v, 100),
            _ => panic!("Expected Integer"),
        }
        
        match stats.get_stat("health").unwrap() {
            StatValue::Float(v) => assert!((*v - 99.5).abs() < 0.01),
            _ => panic!("Expected Float"),
        }
    }

    #[test]
    fn test_debug_stats_reset() {
        let mut stats = DebugStats::new();
        
        stats.begin_frame();
        stats.add_draw_call(10);
        stats.set_stat("test", StatValue::Integer(42));
        stats.end_frame();
        
        stats.begin_frame();
        stats.add_draw_call(20);
        stats.end_frame();
        
        assert_eq!(stats.get_frame_count(), 2);
        assert_eq!(stats.get_draw_calls(), 30);
        
        stats.reset();
        
        assert_eq!(stats.get_frame_count(), 0);
        assert_eq!(stats.get_draw_calls(), 0);
        assert!(stats.get_all_stats().is_empty());
    }

    #[test]
    fn test_debug_stats_report() {
        let mut stats = DebugStats::new();
        
        stats.begin_frame();
        stats.add_draw_call(10);
        stats.set_triangles(100);
        stats.set_memory_usage(1024 * 1024);
        stats.end_frame();
        
        let report = stats.generate_report();
        assert!(report.contains("Debug Stats"));
        assert!(report.contains("Draw Calls: 10"));
        assert!(report.contains("Triangles: 100"));
        assert!(report.contains("Memory: 1.0 MB"));
    }

    #[test]
    fn test_debug_stats_fps_history() {
        let mut stats = DebugStats::new();

        stats.set_fps_history_size(5);

        for _ in 0..10 {
            stats.begin_frame();
            std::thread::sleep(std::time::Duration::from_millis(16));
            stats.end_frame();
        }

        assert!(stats.fps_history.len() <= 5);
    }
}

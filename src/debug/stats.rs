/// Stats - 统计系统
///
/// 提供 FPS、DrawCall、内存等实时统计信息

use std::collections::VecDeque;

/// 统计数据快照
#[derive(Debug, Clone, Default)]
pub struct StatsSnapshot {
    pub fps: f32,
    pub frame_time_ms: f32,
    pub draw_calls: u32,
    pub vertices: u32,
    pub triangles: u32,
    pub texture_memory_mb: f32,
    pub node_count: u32,
    pub visible_node_count: u32,
}

/// 统计系统
pub struct Stats {
    current: StatsSnapshot,
    history: VecDeque<StatsSnapshot>,
    max_history: usize,
    frame_times: VecDeque<f32>,
    max_frame_times: usize,
    frame_count: u64,
    time_accumulator: f32,
    enabled: bool,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            current: StatsSnapshot::default(),
            history: VecDeque::new(),
            max_history: 60,
            frame_times: VecDeque::new(),
            max_frame_times: 120,
            frame_count: 0,
            time_accumulator: 0.0,
            enabled: true,
        }
    }

    pub fn with_capacity(max_history: usize, max_frame_times: usize) -> Self {
        Self {
            current: StatsSnapshot::default(),
            history: VecDeque::with_capacity(max_history),
            max_history,
            frame_times: VecDeque::with_capacity(max_frame_times),
            max_frame_times,
            frame_count: 0,
            time_accumulator: 0.0,
            enabled: true,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if !self.enabled {
            return;
        }

        self.frame_count += 1;
        self.time_accumulator += dt;

        self.frame_times.push_back(dt * 1000.0);
        if self.frame_times.len() > self.max_frame_times {
            self.frame_times.pop_front();
        }

        if self.time_accumulator >= 1.0 {
            let frame_time_ms = if !self.frame_times.is_empty() {
                self.frame_times.iter().sum::<f32>() / self.frame_times.len() as f32
            } else {
                0.0
            };

            self.current.fps = if frame_time_ms > 0.0 {
                1000.0 / frame_time_ms
            } else {
                0.0
            };

            self.current.frame_time_ms = frame_time_ms;

            self.history.push_back(self.current.clone());
            if self.history.len() > self.max_history {
                self.history.pop_front();
            }

            self.time_accumulator = 0.0;
        }
    }

    pub fn begin_frame(&mut self) {
        if !self.enabled {
            return;
        }

        self.current.draw_calls = 0;
        self.current.vertices = 0;
        self.current.triangles = 0;
    }

    pub fn record_draw_call(&mut self, vertices: u32, triangles: u32) {
        if !self.enabled {
            return;
        }

        self.current.draw_calls += 1;
        self.current.vertices += vertices;
        self.current.triangles += triangles;
    }

    pub fn record_node_count(&mut self, total: u32, visible: u32) {
        if !self.enabled {
            return;
        }

        self.current.node_count = total;
        self.current.visible_node_count = visible;
    }

    pub fn record_texture_memory(&mut self, bytes: usize) {
        if !self.enabled {
            return;
        }

        self.current.texture_memory_mb = bytes as f32 / (1024.0 * 1024.0);
    }

    pub fn get_current(&self) -> &StatsSnapshot {
        &self.current
    }

    pub fn get_history(&self) -> &VecDeque<StatsSnapshot> {
        &self.history
    }

    pub fn get_average_fps(&self, samples: usize) -> f32 {
        if self.history.is_empty() {
            return self.current.fps;
        }

        let count = samples.min(self.history.len());
        let sum: f32 = self.history.iter().rev().take(count).map(|s| s.fps).sum();
        sum / count as f32
    }

    pub fn get_average_frame_time(&self, samples: usize) -> f32 {
        if self.frame_times.is_empty() {
            return 0.0;
        }

        let count = samples.min(self.frame_times.len());
        let sum: f32 = self.frame_times.iter().rev().take(count).sum();
        sum / count as f32
    }

    pub fn get_min_fps(&self) -> f32 {
        self.history
            .iter()
            .map(|s| s.fps)
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0)
    }

    pub fn get_max_fps(&self) -> f32 {
        self.history
            .iter()
            .map(|s| s.fps)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0)
    }

    pub fn get_peak_draw_calls(&self) -> u32 {
        self.history.iter().map(|s| s.draw_calls).max().unwrap_or(0)
    }

    pub fn get_peak_vertices(&self) -> u32 {
        self.history.iter().map(|s| s.vertices).max().unwrap_or(0)
    }

    pub fn reset(&mut self) {
        self.current = StatsSnapshot::default();
        self.history.clear();
        self.frame_times.clear();
        self.frame_count = 0;
        self.time_accumulator = 0.0;
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn get_frame_count(&self) -> u64 {
        self.frame_count
    }
}

impl Default for Stats {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stats_creation() {
        let stats = Stats::new();
        assert!(stats.is_enabled());
        assert_eq!(stats.get_frame_count(), 0);
    }

    #[test]
    fn test_fps_calculation() {
        let mut stats = Stats::new();

        // Update for more than 1 second to trigger FPS calculation
        for _ in 0..120 {
            stats.update(1.0 / 60.0);
        }

        // FPS should be calculated after time_accumulator >= 1.0
        assert!(stats.get_current().fps > 0.0);
    }

    #[test]
    fn test_frame_time() {
        let mut stats = Stats::new();

        stats.update(0.016);
        stats.update(0.017);

        assert!(stats.get_average_frame_time(10) > 15.0 && stats.get_average_frame_time(10) < 18.0);
    }

    #[test]
    fn test_draw_call_recording() {
        let mut stats = Stats::new();

        stats.begin_frame();
        stats.record_draw_call(100, 50);
        stats.record_draw_call(200, 100);

        assert_eq!(stats.get_current().draw_calls, 2);
        assert_eq!(stats.get_current().vertices, 300);
        assert_eq!(stats.get_current().triangles, 150);
    }

    #[test]
    fn test_node_count() {
        let mut stats = Stats::new();
        stats.record_node_count(100, 80);

        assert_eq!(stats.get_current().node_count, 100);
        assert_eq!(stats.get_current().visible_node_count, 80);
    }

    #[test]
    fn test_texture_memory() {
        let mut stats = Stats::new();
        stats.record_texture_memory(1024 * 1024 * 10); // 10 MB

        assert!((stats.get_current().texture_memory_mb - 10.0).abs() < 0.01);
    }

    #[test]
    fn test_history() {
        let mut stats = Stats::with_capacity(5, 60);

        for _ in 0..10 {
            for _ in 0..60 {
                stats.update(1.0 / 60.0);
            }
        }

        assert_eq!(stats.get_history().len(), 5);
    }

    #[test]
    fn test_average_fps() {
        let mut stats = Stats::new();

        for _ in 0..120 {
            stats.update(1.0 / 60.0);
        }

        let avg = stats.get_average_fps(10);
        assert!(avg > 55.0 && avg < 65.0);
    }

    #[test]
    fn test_min_max_fps() {
        let mut stats = Stats::new();

        // First batch: 60 FPS
        for _ in 0..120 {
            stats.update(1.0 / 60.0);
        }

        // Second batch: 30 FPS
        for _ in 0..120 {
            stats.update(1.0 / 30.0);
        }

        let min_fps = stats.get_min_fps();
        let max_fps = stats.get_max_fps();

        // Should have recorded different FPS values
        assert!(min_fps > 0.0 || max_fps > 0.0);
    }

    #[test]
    fn test_peak_draw_calls() {
        let mut stats = Stats::new();

        stats.begin_frame();
        stats.record_draw_call(100, 50);

        for _ in 0..60 {
            stats.update(1.0 / 60.0);
        }

        stats.begin_frame();
        stats.record_draw_call(200, 100);
        stats.record_draw_call(300, 150);

        for _ in 0..60 {
            stats.update(1.0 / 60.0);
        }

        assert!(stats.get_peak_draw_calls() >= 2);
    }

    #[test]
    fn test_reset() {
        let mut stats = Stats::new();

        stats.update(0.016);
        stats.record_draw_call(100, 50);

        stats.reset();

        assert_eq!(stats.get_frame_count(), 0);
        assert_eq!(stats.get_current().draw_calls, 0);
        assert_eq!(stats.get_history().len(), 0);
    }

    #[test]
    fn test_enable_disable() {
        let mut stats = Stats::new();

        stats.set_enabled(false);
        assert!(!stats.is_enabled());

        stats.update(0.016);
        assert_eq!(stats.get_frame_count(), 0);

        stats.set_enabled(true);
        stats.update(0.016);
        assert_eq!(stats.get_frame_count(), 1);
    }

    #[test]
    fn test_zero_frame_time() {
        let mut stats = Stats::new();
        stats.update(0.0);

        assert_eq!(stats.get_current().fps, 0.0);
    }

    #[test]
    fn test_large_frame_count() {
        let mut stats = Stats::new();

        for _ in 0..10000 {
            stats.update(0.016);
        }

        assert!(stats.get_frame_count() >= 10000);
    }

    #[test]
    fn test_frame_times_limit() {
        let mut stats = Stats::with_capacity(60, 10);

        for _ in 0..20 {
            stats.update(0.016);
        }

        assert!(stats.frame_times.len() <= 10);
    }

    #[test]
    fn test_empty_history_average() {
        let stats = Stats::new();
        let avg = stats.get_average_fps(10);
        assert_eq!(avg, 0.0);
    }

    #[test]
    fn test_peak_vertices() {
        let mut stats = Stats::new();

        stats.begin_frame();
        stats.record_draw_call(1000, 500);

        // Update for enough time to record stats
        for _ in 0..120 {
            stats.update(1.0 / 60.0);
        }

        // Peak should be recorded
        assert!(stats.get_peak_vertices() > 0);
    }

    #[test]
    fn test_multiple_updates_per_second() {
        let mut stats = Stats::new();

        // Simulate 125 FPS (8ms per frame)
        for _ in 0..250 {
            stats.update(0.008);
        }

        // Should record FPS after accumulating >= 1 second
        assert!(stats.get_current().fps > 0.0);
    }

    #[test]
    fn test_precision() {
        let mut stats = Stats::new();

        // Simulate exactly 60 FPS for 2 seconds
        for _ in 0..120 {
            stats.update(1.0 / 60.0);
        }

        // Should have recorded FPS
        let fps = stats.get_current().fps;
        assert!(fps > 0.0);
    }
}

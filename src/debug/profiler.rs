/// Profiler - 性能分析器
///
/// 提供函数调用计时、作用域分析和热点识别

use std::collections::{HashMap, VecDeque};
use std::time::Instant;

/// 性能作用域
#[derive(Debug)]
struct ProfileScope {
    name: String,
    start_time: Instant,
}

/// 作用域性能数据
#[derive(Debug, Clone)]
pub struct ScopeProfile {
    pub name: String,
    pub duration_ms: f32,
    pub call_count: u32,
    pub percentage: f32,
    pub children: Vec<ScopeProfile>,
}

/// 帧性能分析
#[derive(Debug, Clone)]
pub struct FrameProfile {
    pub frame_number: u64,
    pub total_time_ms: f32,
    pub scopes: Vec<ScopeProfile>,
}

/// 作用域统计信息
#[derive(Debug, Clone)]
pub struct ScopeStats {
    pub total_time_ms: f64,
    pub call_count: u64,
    pub min_time_ms: f32,
    pub max_time_ms: f32,
    pub avg_time_ms: f32,
}

impl ScopeStats {
    fn new() -> Self {
        Self {
            total_time_ms: 0.0,
            call_count: 0,
            min_time_ms: f32::MAX,
            max_time_ms: 0.0,
            avg_time_ms: 0.0,
        }
    }

    fn update(&mut self, duration_ms: f32) {
        self.total_time_ms += duration_ms as f64;
        self.call_count += 1;
        self.min_time_ms = self.min_time_ms.min(duration_ms);
        self.max_time_ms = self.max_time_ms.max(duration_ms);
        self.avg_time_ms = (self.total_time_ms / self.call_count as f64) as f32;
    }
}

/// 性能分析器
pub struct Profiler {
    enabled: bool,
    current_frame: u64,
    scope_stack: Vec<ProfileScope>,
    frame_start: Option<Instant>,
    frame_history: VecDeque<FrameProfile>,
    max_frames: usize,
    scope_stats: HashMap<String, ScopeStats>,
}

impl Profiler {
    pub fn new() -> Self {
        Self {
            enabled: true,
            current_frame: 0,
            scope_stack: Vec::new(),
            frame_start: None,
            frame_history: VecDeque::new(),
            max_frames: 60,
            scope_stats: HashMap::new(),
        }
    }

    pub fn with_capacity(max_frames: usize) -> Self {
        Self {
            enabled: true,
            current_frame: 0,
            scope_stack: Vec::new(),
            frame_start: None,
            frame_history: VecDeque::with_capacity(max_frames),
            max_frames,
            scope_stats: HashMap::new(),
        }
    }

    pub fn begin_frame(&mut self) {
        if !self.enabled {
            return;
        }

        self.frame_start = Some(Instant::now());
        self.current_frame += 1;
    }

    pub fn end_frame(&mut self) -> FrameProfile {
        let total_time_ms = if let Some(start) = self.frame_start {
            start.elapsed().as_secs_f32() * 1000.0
        } else {
            0.0
        };

        let profile = FrameProfile {
            frame_number: self.current_frame,
            total_time_ms,
            scopes: Vec::new(),
        };

        if self.enabled {
            self.frame_history.push_back(profile.clone());
            if self.frame_history.len() > self.max_frames {
                self.frame_history.pop_front();
            }
        }

        self.frame_start = None;
        profile
    }

    pub fn begin_scope(&mut self, name: &str) {
        if !self.enabled {
            return;
        }

        let scope = ProfileScope {
            name: name.to_string(),
            start_time: Instant::now(),
        };

        self.scope_stack.push(scope);
    }

    pub fn end_scope(&mut self) {
        if !self.enabled || self.scope_stack.is_empty() {
            return;
        }

        if let Some(scope) = self.scope_stack.pop() {
            let duration_ms = scope.start_time.elapsed().as_secs_f32() * 1000.0;

            let stats = self
                .scope_stats
                .entry(scope.name.clone())
                .or_insert_with(ScopeStats::new);
            stats.update(duration_ms);
        }
    }

    pub fn get_scope_stats(&self, name: &str) -> Option<&ScopeStats> {
        self.scope_stats.get(name)
    }

    pub fn get_all_scope_stats(&self) -> &HashMap<String, ScopeStats> {
        &self.scope_stats
    }

    pub fn get_hottest_scopes(&self, count: usize) -> Vec<(&str, &ScopeStats)> {
        let mut scopes: Vec<_> = self
            .scope_stats
            .iter()
            .map(|(name, stats)| (name.as_str(), stats))
            .collect();

        scopes.sort_by(|a, b| {
            b.1.total_time_ms
                .partial_cmp(&a.1.total_time_ms)
                .unwrap()
        });

        scopes.into_iter().take(count).collect()
    }

    pub fn get_frame_history(&self) -> &VecDeque<FrameProfile> {
        &self.frame_history
    }

    pub fn get_average_frame_time(&self, samples: usize) -> f32 {
        if self.frame_history.is_empty() {
            return 0.0;
        }

        let count = samples.min(self.frame_history.len());
        let sum: f32 = self
            .frame_history
            .iter()
            .rev()
            .take(count)
            .map(|f| f.total_time_ms)
            .sum();
        sum / count as f32
    }

    pub fn clear(&mut self) {
        self.scope_stack.clear();
        self.frame_history.clear();
        self.scope_stats.clear();
        self.current_frame = 0;
        self.frame_start = None;
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn get_frame_number(&self) -> u64 {
        self.current_frame
    }
}

impl Default for Profiler {
    fn default() -> Self {
        Self::new()
    }
}

/// RAII 作用域守卫
pub struct ScopeGuard<'a> {
    profiler: &'a mut Profiler,
}

impl<'a> ScopeGuard<'a> {
    pub fn new(profiler: &'a mut Profiler, name: &str) -> Self {
        profiler.begin_scope(name);
        Self { profiler }
    }
}

impl<'a> Drop for ScopeGuard<'a> {
    fn drop(&mut self) {
        self.profiler.end_scope();
    }
}

/// 性能分析宏
#[macro_export]
macro_rules! profile_scope {
    ($profiler:expr, $name:expr) => {
        let _guard = $crate::debug::profiler::ScopeGuard::new($profiler, $name);
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_profiler_creation() {
        let profiler = Profiler::new();
        assert!(profiler.is_enabled());
        assert_eq!(profiler.get_frame_number(), 0);
    }

    #[test]
    fn test_frame_timing() {
        let mut profiler = Profiler::new();

        profiler.begin_frame();
        thread::sleep(Duration::from_millis(10));
        let profile = profiler.end_frame();

        assert!(profile.total_time_ms >= 10.0);
        assert_eq!(profile.frame_number, 1);
    }

    #[test]
    fn test_scope_timing() {
        let mut profiler = Profiler::new();

        profiler.begin_scope("test_scope");
        thread::sleep(Duration::from_millis(5));
        profiler.end_scope();

        let stats = profiler.get_scope_stats("test_scope");
        assert!(stats.is_some());
        assert!(stats.unwrap().avg_time_ms >= 5.0);
    }

    #[test]
    fn test_nested_scopes() {
        let mut profiler = Profiler::new();

        profiler.begin_scope("outer");
        profiler.begin_scope("inner");
        thread::sleep(Duration::from_millis(5));
        profiler.end_scope();
        profiler.end_scope();

        assert!(profiler.get_scope_stats("outer").is_some());
        assert!(profiler.get_scope_stats("inner").is_some());
    }

    #[test]
    fn test_scope_stats() {
        let mut profiler = Profiler::new();

        for _ in 0..10 {
            profiler.begin_scope("repeated");
            thread::sleep(Duration::from_millis(1));
            profiler.end_scope();
        }

        let stats = profiler.get_scope_stats("repeated").unwrap();
        assert_eq!(stats.call_count, 10);
        assert!(stats.avg_time_ms >= 1.0);
        assert!(stats.min_time_ms > 0.0);
        assert!(stats.max_time_ms >= stats.min_time_ms);
    }

    #[test]
    fn test_hottest_scopes() {
        let mut profiler = Profiler::new();

        profiler.begin_scope("fast");
        thread::sleep(Duration::from_millis(1));
        profiler.end_scope();

        profiler.begin_scope("slow");
        thread::sleep(Duration::from_millis(10));
        profiler.end_scope();

        let hottest = profiler.get_hottest_scopes(2);
        assert_eq!(hottest.len(), 2);
        assert_eq!(hottest[0].0, "slow");
    }

    #[test]
    fn test_frame_history() {
        let mut profiler = Profiler::with_capacity(5);

        for _ in 0..10 {
            profiler.begin_frame();
            profiler.end_frame();
        }

        assert_eq!(profiler.get_frame_history().len(), 5);
    }

    #[test]
    fn test_average_frame_time() {
        let mut profiler = Profiler::new();

        for _ in 0..10 {
            profiler.begin_frame();
            thread::sleep(Duration::from_millis(5));
            profiler.end_frame();
        }

        let avg = profiler.get_average_frame_time(10);
        assert!(avg >= 5.0);
    }

    #[test]
    fn test_clear() {
        let mut profiler = Profiler::new();

        profiler.begin_frame();
        profiler.end_frame();
        profiler.begin_scope("test");
        profiler.end_scope();

        profiler.clear();

        assert_eq!(profiler.get_frame_number(), 0);
        assert_eq!(profiler.get_frame_history().len(), 0);
        assert!(profiler.get_scope_stats("test").is_none());
    }

    #[test]
    fn test_enable_disable() {
        let mut profiler = Profiler::new();

        profiler.set_enabled(false);
        profiler.begin_frame();
        profiler.end_frame();

        assert_eq!(profiler.get_frame_history().len(), 0);

        profiler.set_enabled(true);
        profiler.begin_frame();
        profiler.end_frame();

        assert_eq!(profiler.get_frame_history().len(), 1);
    }

    #[test]
    fn test_scope_guard() {
        let mut profiler = Profiler::new();

        {
            let _guard = ScopeGuard::new(&mut profiler, "guarded");
            thread::sleep(Duration::from_millis(5));
        }

        let stats = profiler.get_scope_stats("guarded");
        assert!(stats.is_some());
        assert!(stats.unwrap().avg_time_ms >= 5.0);
    }

    #[test]
    fn test_empty_profiler_stats() {
        let profiler = Profiler::new();
        assert_eq!(profiler.get_average_frame_time(10), 0.0);
        assert_eq!(profiler.get_hottest_scopes(5).len(), 0);
    }

    #[test]
    fn test_scope_without_begin_frame() {
        let mut profiler = Profiler::new();

        profiler.begin_scope("test");
        profiler.end_scope();

        assert!(profiler.get_scope_stats("test").is_some());
    }

    #[test]
    fn test_multiple_frames() {
        let mut profiler = Profiler::new();

        for i in 0..5 {
            profiler.begin_frame();
            let profile = profiler.end_frame();
            assert_eq!(profile.frame_number, i + 1);
        }

        assert_eq!(profiler.get_frame_number(), 5);
    }

    #[test]
    fn test_all_scope_stats() {
        let mut profiler = Profiler::new();

        profiler.begin_scope("scope1");
        profiler.end_scope();
        profiler.begin_scope("scope2");
        profiler.end_scope();

        let all_stats = profiler.get_all_scope_stats();
        assert_eq!(all_stats.len(), 2);
    }
}

/// DebugProfiler - 性能分析器
///
/// 功能：
/// - 函数/代码块性能分析
/// - 采样分析
/// - 性能报告生成
/// - 热点函数识别
/// - 调用树追踪

use std::time::{Instant, Duration};
use std::collections::{HashMap, VecDeque};

pub struct ProfilerSample {
    pub name: String,
    pub start_time: Instant,
    pub duration: Duration,
    pub call_count: u64,
}

pub struct ProfilerScope<'a> {
    profiler: &'a mut DebugProfiler,
    name: String,
    start_time: Instant,
}

impl<'a> Drop for ProfilerScope<'a> {
    fn drop(&mut self) {
        let elapsed = self.start_time.elapsed();
        self.profiler.record(&self.name, elapsed);
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ProfilerCategory {
    Update,
    Render,
    Physics,
    Audio,
    Network,
    Script,
    Input,
    Other,
}

impl ProfilerCategory {
    pub fn to_string(&self) -> &'static str {
        match self {
            ProfilerCategory::Update => "Update",
            ProfilerCategory::Render => "Render",
            ProfilerCategory::Physics => "Physics",
            ProfilerCategory::Audio => "Audio",
            ProfilerCategory::Network => "Network",
            ProfilerCategory::Script => "Script",
            ProfilerCategory::Input => "Input",
            ProfilerCategory::Other => "Other",
        }
    }
}

#[derive(Clone)]
pub struct ProfilerEntry {
    pub name: String,
    pub total_time: Duration,
    pub self_time: Duration,
    pub call_count: u64,
    pub min_time: Duration,
    pub max_time: Duration,
    pub category: ProfilerCategory,
    pub children: Vec<String>,
}

impl ProfilerEntry {
    pub fn avg_time(&self) -> Duration {
        if self.call_count == 0 {
            Duration::ZERO
        } else {
            Duration::from_nanos(self.total_time.as_nanos() as u64 / self.call_count)
        }
    }

    pub fn avg_time_ms(&self) -> f64 {
        self.avg_time().as_secs_f64() * 1000.0
    }

    pub fn total_time_ms(&self) -> f64 {
        self.total_time.as_secs_f64() * 1000.0
    }

    pub fn percentage(&self, total: Duration) -> f64 {
        if total.as_nanos() == 0 {
            0.0
        } else {
            (self.total_time.as_nanos() as f64 / total.as_nanos() as f64) * 100.0
        }
    }
}

#[derive(Debug)]
struct ProfilerData {
    name: String,
    total_time: Duration,
    self_time: Duration,
    call_count: u64,
    min_time: Duration,
    max_time: Duration,
    category: ProfilerCategory,
    parent: Option<String>,
    children: Vec<String>,
}

pub struct DebugProfiler {
    entries: HashMap<String, ProfilerData>,
    call_stack: Vec<String>,
    start_times: HashMap<String, Instant>,
    frame_start: Instant,
    frame_time: Duration,
    enabled: bool,
    sample_rate: Duration,
    last_sample: Instant,
    samples: VecDeque<ProfilerSample>,
    max_samples: usize,
    frame_times: VecDeque<Duration>,
    frame_count: u64,
    auto_frame: bool,
}

impl DebugProfiler {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            call_stack: Vec::new(),
            start_times: HashMap::new(),
            frame_start: Instant::now(),
            frame_time: Duration::ZERO,
            enabled: true,
            sample_rate: Duration::from_millis(1),
            last_sample: Instant::now(),
            samples: VecDeque::new(),
            max_samples: 1000,
            frame_times: VecDeque::with_capacity(60),
            frame_count: 0,
            auto_frame: true,
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_sample_rate(&mut self, rate: Duration) {
        self.sample_rate = rate;
    }

    pub fn set_auto_frame(&mut self, auto: bool) {
        self.auto_frame = auto;
    }

    pub fn begin_frame(&mut self) {
        if !self.enabled {
            return;
        }
        self.frame_start = Instant::now();
        self.call_stack.clear();
    }

    pub fn end_frame(&mut self) {
        if !self.enabled {
            return;
        }
        self.frame_time = self.frame_start.elapsed();
        self.frame_count += 1;
        self.frame_times.push_back(self.frame_time);
        if self.frame_times.len() > 60 {
            self.frame_times.pop_front();
        }
    }

    pub fn begin(&mut self, name: &str) {
        if !self.enabled {
            return;
        }
        self.call_stack.push(name.to_string());
        self.start_times.insert(name.to_string(), Instant::now());
    }

    pub fn end(&mut self, name: &str) {
        if !self.enabled {
            return;
        }

        if let Some(popped) = self.call_stack.pop() {
            if popped != name {
                return;
            }
        }

        if let Some(start_time) = self.start_times.remove(name) {
            let duration = start_time.elapsed();
            let last_parent = self.call_stack.last().cloned();
            if let Some(parent) = last_parent {
                self.record_with_parent(name, &parent, duration);
            } else {
                self.record(name, duration);
            }
        }
    }

    pub fn scope(&mut self, name: &str) -> ProfilerScope {
        self.begin(name);
        ProfilerScope {
            profiler: self,
            name: name.to_string(),
            start_time: Instant::now(),
        }
    }

    pub fn record(&mut self, name: &str, duration: Duration) {
        if !self.enabled {
            return;
        }

        let entry = self.entries.entry(name.to_string()).or_insert(ProfilerData {
            name: name.to_string(),
            total_time: Duration::ZERO,
            self_time: Duration::ZERO,
            call_count: 0,
            min_time: Duration::MAX,
            max_time: Duration::ZERO,
            category: ProfilerCategory::Other,
            parent: None,
            children: Vec::new(),
        });

        entry.total_time += duration;
        entry.call_count += 1;
        entry.min_time = entry.min_time.min(duration);
        entry.max_time = entry.max_time.max(duration);

        self.record_sample(name, duration);
    }

    fn record_with_parent(&mut self, name: &str, parent: &str, duration: Duration) {
        if !self.enabled {
            return;
        }

        let entry = self.entries.entry(name.to_string()).or_insert(ProfilerData {
            name: name.to_string(),
            total_time: Duration::ZERO,
            self_time: Duration::ZERO,
            call_count: 0,
            min_time: Duration::MAX,
            max_time: Duration::ZERO,
            category: ProfilerCategory::Other,
            parent: None,
            children: Vec::new(),
        });

        entry.parent = Some(parent.to_string());

        if !entry.children.iter().any(|c| c == parent) {
            entry.children.push(parent.to_string());
        }

        self.record(name, duration);
    }

    fn record_sample(&mut self, name: &str, duration: Duration) {
        let now = Instant::now();
        if now.duration_since(self.last_sample) >= self.sample_rate {
            self.last_sample = now;
            self.samples.push_back(ProfilerSample {
                name: name.to_string(),
                start_time: now,
                duration,
                call_count: 1,
            });
            if self.samples.len() > self.max_samples {
                self.samples.pop_front();
            }
        }
    }

    pub fn set_category(&mut self, name: &str, category: ProfilerCategory) {
        if let Some(entry) = self.entries.get_mut(name) {
            entry.category = category;
        }
    }

    pub fn get_entry(&self, name: &str) -> Option<ProfilerEntry> {
        self.entries.get(name).map(|d| ProfilerEntry {
            name: d.name.clone(),
            total_time: d.total_time,
            self_time: d.self_time,
            call_count: d.call_count,
            min_time: d.min_time,
            max_time: d.max_time,
            category: d.category,
            children: d.children.clone(),
        })
    }

    pub fn get_all_entries(&self) -> Vec<ProfilerEntry> {
        self.entries.values().map(|d| ProfilerEntry {
            name: d.name.clone(),
            total_time: d.total_time,
            self_time: d.self_time,
            call_count: d.call_count,
            min_time: d.min_time,
            max_time: d.max_time,
            category: d.category,
            children: d.children.clone(),
        }).collect()
    }

    pub fn get_hotspots(&self, count: usize) -> Vec<(String, f64)> {
        let mut entries: Vec<_> = self.entries.iter()
            .map(|(name, data)| (name.clone(), data.total_time))
            .collect();
        entries.sort_by(|a, b| b.1.cmp(&a.1));
        entries.into_iter().take(count)
            .map(|(name, time)| (name, time.as_secs_f64() * 1000.0))
            .collect()
    }

    pub fn get_frame_time(&self) -> Duration {
        self.frame_time
    }

    pub fn get_frame_time_ms(&self) -> f64 {
        self.frame_time.as_secs_f64() * 1000.0
    }

    pub fn get_frame_count(&self) -> u64 {
        self.frame_count
    }

    pub fn get_avg_frame_time(&self) -> Duration {
        if self.frame_times.is_empty() {
            Duration::ZERO
        } else {
            let total: Duration = self.frame_times.iter().sum();
            let avg_nanos = total.as_nanos() / self.frame_times.len() as u128;
            Duration::from_nanos(avg_nanos as u64)
        }
    }

    pub fn get_avg_fps(&self) -> f64 {
        let avg_frame_time = self.get_avg_frame_time();
        if avg_frame_time.as_nanos() == 0 {
            0.0
        } else {
            1.0 / avg_frame_time.as_secs_f64()
        }
    }

    pub fn get_min_frame_time(&self) -> Option<Duration> {
        self.frame_times.iter().min().copied()
    }

    pub fn get_max_frame_time(&self) -> Option<Duration> {
        self.frame_times.iter().max().copied()
    }

    pub fn get_samples(&self) -> &VecDeque<ProfilerSample> {
        &self.samples
    }

    pub fn clear(&mut self) {
        self.entries.clear();
        self.call_stack.clear();
        self.start_times.clear();
        self.samples.clear();
        self.frame_times.clear();
        self.frame_count = 0;
    }

    pub fn reset_frame(&mut self) {
        self.entries.clear();
        self.call_stack.clear();
        self.start_times.clear();
        self.frame_times.push_back(self.frame_time);
        if self.frame_times.len() > 60 {
            self.frame_times.pop_front();
        }
    }

    pub fn generate_report(&self) -> String {
        let mut report = String::new();

        report.push_str("=== Profiler Report ===\n");
        report.push_str(&format!("Frame Time: {:.2} ms\n", self.get_frame_time_ms()));
        report.push_str(&format!("Avg Frame Time: {:.2} ms\n", self.get_avg_frame_time().as_secs_f64() * 1000.0));
        report.push_str(&format!("Avg FPS: {:.1}\n", self.get_avg_fps()));
        report.push_str(&format!("Frame Count: {}\n", self.frame_count));
        report.push_str(&format!("Total Entries: {}\n", self.entries.len()));

        report.push_str("\n=== Hotspots (by total time) ===\n");
        let mut entries: Vec<_> = self.entries.iter()
            .map(|(name, data)| {
                let time_ms = data.total_time.as_secs_f64() * 1000.0;
                let avg_ms = if data.call_count > 0 { time_ms / data.call_count as f64 } else { 0.0 };
                (name.clone(), time_ms, avg_ms, data.call_count)
            })
            .collect();
        entries.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        for (name, total_ms, avg_ms, count) in entries.iter().take(20) {
            report.push_str(&format!(
                "{:30} Total: {:8.2}ms  Avg: {:8.4}ms  Calls: {:6}  Category: {}\n",
                name,
                total_ms,
                avg_ms,
                count,
                ProfilerCategory::Other.to_string()
            ));
        }

        report.push_str("\n=== Samples ===\n");
        for sample in self.samples.iter().take(10) {
            report.push_str(&format!(
                "{:30} {:8.2}ms\n",
                sample.name,
                sample.duration.as_secs_f64() * 1000.0
            ));
        }

        report
    }
}

impl Default for DebugProfiler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_profiler_creation() {
        let profiler = DebugProfiler::new();
        assert!(profiler.is_enabled());
        assert_eq!(profiler.get_frame_count(), 0);
    }

    #[test]
    fn test_profiler_enable_disable() {
        let mut profiler = DebugProfiler::new();
        profiler.set_enabled(false);
        assert!(!profiler.is_enabled());

        profiler.set_enabled(true);
        assert!(profiler.is_enabled());
    }

    #[test]
    fn test_profiler_frame() {
        let mut profiler = DebugProfiler::new();
        profiler.begin_frame();
        thread::sleep(Duration::from_millis(10));
        profiler.end_frame();

        assert_eq!(profiler.get_frame_count(), 1);
        assert!(profiler.get_frame_time_ms() > 0.0);
    }

    #[test]
    fn test_profiler_scope() {
        let mut profiler = DebugProfiler::new();
        profiler.begin_frame();

        {
            let _s = profiler.scope("test_scope");
            thread::sleep(Duration::from_millis(5));
        }

        profiler.end_frame();

        let entry = profiler.get_entry("test_scope");
        assert!(entry.is_some());
        assert_eq!(entry.unwrap().call_count, 1);
    }

    #[test]
    fn test_profiler_record() {
        let mut profiler = DebugProfiler::new();
        profiler.record("test", Duration::from_millis(10));
        profiler.record("test", Duration::from_millis(20));

        let entry = profiler.get_entry("test").unwrap();
        assert_eq!(entry.call_count, 2);
        assert!(entry.total_time.as_millis() >= 30);
    }

    #[test]
    fn test_profiler_hotspots() {
        let mut profiler = DebugProfiler::new();

        for _ in 0..100 {
            profiler.record("slow", Duration::from_millis(10));
            profiler.record("fast", Duration::from_millis(1));
        }

        let hotspots = profiler.get_hotspots(2);
        assert_eq!(hotspots.len(), 2);
        assert_eq!(hotspots[0].0, "slow");
        assert_eq!(hotspots[1].0, "fast");
    }

    #[test]
    fn test_profiler_avg_fps() {
        let mut profiler = DebugProfiler::new();

        for _ in 0..10 {
            profiler.begin_frame();
            thread::sleep(Duration::from_millis(16));
            profiler.end_frame();
        }

        assert_eq!(profiler.get_frame_count(), 10);
        let fps = profiler.get_avg_fps();
        assert!(fps > 0.0 && fps < 100.0);
    }

    #[test]
    fn test_profiler_clear() {
        let mut profiler = DebugProfiler::new();

        profiler.record("test", Duration::from_millis(10));
        profiler.begin_frame();
        profiler.end_frame();

        profiler.clear();

        assert!(profiler.get_entry("test").is_none());
        assert_eq!(profiler.get_frame_count(), 0);
    }

    #[test]
    fn test_profiler_category() {
        let mut profiler = DebugProfiler::new();
        profiler.record("update", Duration::from_millis(5));
        profiler.set_category("update", ProfilerCategory::Update);

        let entry = profiler.get_entry("update").unwrap();
        matches!(entry.category, ProfilerCategory::Update);
    }

    #[test]
    fn test_profiler_report() {
        let mut profiler = DebugProfiler::new();
        profiler.record("func1", Duration::from_millis(5));
        profiler.record("func2", Duration::from_millis(3));

        let report = profiler.generate_report();
        assert!(report.contains("Profiler Report"));
        assert!(report.contains("func1"));
        assert!(report.contains("func2"));
    }

    #[test]
    fn test_profiler_entry_average() {
        let mut profiler = DebugProfiler::new();

        profiler.record("test", Duration::from_millis(10));
        profiler.record("test", Duration::from_millis(20));
        profiler.record("test", Duration::from_millis(30));

        let entry = profiler.get_entry("test").unwrap();
        let avg = entry.avg_time_ms();
        assert!((avg - 20.0).abs() < 0.01);
    }

    #[test]
    fn test_profiler_all_entries() {
        let mut profiler = DebugProfiler::new();
        profiler.record("a", Duration::from_millis(5));
        profiler.record("b", Duration::from_millis(10));

        let entries = profiler.get_all_entries();
        assert_eq!(entries.len(), 2);
    }
}

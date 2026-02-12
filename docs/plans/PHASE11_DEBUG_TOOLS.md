# Phase 11: 调试与工具系统

## 📋 概览

**目标：** 实现完整的调试和开发者工具支持，提升开发体验

**时间估算：** 3-4 天

**测试目标：** 50+ 单元测试

**优先级：** 高 🔴

---

## 🎯 核心目标

### 1. Console 控制台系统
- 实时日志输出和过滤
- 命令行接口（REPL）
- 日志级别管理
- 历史记录保存

### 2. Stats 统计系统
- FPS 实时监控
- DrawCall 统计
- 顶点/三角形计数
- 纹理内存使用
- 节点数量统计

### 3. Profiler 性能分析器
- 函数调用计时
- 帧分析工具
- 性能热点识别
- 内存分配跟踪

---

## 📦 模块设计

### Console 模块

```rust
pub mod console {
    use std::collections::VecDeque;
    use std::sync::{Arc, Mutex};

    /// 日志级别
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum LogLevel {
        Trace,
        Debug,
        Info,
        Warning,
        Error,
        Fatal,
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

    /// 命令处理器
    pub type CommandHandler = Box<dyn Fn(&[&str]) -> Result<String, String> + Send + Sync>;

    /// 控制台系统
    pub struct Console {
        logs: VecDeque<LogEntry>,
        max_logs: usize,
        commands: std::collections::HashMap<String, CommandHandler>,
        history: VecDeque<String>,
        max_history: usize,
        visible: bool,
        filter_level: LogLevel,
    }

    impl Console {
        pub fn new() -> Self;
        pub fn log(&mut self, level: LogLevel, message: impl Into<String>);
        pub fn register_command(&mut self, name: &str, handler: CommandHandler);
        pub fn execute_command(&mut self, command: &str) -> Result<String, String>;
        pub fn get_logs(&self, level: Option<LogLevel>) -> Vec<&LogEntry>;
        pub fn clear(&mut self);
        pub fn set_visible(&mut self, visible: bool);
        pub fn toggle_visible(&mut self);
    }
}
```

### Stats 模块

```rust
pub mod stats {
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
        frame_count: u64,
        time_accumulator: f32,
        enabled: bool,
    }

    impl Stats {
        pub fn new() -> Self;
        pub fn update(&mut self, dt: f32);
        pub fn record_draw_call(&mut self, vertices: u32, triangles: u32);
        pub fn record_node_count(&mut self, total: u32, visible: u32);
        pub fn record_texture_memory(&mut self, bytes: usize);
        pub fn get_current(&self) -> &StatsSnapshot;
        pub fn get_average_fps(&self, samples: usize) -> f32;
        pub fn reset(&mut self);
        pub fn set_enabled(&mut self, enabled: bool);
    }
}
```

### Profiler 模块

```rust
pub mod profiler {
    use std::time::Instant;
    use std::collections::HashMap;

    /// 性能作用域
    pub struct ProfileScope {
        name: String,
        start_time: Instant,
        children: Vec<ProfileScope>,
    }

    /// 帧性能分析
    #[derive(Debug, Clone)]
    pub struct FrameProfile {
        pub frame_number: u64,
        pub total_time_ms: f32,
        pub scopes: Vec<ScopeProfile>,
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

    /// 性能分析器
    pub struct Profiler {
        enabled: bool,
        current_frame: u64,
        scope_stack: Vec<ProfileScope>,
        frame_history: VecDeque<FrameProfile>,
        max_frames: usize,
        scope_stats: HashMap<String, ScopeStats>,
    }

    #[derive(Debug, Clone)]
    pub struct ScopeStats {
        pub total_time_ms: f64,
        pub call_count: u64,
        pub min_time_ms: f32,
        pub max_time_ms: f32,
        pub avg_time_ms: f32,
    }

    impl Profiler {
        pub fn new() -> Self;
        pub fn begin_frame(&mut self);
        pub fn end_frame(&mut self) -> FrameProfile;
        pub fn begin_scope(&mut self, name: &str);
        pub fn end_scope(&mut self);
        pub fn get_scope_stats(&self, name: &str) -> Option<&ScopeStats>;
        pub fn get_hottest_scopes(&self, count: usize) -> Vec<(&str, &ScopeStats)>;
        pub fn clear(&mut self);
        pub fn set_enabled(&mut self, enabled: bool);
    }

    /// RAII 作用域守卫
    pub struct ScopeGuard<'a> {
        profiler: &'a mut Profiler,
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
            $profiler.begin_scope($name);
            let _guard = ScopeGuard { profiler: $profiler };
        };
    }
}
```

---

## 🧪 测试计划

### Console 测试（15个）
- [x] 创建和初始化
- [x] 添加不同级别的日志
- [x] 日志过滤
- [x] 日志容量限制
- [x] 命令注册和执行
- [x] 命令参数解析
- [x] 命令错误处理
- [x] 历史记录
- [x] 清空日志
- [x] 可见性切换
- [x] 时间戳验证
- [x] 文件和行号记录
- [x] 并发安全性
- [x] 内存泄漏测试
- [x] 性能测试

### Stats 测试（20个）
- [x] 创建和初始化
- [x] FPS 计算
- [x] 帧时间统计
- [x] DrawCall 记录
- [x] 顶点统计
- [x] 三角形统计
- [x] 纹理内存统计
- [x] 节点计数
- [x] 历史记录
- [x] 平均值计算
- [x] 峰值统计
- [x] 重置功能
- [x] 启用/禁用
- [x] 多帧统计
- [x] 极限值测试
- [x] 精度测试
- [x] 零值处理
- [x] 大数值处理
- [x] 性能开销测试
- [x] 内存占用测试

### Profiler 测试（15个）
- [x] 创建和初始化
- [x] 帧开始/结束
- [x] 作用域计时
- [x] 嵌套作用域
- [x] 作用域统计
- [x] 热点识别
- [x] 帧历史
- [x] 统计数据准确性
- [x] 启用/禁用
- [x] 清空数据
- [x] 并发测试
- [x] RAII守卫测试
- [x] 宏测试
- [x] 性能开销
- [x] 内存占用

---

## 📝 实现步骤

### Day 1: Console 系统
1. ✅ 创建 `src/debug/mod.rs`
2. ✅ 实现 Console 核心结构
3. ✅ 实现日志系统
4. ✅ 实现命令系统
5. ✅ 编写 15 个单元测试

### Day 2: Stats 系统
1. ✅ 实现 Stats 核心结构
2. ✅ 实现 FPS 监控
3. ✅ 实现渲染统计
4. ✅ 实现内存统计
5. ✅ 编写 20 个单元测试

### Day 3: Profiler 系统
1. ✅ 实现 Profiler 核心结构
2. ✅ 实现作用域计时
3. ✅ 实现统计分析
4. ✅ 实现 RAII 守卫
5. ✅ 编写 15 个单元测试

### Day 4: 集成与优化
1. ✅ 集成到 Director
2. ✅ 添加便捷宏
3. ✅ 性能优化
4. ✅ 文档完善
5. ✅ 最终测试

---

## 🎨 使用示例

```rust
use cocos2d_rust::debug::{Console, Stats, Profiler};

// 1. 控制台使用
let mut console = Console::new();
console.log(LogLevel::Info, "Game started");
console.register_command("fps", |args| {
    Ok(format!("Current FPS: 60"))
});
console.execute_command("fps")?;

// 2. 统计系统
let mut stats = Stats::new();
stats.update(0.016); // 每帧更新
stats.record_draw_call(100, 200);
println!("FPS: {}", stats.get_current().fps);

// 3. 性能分析
let mut profiler = Profiler::new();
profiler.begin_frame();
{
    profiler.begin_scope("render");
    // 渲染代码
    profiler.end_scope();
}
let frame = profiler.end_frame();
println!("Frame time: {}ms", frame.total_time_ms);

// 4. 使用宏
profile_scope!(profiler, "physics_update");
// 代码会自动计时
```

---

## 📊 成功指标

- ✅ 所有 50+ 测试通过
- ✅ 零性能开销（禁用时）
- ✅ < 1ms 性能开销（启用时）
- ✅ 完整的 API 文档
- ✅ 使用示例代码

---

## 🔗 相关模块

- **base::Director** - 集成调试工具
- **renderer::Renderer** - 提供渲染统计
- **scene::Node** - 提供节点统计

---

## 📅 时间线

- **Day 1:** Console 系统 ✅
- **Day 2:** Stats 系统 ✅
- **Day 3:** Profiler 系统 ✅
- **Day 4:** 集成与测试 ✅

**预计完成时间：** 2026年2月12日

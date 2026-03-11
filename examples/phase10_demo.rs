/// Phase 10 Demo - 调试系统演示
///
/// 展示：
/// - DebugStats 性能统计
/// - DebugConsole 控制台
/// - DebugProfiler 性能分析器
/// - DebugLayer 集成调试 UI

use cocos2d_rust::{
    DebugStats, DebugConsole, DebugProfiler, DebugLayer,
    LogLevel, LogEntry, ConsoleCommand, ConsoleHandler,
    ProfilerScope, ProfilerCategory, ProfilerEntry,
    DebugPanel, DebugPanelConfig,
};
use cocos2d_rust::base::debug_console::CommandResult;
use std::time::{Duration, Instant};
use std::thread;
use std::collections::HashMap;

struct CustomConsoleHandler;

impl ConsoleHandler for CustomConsoleHandler {
    fn handle_command(&mut self, command: &str) -> CommandResult {
        let parts: Vec<&str> = command.trim().split_whitespace().collect();
        if parts.is_empty() {
            return CommandResult::new(String::new(), true);
        }

        match parts[0].to_lowercase().as_str() {
            "status" => CommandResult::new("System is running normally".to_string(), true),
            "echo" if parts.len() > 1 => {
                let msg = parts[1..].join(" ");
                CommandResult::new(format!("Echo: {}", msg), true)
            },
            "version" => CommandResult::new("Cocos2d-Rust v0.1.0".to_string(), true),
            "count" => CommandResult::new("Console commands executed: count feature disabled".to_string(), true),
            _ => CommandResult::new(format!("Unknown command: {}", parts[0]), false),
        }
    }

    fn get_help(&self) -> String {
        vec![
            "=== Custom Console Commands ===",
            "status  - Show system status",
            "echo <msg> - Echo message back",
            "version - Show version info",
            "count   - Show command count",
        ].join("\n")
    }
}

fn demo_debug_stats() {
    println!("\n=== DebugStats Demo ===\n");

    let mut stats = DebugStats::new();

    println!("1. 创建性能统计器");
    println!("   - 默认启用: {}", stats.is_enabled());
    println!("   - 默认可见: {}", stats.is_visible());

    println!("\n2. 设置统计参数");
    stats.set_enabled(true);
    stats.set_visible(true);
    stats.set_fps_history_size(60);

    println!("\n3. 模拟帧更新");
    for i in 0..5 {
        stats.begin_frame();
        thread::sleep(Duration::from_millis(16));
        stats.add_draw_call((i + 1) * 10);
        stats.add_triangles((i + 1) * 100);
        stats.set_memory_usage(1024 * 1024 * (10 + i as u64));
        stats.end_frame();

        println!("   帧 {}: FPS {:.1}, DrawCalls: {}, 内存: {}",
            i + 1,
            stats.get_fps(),
            stats.get_draw_calls(),
            stats.get_memory_usage_string());
    }

    println!("\n4. 自定义统计");
    stats.set_int_stat("score", 1000);
    stats.set_float_stat("health", 99.5);
    stats.set_percentage_stat("battery", 75.0);

    if let Some(stat) = stats.get_stat("score") {
        println!("   分数: {:?}", stat);
    }

    println!("\n5. 统计摘要");
    println!("   FPS: {:.1}", stats.get_fps());
    println!("   帧时间: {}", stats.get_frame_time_string());
    println!("   帧计数: {}", stats.get_frame_count());
    println!("   绘制调用: {}", stats.get_draw_calls());
    println!("   三角形: {}", stats.get_triangles());
    println!("   顶点: {}", stats.get_vertices());
    println!("   内存: {}", stats.get_memory_usage_string());

    println!("\n6. 性能报告");
    println!("{}", stats.generate_report());

    println!("\n7. 重置统计");
    stats.reset();
    assert_eq!(stats.get_frame_count(), 0);
    assert_eq!(stats.get_draw_calls(), 0);
    println!("   统计已重置 ✓");

    println!("\nDebugStats 演示完成!\n");
}

fn demo_debug_console() {
    println!("\n=== DebugConsole Demo ===\n");

    let mut console = DebugConsole::new();

    println!("1. 创建控制台");
    println!("   - 默认可见: {}", console.is_visible());
    println!("   - 最大行数: {}", console.get_max_lines());

    println!("\n2. 日志级别测试");
    console.debug("调试消息");
    console.info("信息消息");
    console.warning("警告消息");
    console.error("错误消息");
    console.critical("严重错误消息");

    println!("\n3. 带标签的日志");
    console.log_with_tag(LogLevel::Info, "系统启动完成", Some("SYSTEM"));
    console.log_with_tag(LogLevel::Warning, "内存使用偏高", Some("MEMORY"));
    console.log_with_tag(LogLevel::Error, "网络连接失败", Some("NETWORK"));

    println!("\n4. 格式化日志");
    console.log_fmt(LogLevel::Info, None, format_args!("玩家 {} 得分: {}", "Player1", 1500));
    console.log_fmt(LogLevel::Debug, Some("GAME"), format_args!("关卡 {} 加载完成", 3));

    println!("\n5. 日志过滤");
    let error_logs: Vec<&LogEntry> = console.get_filtered_logs(LogLevel::Error);
    println!("   错误级别及以上日志数: {}", error_logs.len());

    let warning_logs: Vec<&LogEntry> = console.get_filtered_logs(LogLevel::Warning);
    println!("   警告级别及以上日志数: {}", warning_logs.len());

    println!("\n6. 最近日志");
    for entry in console.get_recent_logs(3) {
        println!("   {}", entry.formatted());
    }

    println!("\n7. 命令执行");
    let result = console.execute_command("help");
    println!("   help 命令输出: {}", result.output);

    let result = console.execute_command("stats");
    println!("   stats 命令成功: {}", result.success);

    let result = console.execute_command("unknown");
    println!("   unknown 命令成功: {}, 输出: {}", result.success, result.output);

    println!("\n8. 命令历史");
    assert!(console.get_input().is_empty());
    console.set_input("test input");
    assert_eq!(console.get_input(), "test input");

    println!("\n9. 控制台样式");
    let style = console.get_style();
    println!("   背景色: [{}, {}, {}, {}]",
        style.background_color[0],
        style.background_color[1],
        style.background_color[2],
        style.background_color[3]);
    println!("   字体大小: {}", style.font_size);
    println!("   自动滚动: {}", style.auto_scroll);

    println!("\n10. 自定义处理器");
    let mut custom_console = DebugConsole::with_handler(CustomConsoleHandler);
    let result = custom_console.execute_command("status");
    println!("    status 命令: {}", result.output);

    let result = custom_console.execute_command("echo Hello World");
    println!("    echo 命令: {}", result.output);

    println!("\n11. 控制台报告");
    println!("{}", console.generate_report());

    println!("\n12. 清理");
    console.clear();
    assert!(console.get_logs().is_empty());
    println!("   控制台已清理 ✓");

    println!("\nDebugConsole 演示完成!\n");
}

fn demo_debug_profiler() {
    println!("\n=== DebugProfiler Demo ===\n");

    let mut profiler = DebugProfiler::new();

    println!("1. 创建性能分析器");
    println!("   - 默认启用: {}", profiler.is_enabled());

    println!("\n2. 帧性能分析");
    for frame in 0..3 {
        profiler.begin_frame();

        profiler.begin("update");
        thread::sleep(Duration::from_millis(5));
        profiler.end("update");

        profiler.begin("render");
        thread::sleep(Duration::from_millis(3));
        profiler.end("render");

        profiler.begin("physics");
        thread::sleep(Duration::from_millis(2));
        profiler.end("physics");

        profiler.end_frame();

        println!("   帧 {}: {:.2}ms, FPS: {:.1}",
            frame + 1,
            profiler.get_frame_time_ms(),
            1000.0 / profiler.get_frame_time_ms());
    }

    println!("\n3. 作用域分析");
    profiler.begin_frame();
    {
        let _s = profiler.scope("scoped_operation");
        thread::sleep(Duration::from_millis(10));
    }
    profiler.end_frame();

    println!("   作用域操作已记录");
    if let Some(entry) = profiler.get_entry("scoped_operation") {
        println!("   - 名称: {}", entry.name);
        println!("   - 调用次数: {}", entry.call_count);
        println!("   - 总时间: {:.2}ms", entry.total_time_ms());
        println!("   - 平均时间: {:.2}ms", entry.avg_time_ms());
    }

    println!("\n4. 手动记录");
    profiler.record("custom_function", Duration::from_millis(15));
    profiler.record("custom_function", Duration::from_millis(20));

    if let Some(entry) = profiler.get_entry("custom_function") {
        println!("   - 名称: {}", entry.name);
        println!("   - 调用次数: {}", entry.call_count);
        println!("   - 总时间: {:.2}ms", entry.total_time_ms());
        println!("   - 平均时间: {:.2}ms", entry.avg_time_ms());
        println!("   - 最小时间: {:.2}ms", entry.min_time.as_secs_f64() * 1000.0);
        println!("   - 最大时间: {:.2}ms", entry.max_time.as_secs_f64() * 1000.0);
    }

    println!("\n5. 热点函数");
    profiler.record("hot_function_a", Duration::from_millis(50));
    profiler.record("hot_function_a", Duration::from_millis(50));
    profiler.record("hot_function_b", Duration::from_millis(30));
    profiler.record("hot_function_b", Duration::from_millis(30));
    profiler.record("hot_function_c", Duration::from_millis(10));

    println!("   热点排名 (前3):");
    for (name, time_ms) in profiler.get_hotspots(3) {
        println!("   - {}: {:.2}ms", name, time_ms);
    }

    println!("\n6. 分类设置");
    profiler.set_category("update", ProfilerCategory::Update);
    profiler.set_category("render", ProfilerCategory::Render);
    profiler.set_category("physics", ProfilerCategory::Physics);

    if let Some(entry) = profiler.get_entry("update") {
        println!("   update 分类: {:?}", entry.category);
    }

    println!("\n7. 性能报告");
    println!("{}", profiler.generate_report());

    println!("\n8. 帧统计");
    println!("   帧计数: {}", profiler.get_frame_count());
    println!("   平均帧时间: {:.2}ms", profiler.get_avg_frame_time().as_secs_f64() * 1000.0);
    println!("   平均 FPS: {:.1}", profiler.get_avg_fps());

    if let Some(min) = profiler.get_min_frame_time() {
        println!("   最小帧时间: {:.2}ms", min.as_secs_f64() * 1000.0);
    }
    if let Some(max) = profiler.get_max_frame_time() {
        println!("   最大帧时间: {:.2}ms", max.as_secs_f64() * 1000.0);
    }

    println!("\n9. 所有条目");
    for entry in profiler.get_all_entries() {
        println!("   - {}: {:.2}ms ({} 次调用)",
            entry.name,
            entry.total_time_ms(),
            entry.call_count);
    }

    println!("\n10. 清理");
    profiler.clear();
    assert!(profiler.get_all_entries().is_empty());
    assert_eq!(profiler.get_frame_count(), 0);
    println!("    分析器已清理 ✓");

    println!("\nDebugProfiler 演示完成!\n");
}

fn demo_debug_layer() {
    println!("\n=== DebugLayer Demo ===\n");

    let mut layer = DebugLayer::new();

    println!("1. 创建调试层");
    println!("   - 默认可见: {}", layer.is_visible());
    println!("   - 默认展开: {}", layer.is_expanded());

    println!("\n2. 帧循环模拟");
    for frame in 0..5 {
        layer.begin_frame();

        {
            let _s = layer.profile_scope("game_logic");
            thread::sleep(Duration::from_millis(5));
        }

        layer.log_debug(&format!("帧 {} 逻辑处理完成", frame));
        layer.set_draw_calls((frame + 1) * 10);
        layer.set_triangles((frame + 1) * 100);
        layer.set_memory_usage(1024 * 1024 * (20 + frame as u64));

        layer.end_frame();

        println!("   帧 {}: FPS {:.1}", frame + 1, layer.get_avg_fps());
    }

    println!("\n3. 面板管理");
    println!("   当前面板: {:?}", layer.get_panel());

    layer.set_panel(DebugPanel::Stats);
    println!("   设置为 Stats: {:?}", layer.get_panel());

    layer.toggle_console();
    println!("   切换到 Console: {:?}", layer.get_panel());

    layer.toggle_stats();
    println!("   切换到 Stats: {:?}", layer.get_panel());

    layer.toggle_profiler();
    println!("   切换到 Profiler: {:?}", layer.get_panel());

    layer.toggle_expand();
    println!("   切换展开状态: {}", layer.is_expanded());

    println!("\n4. 位置和大小");
    layer.set_position(100.0, 200.0);
    let (x, y) = layer.get_position();
    println!("   位置: ({}, {})", x, y);

    layer.set_size(400.0, 300.0);
    let size = layer.get_size();
    println!("   大小: {}x{}", size.width, size.height);

    println!("\n5. 图形设置");
    layer.set_show_fps_graph(false);
    layer.set_show_memory_graph(true);
    layer.set_show_call_graph(true);
    println!("   FPS 图形: 关闭");
    println!("   内存图形: 开启");
    println!("   调用图形: 开启");

    println!("\n6. 日志功能");
    layer.log("普通日志消息");
    layer.log_debug("调试日志消息");
    layer.log_warning("警告日志消息");
    layer.log_error("错误日志消息");

    let console = layer.get_console();
    println!("   控制台日志数: {}", console.get_logs().len());

    println!("\n7. 性能统计");
    let stats = layer.get_stats();
    stats.set_int_stat("enemies", 10);
    stats.set_int_stat("score", 5000);

    if let Some(stat) = stats.get_stat("enemies") {
        println!("   敌人数: {:?}", stat);
    }

    println!("\n8. FPS 历史");
    let fps_history = layer.get_fps_history();
    println!("   FPS 历史样本数: {}", fps_history.len());
    println!("   平均 FPS: {:.1}", layer.get_avg_fps());
    println!("   最低 FPS: {:.1}", layer.get_min_fps());
    println!("   最高 FPS: {:.1}", layer.get_max_fps());

    println!("\n9. 帧时间历史");
    let frame_time_history = layer.get_frame_time_history();
    println!("   帧时间样本数: {}", frame_time_history.len());
    println!("   平均帧时间: {:.2}ms", layer.get_avg_frame_time());

    println!("\n10. 颜色和样式");
    layer.set_background_color(cocos2d_rust::Color4B::new(0, 0, 0, 200));
    layer.set_text_color(cocos2d_rust::Color4B::new(255, 255, 255, 255));
    layer.set_font_size(16.0);
    layer.set_opacity(180);
    println!("   背景色、文本色、字体大小、透明度已设置");

    println!("\n11. 清理功能");
    layer.clear_console();
    layer.clear_profiler();
    println!("   控制台和性能分析器已清理");

    layer.clear_all();
    println!("   所有调试数据已清理");

    println!("\n12. 完整报告");
    println!("{}", layer.generate_full_report());

    println!("\n13. 摘要信息");
    println!("{}", layer.generate_summary());

    println!("\nDebugLayer 演示完成!\n");
}

fn main() {
    println!("\n========================================");
    println!("  Cocos2d-Rust Phase 10 Demo");
    println!("  调试系统 (Debug System)");
    println!("========================================\n");

    demo_debug_stats();
    demo_debug_console();
    demo_debug_profiler();
    demo_debug_layer();

    println!("========================================");
    println!("  Phase 10 演示完成!");
    println!("========================================\n");

    println!("\n调试系统包含:");
    println!("  - DebugStats:    FPS、帧时间、内存、绘制调用统计");
    println!("  - DebugConsole:  日志输出、命令执行、历史记录");
    println!("  - DebugProfiler: 函数分析、热点检测、调用追踪");
    println!("  - DebugLayer:    集成调试 UI、面板管理、图形显示");
}

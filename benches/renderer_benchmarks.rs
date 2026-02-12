/// 性能基准测试
///
/// 用于验证优化效果，对比 Cocos2d-x 的问题场景

use cocos2d_rust::renderer::{
    BatchKey, OptimizedBatchRenderer, OptimizedVertex, RenderCommand,
};
use cocos2d_rust::math::{Vec2, Vec3};
use cocos2d_rust::base::types::Color4F;
use std::time::Instant;

/// 基准测试结果
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub name: String,
    pub iterations: usize,
    pub total_time_ms: f64,
    pub avg_time_ms: f64,
    pub min_time_ms: f64,
    pub max_time_ms: f64,
    pub ops_per_sec: f64,
}

impl BenchmarkResult {
    pub fn new(name: String, iterations: usize, times: Vec<f64>) -> Self {
        let total_time_ms: f64 = times.iter().sum();
        let avg_time_ms = total_time_ms / iterations as f64;
        let min_time_ms = times.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_time_ms = times.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let ops_per_sec = 1000.0 / avg_time_ms;

        Self {
            name,
            iterations,
            total_time_ms,
            avg_time_ms,
            min_time_ms,
            max_time_ms,
            ops_per_sec,
        }
    }

    pub fn print(&self) {
        println!("\n=== {} ===", self.name);
        println!("  Iterations: {}", self.iterations);
        println!("  Total time: {:.3} ms", self.total_time_ms);
        println!("  Average:    {:.3} ms", self.avg_time_ms);
        println!("  Min:        {:.3} ms", self.min_time_ms);
        println!("  Max:        {:.3} ms", self.max_time_ms);
        println!("  Throughput: {:.0} ops/sec", self.ops_per_sec);
    }
}

/// 性能基准测试套件
pub struct BenchmarkSuite {
    results: Vec<BenchmarkResult>,
}

impl BenchmarkSuite {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }

    /// 运行基准测试
    pub fn run<F>(&mut self, name: &str, iterations: usize, mut test_fn: F)
    where
        F: FnMut() -> (),
    {
        println!("\nRunning benchmark: {}", name);
        let mut times = Vec::with_capacity(iterations);

        // 预热
        for _ in 0..10 {
            test_fn();
        }

        // 实际测试
        for i in 0..iterations {
            let start = Instant::now();
            test_fn();
            let elapsed = start.elapsed().as_secs_f64() * 1000.0;
            times.push(elapsed);

            if (i + 1) % (iterations / 10).max(1) == 0 {
                print!(".");
                std::io::Write::flush(&mut std::io::stdout()).ok();
            }
        }
        println!(" Done!");

        let result = BenchmarkResult::new(name.to_string(), iterations, times);
        result.print();
        self.results.push(result);
    }

    /// 打印对比结果
    pub fn print_comparison(&self, baseline_name: &str, optimized_name: &str) {
        let baseline = self.results.iter().find(|r| r.name == baseline_name);
        let optimized = self.results.iter().find(|r| r.name == optimized_name);

        if let (Some(baseline), Some(optimized)) = (baseline, optimized) {
            println!("\n=== Performance Comparison ===");
            println!("Baseline:  {} - {:.3} ms", baseline.name, baseline.avg_time_ms);
            println!("Optimized: {} - {:.3} ms", optimized.name, optimized.avg_time_ms);
            
            let speedup = baseline.avg_time_ms / optimized.avg_time_ms;
            let improvement = ((baseline.avg_time_ms - optimized.avg_time_ms) / baseline.avg_time_ms) * 100.0;
            
            println!("Speedup:   {:.2}x", speedup);
            println!("Improvement: {:.1}%", improvement);
        }
    }

    /// 打印所有结果
    pub fn print_summary(&self) {
        println!("\n=== Benchmark Summary ===");
        for result in &self.results {
            println!("{:30} {:8.3} ms  ({:.0} ops/sec)", 
                     result.name, result.avg_time_ms, result.ops_per_sec);
        }
    }
}

/// 批处理渲染器基准测试
pub fn bench_batch_renderer() {
    let mut suite = BenchmarkSuite::new();

    // 测试 1: 提交 1000 个相同批次键的命令（最佳情况）
    suite.run("Batch: 1000 quads, same key", 100, || {
        let mut renderer = OptimizedBatchRenderer::new();
        let key = BatchKey::new(1, 100, 1, 0, 0);

        for _ in 0..1000 {
            let mut command = RenderCommand::new(key);
            command.add_quad(
                [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0]],
                [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
                [1.0, 1.0, 1.0, 1.0],
            );
            renderer.submit(command);
        }
        renderer.flush();
    });

    // 测试 2: 提交 1000 个不同批次键的命令（最坏情况）
    suite.run("Batch: 1000 quads, different keys", 100, || {
        let mut renderer = OptimizedBatchRenderer::new();

        for i in 0..1000 {
            let key = BatchKey::new(i, i * 100, i, 0, 0);
            let mut command = RenderCommand::new(key);
            command.add_quad(
                [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0]],
                [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
                [1.0, 1.0, 1.0, 1.0],
            );
            renderer.submit(command);
        }
        renderer.flush();
    });

    // 测试 3: 提交 10000 个命令，10 个不同的键（真实场景）
    suite.run("Batch: 10000 quads, 10 keys", 100, || {
        let mut renderer = OptimizedBatchRenderer::new();

        for i in 0..10000 {
            let key = BatchKey::new(i % 10, (i % 10) * 100, i % 10, 0, i as i32 / 1000);
            let mut command = RenderCommand::new(key);
            command.add_quad(
                [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0]],
                [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
                [1.0, 1.0, 1.0, 1.0],
            );
            renderer.submit(command);
        }
        renderer.flush();
    });

    suite.print_summary();
}

/// 内存分配基准测试
pub fn bench_memory_allocation() {
    let mut suite = BenchmarkSuite::new();

    // 测试 1: 顶点缓冲复用（优化版）
    suite.run("Memory: Vertex buffer reuse", 1000, || {
        let mut renderer = OptimizedBatchRenderer::new();
        let key = BatchKey::default_key();

        for _ in 0..100 {
            let mut command = RenderCommand::new(key);
            command.add_quad(
                [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0]],
                [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
                [1.0, 1.0, 1.0, 1.0],
            );
            renderer.submit(command);
        }
        renderer.flush();
        // 缓冲区被复用，无需重新分配
    });

    // 测试 2: 每次都创建新的顶点缓冲（模拟未优化版本）
    suite.run("Memory: No buffer reuse", 1000, || {
        for _ in 0..100 {
            let mut renderer = OptimizedBatchRenderer::new();
            let key = BatchKey::default_key();
            let mut command = RenderCommand::new(key);
            command.add_quad(
                [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0]],
                [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
                [1.0, 1.0, 1.0, 1.0],
            );
            renderer.submit(command);
            renderer.flush();
        }
        // 每次都重新分配
    });

    suite.print_comparison("Memory: No buffer reuse", "Memory: Vertex buffer reuse");
    suite.print_summary();
}

/// 批次键比较基准测试
pub fn bench_batch_key_comparison() {
    let mut suite = BenchmarkSuite::new();

    let keys: Vec<BatchKey> = (0..1000)
        .map(|i| BatchKey::new(i % 10, (i % 10) * 100, i % 10, 0, 0))
        .collect();

    // 测试 1: 结构体相等比较（优化版）
    suite.run("Comparison: Struct equality", 10000, || {
        let key1 = &keys[0];
        for key2 in &keys {
            let _ = key1 == key2;
        }
    });

    // 测试 2: 模拟函数调用比较（Cocos2d-x 风格）
    fn can_batch_functional(k1: &BatchKey, k2: &BatchKey) -> bool {
        k1.material_id == k2.material_id
            && k1.texture_id == k2.texture_id
            && k1.shader_id == k2.shader_id
            && k1.blend_mode == k2.blend_mode
    }

    suite.run("Comparison: Function calls", 10000, || {
        let key1 = &keys[0];
        for key2 in &keys {
            let _ = can_batch_functional(key1, key2);
        }
    });

    suite.print_comparison("Comparison: Function calls", "Comparison: Struct equality");
    suite.print_summary();
}

/// 运行所有基准测试
pub fn run_all_benchmarks() {
    println!("=".repeat(60));
    println!("Cocos2d-Rust Performance Benchmarks");
    println!("=".repeat(60));

    println!("\n1. Batch Renderer Performance");
    bench_batch_renderer();

    println!("\n2. Memory Allocation Performance");
    bench_memory_allocation();

    println!("\n3. Batch Key Comparison Performance");
    bench_batch_key_comparison();

    println!("\n{}", "=".repeat(60));
    println!("All benchmarks completed!");
    println!("{}", "=".repeat(60));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_benchmark_result_creation() {
        let times = vec![10.0, 20.0, 30.0, 40.0, 50.0];
        let result = BenchmarkResult::new("test".to_string(), 5, times);

        assert_eq!(result.iterations, 5);
        assert_eq!(result.total_time_ms, 150.0);
        assert_eq!(result.avg_time_ms, 30.0);
        assert_eq!(result.min_time_ms, 10.0);
        assert_eq!(result.max_time_ms, 50.0);
    }

    #[test]
    fn test_benchmark_suite() {
        let mut suite = BenchmarkSuite::new();
        
        suite.run("simple_test", 10, || {
            std::thread::sleep(std::time::Duration::from_micros(100));
        });

        assert_eq!(suite.results.len(), 1);
        assert_eq!(suite.results[0].name, "simple_test");
        assert_eq!(suite.results[0].iterations, 10);
    }

    #[test]
    fn test_batch_key_equality() {
        let key1 = BatchKey::new(1, 100, 1, 0, 0);
        let key2 = BatchKey::new(1, 100, 1, 0, 0);
        let key3 = BatchKey::new(2, 100, 1, 0, 0);

        assert_eq!(key1, key2);
        assert_ne!(key1, key3);
    }
}

// 示例：如何运行基准测试
#[cfg(not(test))]
fn main() {
    run_all_benchmarks();
}

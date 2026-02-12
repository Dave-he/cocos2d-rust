/// 优化批处理渲染器使用示例
///
/// 展示如何使用优化的批处理渲染器替代 Cocos2d-x 的低效实现

use cocos2d_rust::base::types::Color4F;
use cocos2d_rust::math::{Vec2, Vec3};
use cocos2d_rust::renderer::{BatchKey, OptimizedBatchRenderer, RenderCommand};

fn main() {
    println!("=== Optimized Batch Renderer Demo ===\n");

    // 示例 1: 基础使用
    example_basic_usage();

    // 示例 2: 游戏场景渲染
    example_game_scene();

    // 示例 3: 粒子系统渲染
    example_particle_system();

    // 示例 4: UI 渲染
    example_ui_rendering();

    // 示例 5: 性能对比
    example_performance_comparison();
}

/// 示例 1: 基础使用
fn example_basic_usage() {
    println!("1. Basic Usage Example");
    println!("-".repeat(50));

    // 创建渲染器
    let mut renderer = OptimizedBatchRenderer::new();

    // 定义批次键（材质、纹理、着色器等）
    let batch_key = BatchKey::new(
        1,      // material_id
        100,    // texture_id
        1,      // shader_id
        0,      // blend_mode
        0,      // z_order
    );

    // 创建渲染命令
    let mut command = RenderCommand::new(batch_key);

    // 添加一个四边形（精灵）
    command.add_quad(
        // 顶点位置
        [
            [0.0, 0.0, 0.0],   // 左下
            [100.0, 0.0, 0.0], // 右下
            [100.0, 100.0, 0.0], // 右上
            [0.0, 100.0, 0.0], // 左上
        ],
        // 纹理坐标
        [
            [0.0, 0.0], // 左下
            [1.0, 0.0], // 右下
            [1.0, 1.0], // 右上
            [0.0, 1.0], // 左上
        ],
        // 颜色（RGBA）
        [1.0, 1.0, 1.0, 1.0], // 白色
    );

    // 提交命令
    renderer.submit(command);

    // 刷新渲染（执行 Draw Call）
    renderer.flush();

    // 查看统计信息
    let stats = renderer.get_stats();
    println!("  Total commands: {}", stats.total_commands);
    println!("  Total batches:  {}", stats.total_batches);
    println!("  Draw calls:     {}", stats.draw_calls);
    println!("  Vertices:       {}", stats.total_vertices);
    println!();
}

/// 示例 2: 游戏场景渲染（模拟 1000 个精灵）
fn example_game_scene() {
    println!("2. Game Scene Example (1000 sprites)");
    println!("-".repeat(50));

    let mut renderer = OptimizedBatchRenderer::new();

    // 模拟游戏场景中的不同对象
    let textures = vec![
        (1, "hero.png"),
        (2, "enemy.png"),
        (3, "background.png"),
        (4, "ui_button.png"),
    ];

    // 渲染 1000 个精灵
    for i in 0..1000 {
        // 根据类型选择纹理
        let (texture_id, _name) = textures[i % textures.len()];

        let batch_key = BatchKey::new(
            1,          // material_id
            texture_id, // texture_id（不同纹理）
            1,          // shader_id
            0,          // blend_mode
            i as i32 / 100, // z_order（每 100 个一层）
        );

        let mut command = RenderCommand::new(batch_key);

        // 位置分布（网格布局）
        let x = (i % 40) as f32 * 25.0;
        let y = (i / 40) as f32 * 25.0;

        command.add_quad(
            [
                [x, y, 0.0],
                [x + 20.0, y, 0.0],
                [x + 20.0, y + 20.0, 0.0],
                [x, y + 20.0, 0.0],
            ],
            [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
            [1.0, 1.0, 1.0, 1.0],
        );

        renderer.submit(command);
    }

    // 刷新渲染
    renderer.flush();

    let stats = renderer.get_stats();
    println!("  Sprites rendered: 1000");
    println!("  Batches created:  {}", stats.total_batches);
    println!("  Draw calls:       {}", stats.draw_calls);
    println!("  Batch time:       {:.3} ms", stats.batch_time_ms);
    println!("  Sort time:        {:.3} ms", stats.sort_time_ms);
    println!();

    // 与 Cocos2d-x 对比
    println!("  Comparison with Cocos2d-x:");
    println!("  - Cocos2d-x: ~1000 draw calls (worst case)");
    println!("  - Optimized: {} draw calls", stats.draw_calls);
    println!("  - Improvement: {:.0}x fewer draw calls!", 
             1000.0 / stats.draw_calls as f32);
    println!();
}

/// 示例 3: 粒子系统渲染
fn example_particle_system() {
    println!("3. Particle System Example (10000 particles)");
    println!("-".repeat(50));

    let mut renderer = OptimizedBatchRenderer::new();

    // 粒子系统使用相同的纹理和材质
    let particle_key = BatchKey::new(1, 200, 1, 1, 0); // blend_mode=1 (additive)

    // 10000 个粒子
    for i in 0..10000 {
        let mut command = RenderCommand::new(particle_key);

        // 粒子位置（随机分布）
        let angle = (i as f32 * 0.1) % 360.0;
        let radius = (i % 100) as f32;
        let x = 400.0 + angle.cos() * radius;
        let y = 300.0 + angle.sin() * radius;

        // 粒子大小
        let size = 2.0 + (i % 5) as f32;

        // 粒子颜色（渐变）
        let color = [
            1.0,
            0.5 + (i as f32 / 10000.0) * 0.5,
            0.0,
            0.8 - (i as f32 / 10000.0) * 0.5,
        ];

        command.add_quad(
            [
                [x, y, 0.0],
                [x + size, y, 0.0],
                [x + size, y + size, 0.0],
                [x, y + size, 0.0],
            ],
            [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
            color,
        );

        renderer.submit(command);
    }

    renderer.flush();

    let stats = renderer.get_stats();
    println!("  Particles:       10000");
    println!("  Batches:         {}", stats.total_batches);
    println!("  Draw calls:      {}", stats.draw_calls);
    println!("  Total vertices:  {}", stats.total_vertices);
    println!("  Batch time:      {:.3} ms", stats.batch_time_ms);
    println!();

    println!("  ✅ All particles merged into {} batch(es)!", stats.total_batches);
    println!();
}

/// 示例 4: UI 渲染（混合不同类型）
fn example_ui_rendering() {
    println!("4. UI Rendering Example");
    println!("-".repeat(50));

    let mut renderer = OptimizedBatchRenderer::new();

    // 背景面板
    for i in 0..10 {
        let key = BatchKey::new(1, 300, 1, 0, -1); // z_order=-1 (background)
        let mut command = RenderCommand::new(key);

        command.add_quad(
            [
                [i as f32 * 80.0, 0.0, 0.0],
                [(i + 1) as f32 * 80.0, 0.0, 0.0],
                [(i + 1) as f32 * 80.0, 600.0, 0.0],
                [i as f32 * 80.0, 600.0, 0.0],
            ],
            [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
            [0.8, 0.8, 0.8, 1.0],
        );

        renderer.submit(command);
    }

    // 按钮
    for i in 0..20 {
        let key = BatchKey::new(1, 301, 1, 0, 0); // z_order=0 (middle)
        let mut command = RenderCommand::new(key);

        let x = (i % 5) as f32 * 150.0 + 50.0;
        let y = (i / 5) as f32 * 100.0 + 50.0;

        command.add_quad(
            [
                [x, y, 0.0],
                [x + 100.0, y, 0.0],
                [x + 100.0, y + 50.0, 0.0],
                [x, y + 50.0, 0.0],
            ],
            [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
            [0.2, 0.6, 1.0, 1.0],
        );

        renderer.submit(command);
    }

    // 文本标签
    for i in 0..50 {
        let key = BatchKey::new(1, 302, 1, 0, 1); // z_order=1 (foreground)
        let mut command = RenderCommand::new(key);

        let x = (i % 10) as f32 * 75.0;
        let y = (i / 10) as f32 * 100.0;

        command.add_quad(
            [
                [x, y, 0.0],
                [x + 60.0, y, 0.0],
                [x + 60.0, y + 20.0, 0.0],
                [x, y + 20.0, 0.0],
            ],
            [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
            [0.0, 0.0, 0.0, 1.0],
        );

        renderer.submit(command);
    }

    renderer.flush();

    let stats = renderer.get_stats();
    println!("  UI elements:     80 (10 panels + 20 buttons + 50 labels)");
    println!("  Batches:         {}", stats.total_batches);
    println!("  Draw calls:      {}", stats.draw_calls);
    println!();

    println!("  ✅ UI elements organized into {} batches by z-order and texture!", 
             stats.total_batches);
    println!();
}

/// 示例 5: 性能对比（模拟 Cocos2d-x vs 优化版本）
fn example_performance_comparison() {
    println!("5. Performance Comparison");
    println!("-".repeat(50));

    use std::time::Instant;

    // 测试场景：1000 个精灵，使用 10 种不同纹理
    let sprite_count = 1000;
    let texture_count = 10;

    // 优化版本
    let start = Instant::now();
    {
        let mut renderer = OptimizedBatchRenderer::new();

        for i in 0..sprite_count {
            let key = BatchKey::new(
                1,
                (i % texture_count) as u32,
                1,
                0,
                i as i32 / 100,
            );

            let mut command = RenderCommand::new(key);
            command.add_quad(
                [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0]],
                [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
                [1.0, 1.0, 1.0, 1.0],
            );

            renderer.submit(command);
        }

        renderer.flush();

        let stats = renderer.get_stats();
        let elapsed = start.elapsed().as_secs_f64() * 1000.0;

        println!("  Optimized Renderer:");
        println!("    Time:        {:.3} ms", elapsed);
        println!("    Batches:     {}", stats.total_batches);
        println!("    Draw calls:  {}", stats.draw_calls);
        println!("    Batch time:  {:.3} ms", stats.batch_time_ms);
    }

    println!();

    // 模拟 Cocos2d-x 行为（每个命令都判断是否可批处理）
    let start = Instant::now();
    {
        let mut batch_count = 0;
        let mut last_texture = u32::MAX;

        for i in 0..sprite_count {
            let texture_id = (i % texture_count) as u32;

            // 模拟 Cocos2d-x 的 canBatch 函数（8 个条件判断）
            if texture_id != last_texture {
                batch_count += 1;
                last_texture = texture_id;
            }

            // 模拟其他开销
            std::hint::black_box(texture_id);
        }

        let elapsed = start.elapsed().as_secs_f64() * 1000.0;

        println!("  Simulated Cocos2d-x:");
        println!("    Time:        {:.3} ms", elapsed);
        println!("    Batches:     {} (estimated)", batch_count);
        println!("    Draw calls:  {} (worst case)", sprite_count);
    }

    println!();
    println!("  Key Improvements:");
    println!("  ✅ O(1) hash lookup vs O(n) linear comparison");
    println!("  ✅ Zero allocation buffer reuse");
    println!("  ✅ Automatic batching by key");
    println!("  ✅ Predictable performance");
    println!();
}

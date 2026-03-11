/// Phase 9 Demo - 相机跟随和调试工具演示
/// 
/// 展示 CameraFollow、DebugStats 的功能

use cocos2d_rust::action::CameraFollow;
use cocos2d_rust::base::DebugStats;
use cocos2d_rust::camera::Camera2D;
use cocos2d_rust::base::Node;
use cocos2d_rust::math::Vec2;

fn main() {
    println!("=== Phase 9 Demo: 相机跟随和调试工具演示 ===\n");
    
    println!("--- CameraFollow 相机跟随动作 ---");
    demo_camera_follow();
    println!();
    
    println!("--- DebugStats 性能统计 ---");
    demo_debug_stats();
    println!();
    
    println!("--- 集成演示 ---");
    demo_integration();
    
    println!("\n=== Phase 9 Demo 完成 ===");
}

fn demo_camera_follow() {
    // 1. 创建目标节点和相机
    println!("1. 创建目标节点和相机:");
    let target = Node::new();
    target.borrow_mut().set_position(Vec2::new(100.0, 200.0));
    println!("   目标位置: {:?}", target.borrow().get_position());
    
    let camera = Node::new();
    camera.borrow_mut().set_position(Vec2::ZERO);
    println!("   相机初始位置: {:?}", camera.borrow().get_position());
    
    // 2. 创建相机跟随动作
    println!("\n2. 创建相机跟随动作:");
    let mut follow = CameraFollow::with_target(target.clone());
    follow.set_camera(camera.clone());
    println!("   CameraFollow 创建成功");
    
    // 3. 带偏移的跟随
    println!("\n3. 带偏移的相机跟随:");
    let follow_with_offset = CameraFollow::with_target(target.clone())
        .with_offset(Vec2::new(50.0, 100.0));
    println!("   偏移量: {:?}", follow_with_offset.get_offset());
    
    // 4. 平滑跟随
    println!("\n4. 平滑跟随 (lerp=0.5):");
    let smooth_follow = CameraFollow::with_target(target.clone())
        .with_lerp(0.5);
    println!("   平滑系数: {}", smooth_follow.get_lerp_factor());
    
    // 5. 带边界限制的跟随
    println!("\n5. 带边界限制的相机跟随:");
    let bounded_follow = CameraFollow::with_target(target.clone())
        .with_boundary(Vec2::new(0.0, 0.0), Vec2::new(800.0, 600.0));
    println!("   边界最小: {:?}", bounded_follow.get_bounds_min());
    println!("   边界最大: {:?}", bounded_follow.get_bounds_max());
    assert!(bounded_follow.get_bounds_enabled());
    
    // 6. 世界矩形边界
    println!("\n6. 世界矩形边界:");
    let world_follow = CameraFollow::with_target(target.clone())
        .with_world_rect(0.0, 0.0, 1600.0, 1200.0);
    assert!(world_follow.get_bounds_enabled());
    assert!(world_follow.get_boundary_set());
    println!("   世界矩形: {:?}", world_follow.get_world_rect());
    
    // 7. 模拟相机跟随更新
    println!("\n7. 模拟相机跟随:");
    target.borrow_mut().set_position(Vec2::new(300.0, 400.0));
    
    // 模拟 step 调用
    let mut follow_action = CameraFollow::with_target(target.clone());
    follow_action.set_camera(camera.clone());
    
    // 初始相机位置
    println!("   初始相机位置: {:?}", camera.borrow().get_position());
    println!("   目标位置: {:?}", target.borrow().get_position());
    
    // 8. Camera2D 集成
    println!("\n8. Camera2D 集成:");
    let mut camera2d = Camera2D::new();
    camera2d.set_position(Vec2::new(0.0, 0.0));
    println!("   Camera2D 位置: {:?}", camera2d.get_position());
    
    camera2d.set_zoom(1.5);
    println!("   Camera2D 缩放: {}", camera2d.get_zoom());
    
    camera2d.set_rotation(45.0);
    println!("   Camera2D 旋转: {}度", camera2d.get_rotation());
}

fn demo_debug_stats() {
    // 1. 创建调试统计
    println!("1. 创建调试统计:");
    let mut stats = DebugStats::new();
    println!("   初始启用状态: {}", stats.is_enabled());
    println!("   初始可见状态: {}", stats.is_visible());
    
    // 2. 启用/禁用
    println!("\n2. 启用/禁用统计:");
    stats.set_enabled(false);
    println!("   禁用后状态: {}", stats.is_enabled());
    
    stats.set_enabled(true);
    println!("   启用后状态: {}", stats.is_enabled());
    
    // 3. 显示/隐藏
    println!("\n3. 显示/隐藏控制:");
    stats.set_visible(false);
    println!("   隐藏后状态: {}", stats.is_visible());
    
    stats.set_visible(true);
    println!("   显示后状态: {}", stats.is_visible());
    
    // 4. 帧统计
    println!("\n4. 帧统计:");
    stats.begin_frame();
    std::thread::sleep(std::time::Duration::from_millis(16));
    stats.end_frame();
    
    stats.begin_frame();
    std::thread::sleep(std::time::Duration::from_millis(16));
    stats.end_frame();
    
    stats.begin_frame();
    std::thread::sleep(std::time::Duration::from_millis(16));
    stats.end_frame();
    
    println!("   帧计数: {}", stats.get_frame_count());
    println!("   FPS: {}", stats.get_fps_string());
    println!("   帧时间: {}", stats.get_frame_time_string());
    
    // 5. 渲染统计
    println!("\n5. 渲染统计:");
    stats.add_draw_call(100);
    stats.add_triangles(5000);
    stats.add_vertices(15000);
    
    println!("   绘制调用: {}", stats.get_draw_calls());
    println!("   三角形: {}", stats.get_triangles());
    println!("   顶点: {}", stats.get_vertices());
    
    // 6. 内存统计
    println!("\n6. 内存统计:");
    stats.set_memory_usage(1024 * 1024 * 50); // 50 MB
    println!("   内存使用: {}", stats.get_memory_usage_string());
    
    stats.set_memory_usage(1024 * 1024 * 1024 * 2); // 2 GB
    println!("   内存使用: {}", stats.get_memory_usage_string());
    
    // 7. 自定义统计
    println!("\n7. 自定义统计:");
    stats.set_int_stat("enemies", 10);
    stats.set_float_stat("health", 99.5);
    stats.set_int_stat("score", 15000);
    
    use cocos2d_rust::base::debug_stats::StatValue;
    println!("   敌人数: {}", match stats.get_stat("enemies").unwrap() {
        StatValue::Integer(v) => v.to_string(),
        _ => String::new(),
    });
    
    println!("   生命值: {}", match stats.get_stat("health").unwrap() {
        StatValue::Float(v) => format!("{:.1}", v),
        _ => String::new(),
    });
    
    // 8. 生成报告
    println!("\n8. 生成统计报告:");
    let report = stats.generate_report();
    let lines: Vec<&str> = report.lines().take(5).collect();
    for line in lines {
        println!("   {}", line);
    }
    println!("   ...");
    
    // 9. 重置统计
    println!("\n9. 重置统计:");
    stats.reset();
    println!("   重置后帧计数: {}", stats.get_frame_count());
    println!("   重置后绘制调用: {}", stats.get_draw_calls());
    
    // 10. FPS 历史
    println!("\n10. FPS 历史配置:");
    stats.set_fps_history_size(30);
    println!("   FPS 历史大小: 30");
}

fn demo_integration() {
    println!("\n=== 集成演示 ===\n");
    
    // 1. 相机跟随 + 调试统计
    println!("1. 相机跟随 + 调试统计:");
    let target = Node::new();
    target.borrow_mut().set_position(Vec2::new(500.0, 500.0));
    
    let camera = Node::new();
    camera.borrow_mut().set_position(Vec2::ZERO);
    
    let mut follow = CameraFollow::with_target(target.clone())
        .with_offset(Vec2::new(-100.0, -50.0))
        .with_lerp(0.1)
        .with_boundary(Vec2::new(100.0, 100.0), Vec2::new(900.0, 900.0));
    
    follow.set_camera(camera.clone());
    
    // 2. 游戏循环模拟
    println!("2. 游戏循环模拟:");
    let mut stats = DebugStats::new();
    stats.set_update_interval(std::time::Duration::from_millis(100));
    
    for i in 0..5 {
        // 模拟目标移动
        let new_pos = Vec2::new(500.0 + i as f32 * 50.0, 500.0 + i as f32 * 30.0);
        target.borrow_mut().set_position(new_pos);
        
        // 模拟帧更新
        stats.begin_frame();
        
        // 模拟相机跟随更新
        let camera_pos = camera.borrow().get_position();
        let target_pos = target.borrow().get_position() + Vec2::new(-100.0, -50.0);
        let new_camera_pos = Vec2::new(
            camera_pos.x + (target_pos.x - camera_pos.x) * 0.1,
            camera_pos.y + (target_pos.y - camera_pos.y) * 0.1,
        );
        camera.borrow_mut().set_position(new_camera_pos);
        
        // 模拟渲染统计
        stats.add_draw_call(50 + i * 10);
        stats.add_triangles(2000 + i * 500);
        stats.add_vertices(6000 + i * 1500);
        
        std::thread::sleep(std::time::Duration::from_millis(16));
        
        stats.end_frame();
        
        println!("   帧 {}: 目标位置 {:?}, 相机位置 {:?}", 
            i + 1, 
            target.borrow().get_position(),
            camera.borrow().get_position());
    }
    
    // 3. 最终统计
    println!("\n3. 最终统计:");
    println!("   总帧数: {}", stats.get_frame_count());
    println!("   平均 FPS: {}", stats.get_fps_string());
    println!("   总绘制调用: {}", stats.get_draw_calls());
    println!("   总三角形: {}", stats.get_triangles());
    
    // 4. 调试报告
    println!("\n4. 调试报告预览:");
    let report = stats.generate_report();
    let lines: Vec<&str> = report.lines().take(8).collect();
    for line in lines {
        println!("   {}", line);
    }
    
    // 5. 性能建议
    println!("\n5. 性能分析:");
    let fps = stats.get_fps();
    if fps >= 55.0 {
        println!("   性能优秀 (FPS: {:.1})", fps);
    } else if fps >= 30.0 {
        println!("   性能一般 (FPS: {:.1})", fps);
    } else {
        println!("   性能需要优化 (FPS: {:.1})", fps);
    }
    
    let triangles = stats.get_triangles();
    if triangles < 10000 {
        println!("   三角形数量适中 ({})", triangles);
    } else if triangles < 50000 {
        println!("   三角形数量较多 ({})", triangles);
    } else {
        println!("   三角形数量过多 ({})", triangles);
    }
    
    println!("\n所有演示完成！");
}

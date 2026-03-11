/// # Cocos2D-Rust 使用示例
/// 
/// 本示例展示了如何使用 cocos2d-rust 引擎的主要功能

use cocos2d_rust::*;

fn main() {
    println!("Cocos2D-Rust 示例程序");
    println!("======================\n");
    
    example_motion_streak();
    example_particle_presets();
    example_easing_functions();
    example_scene_graph();
    example_draw_node();
}

/// 示例 1: MotionStreak - 轨迹效果
fn example_motion_streak() {
    println!("1. MotionStreak 轨迹效果示例");
    println!("------------------------------");
    
    let mut streak = MotionStreak::new(
        2.0,              // fade_time: 2秒淡出
        5.0,              // min_seg: 最小移动距离 5 像素
        3.0,              // stroke: 线宽 3 像素
        Color4F::WHITE,   // 白色轨迹
        None              // 无纹理
    );
    
    println!("  创建轨迹: fade_time={}, min_seg={}, stroke={}", 
        streak.get_fade_time(), streak.get_min_seg(), streak.get_stroke());
    
    // 模拟移动并更新轨迹
    let positions = vec![
        Vec2::new(0.0, 0.0),
        Vec2::new(100.0, 0.0),
        Vec2::new(200.0, 50.0),
        Vec2::new(300.0, 100.0),
    ];
    
    for (i, pos) in positions.iter().enumerate() {
        streak.update(0.016, *pos);  // 60 FPS
        println!("  帧 {}: 位置 {:?}, 轨迹点数: {}", 
            i, pos, streak.get_point_count());
    }
    
    // 修改轨迹颜色
    streak.set_color(Color4F::RED);
    println!("  修改颜色为红色");
    
    // 重置轨迹
    streak.reset();
    println!("  重置轨迹后点数: {}\n", streak.get_point_count());
}

/// 示例 2: ParticlePresets - 粒子效果预设
fn example_particle_presets() {
    println!("2. ParticlePresets 粒子效果预设");
    println!("--------------------------------");
    
    // 创建不同的粒子效果
    let effects = vec![
        ("火焰", ParticlePresets::create_fire()),
        ("烟雾", ParticlePresets::create_smoke()),
        ("爆炸", ParticlePresets::create_explosion()),
        ("下雪", ParticlePresets::create_snow()),
        ("下雨", ParticlePresets::create_rain()),
        ("星系", ParticlePresets::create_galaxy()),
        ("烟花", ParticlePresets::create_fireworks()),
        ("螺旋", ParticlePresets::create_spiral()),
    ];
    
    for (name, system) in effects {
        println!("  {}: {} 个粒子", name, system.get_capacity());
    }
    
    // 使用火焰效果
    let mut fire = ParticlePresets::create_fire();
    fire.start();
    println!("\n  启动火焰效果");
    
    // 模拟更新
    for _ in 0..5 {
        fire.update(0.016);
    }
    println!("  更新 5 帧后，活跃粒子数: {}\n", fire.get_particle_count());
}

/// 示例 3: Easing Functions - 缓动函数
fn example_easing_functions() {
    println!("3. Easing Functions 缓动函数");
    println!("----------------------------");
    
    // 测试不同的缓动函数
    let time_points = vec![0.0, 0.25, 0.5, 0.75, 1.0];
    
    println!("  时间点: {:?}\n", time_points);
    
    // 线性 vs 缓入
    let linear_values: Vec<f32> = time_points.iter().map(|&t| t).collect();
    let ease_in = EaseIn::new(2.0);
    let ease_in_values: Vec<f32> = time_points.iter()
        .map(|&t| ease_in.ease(t))
        .collect();
    
    println!("  线性:     {:?}", 
        linear_values.iter().map(|v| format!("{:.2}", v)).collect::<Vec<_>>());
    println!("  缓入:     {:?}", 
        ease_in_values.iter().map(|v| format!("{:.2}", v)).collect::<Vec<_>>());
    
    // 弹跳效果
    let bounce = EaseBounceOut;
    let bounce_values: Vec<f32> = time_points.iter()
        .map(|&t| bounce.ease(t))
        .collect();
    println!("  弹跳:     {:?}", 
        bounce_values.iter().map(|v| format!("{:.2}", v)).collect::<Vec<_>>());
    
    // 弹性效果
    let elastic = EaseElasticOut::default();
    let elastic_values: Vec<f32> = time_points.iter()
        .map(|&t| elastic.ease(t))
        .collect();
    println!("  弹性:     {:?}\n", 
        elastic_values.iter().map(|v| format!("{:.2}", v)).collect::<Vec<_>>());
}

/// 示例 4: Scene Graph - 场景图管理
fn example_scene_graph() {
    println!("4. Scene Graph 场景图");
    println!("---------------------");
    
    let mut scene = Scene::new();
    println!("  创建场景");
    
    let mut layer = Layer::new();
    layer.set_position(Vec2::new(100.0, 100.0));
    println!("  创建图层，位置: {:?}", layer.get_position());
    
    // 创建子节点
    let mut child = Node::new();
    child.set_position(Vec2::new(50.0, 50.0));
    child.set_tag(100);
    println!("  创建子节点，位置: {:?}, 标签: {}", 
        child.get_position(), child.tag());
    
    println!("  场景图层次结构已建立\n");
}

/// 示例 5: DrawNode - 矢量绘图
fn example_draw_node() {
    println!("5. DrawNode 矢量绘图");
    println!("--------------------");
    
    let mut draw_node = DrawNode::new();
    
    // 绘制点
    draw_node.draw_point(Vec2::new(50.0, 50.0), 5.0, Color4F::RED);
    println!("  绘制红色点");
    
    // 绘制线
    draw_node.draw_line(
        Vec2::new(0.0, 0.0), 
        Vec2::new(100.0, 100.0), 
        Color4F::BLUE
    );
    println!("  绘制蓝色线");
    
    // 绘制矩形
    draw_node.draw_rect_corners(
        Vec2::new(10.0, 10.0), 
        Vec2::new(90.0, 90.0), 
        Color4F::GREEN
    );
    println!("  绘制绿色矩形");
    
    // 绘制圆形
    draw_node.draw_circle_simple(Vec2::new(50.0, 50.0), 30.0, 0.0, 32, Color4F::YELLOW);
    println!("  绘制黄色圆形");
    
    println!("  绘制命令数: {}", draw_node.get_command_count());
    
    // 清除绘图
    draw_node.clear();
    println!("  清除所有绘制内容\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_runs() {
        // 确保示例代码可以运行
        example_motion_streak();
        example_particle_presets();
        example_easing_functions();
        example_scene_graph();
        example_draw_node();
    }

    #[test]
    fn test_motion_streak_basic() {
        let mut streak = MotionStreak::create(1.0, 1.0, 1.0);
        assert_eq!(streak.get_point_count(), 0);
        
        streak.update(0.016, Vec2::new(0.0, 0.0));
        streak.update(0.016, Vec2::new(100.0, 0.0));
        
        assert!(streak.get_point_count() > 0);
    }

    #[test]
    fn test_particle_presets_variety() {
        let fire = ParticlePresets::create_fire();
        let smoke = ParticlePresets::create_smoke();
        let explosion = ParticlePresets::create_explosion();
        
        assert_eq!(fire.get_capacity(), 250);
        assert_eq!(smoke.get_capacity(), 200);
        assert_eq!(explosion.get_capacity(), 700);
    }

    #[test]
    fn test_easing_boundary() {
        let ease_in = EaseIn::new(2.0);
        assert_eq!(ease_in.ease(0.0), 0.0);
        assert_eq!(ease_in.ease(1.0), 1.0);
    }
}

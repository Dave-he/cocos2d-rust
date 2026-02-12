/// # 粒子效果展示示例
/// 
/// 展示所有预设粒子效果的特性和用法

use cocos2d_rust::*;

fn main() {
    println!("════════════════════════════════════════");
    println!("  Cocos2D-Rust 粒子效果展示");
    println!("════════════════════════════════════════\n");
    
    showcase_all_effects();
    println!("\n");
    
    custom_particle_effect();
    println!("\n");
    
    particle_lifecycle_demo();
}

/// 展示所有预设效果
fn showcase_all_effects() {
    println!("【预设粒子效果列表】");
    println!("────────────────────────────────────────");
    
    let effects = vec![
        ("🔥 火焰 (Fire)", ParticlePresets::create_fire(), 
         "向上飘散的橙红色火焰效果"),
        ("💨 烟雾 (Smoke)", ParticlePresets::create_smoke(), 
         "缓慢上升的灰白色烟雾"),
        ("💥 爆炸 (Explosion)", ParticlePresets::create_explosion(), 
         "360度扩散的红色爆炸"),
        ("❄️  下雪 (Snow)", ParticlePresets::create_snow(), 
         "缓慢飘落的白色雪花"),
        ("🌧️  下雨 (Rain)", ParticlePresets::create_rain(), 
         "快速下落的蓝白雨滴"),
        ("🌌 星系 (Galaxy)", ParticlePresets::create_galaxy(), 
         "螺旋运动的蓝色星系"),
        ("🎆 烟花 (Fireworks)", ParticlePresets::create_fireworks(), 
         "绚丽的烟花爆炸效果"),
        ("🌀 螺旋 (Spiral)", ParticlePresets::create_spiral(), 
         "优雅的螺旋形粒子流"),
    ];
    
    for (i, (name, system, desc)) in effects.iter().enumerate() {
        println!("{}. {}", i + 1, name);
        println!("   描述: {}", desc);
        println!("   粒子数: {}", system.get_capacity());
        println!("   发射率: {:.0} 粒子/秒", system.get_config().emission_rate);
        println!("   生命周期: {:.1} 秒", system.get_config().life);
        println!();
    }
}

/// 自定义粒子效果
fn custom_particle_effect() {
    println!("【自定义粒子效果】");
    println!("────────────────────────────────────────");
    
    // 创建自定义配置
    let mut system = ParticleSystem::new();
    let mut config = ParticleEmitterConfig::default();
    
    // 魔法光环效果
    config.emitter_type = EmitterType::GRAVITY;
    config.blend_type = BlendType::ADD;
    config.total_particles = 300;
    config.emission_rate = 300.0;
    config.life = 2.0;
    config.life_var = 0.5;
    
    config.angle = 90.0;
    config.angle_var = 360.0;
    config.speed = 50.0;
    config.speed_var = 20.0;
    
    config.start_size = 20.0;
    config.start_size_var = 5.0;
    config.end_size = 5.0;
    config.end_size_var = 2.0;
    
    config.start_color = Color4F::new(0.5, 0.2, 1.0, 1.0);  // 紫色
    config.end_color = Color4F::new(0.2, 0.6, 1.0, 0.0);    // 蓝色淡出
    
    config.gravity = Vec3::ZERO;
    config.radial_accel = -50.0;
    config.tangential_accel = 30.0;
    
    system.set_config(config);
    
    println!("✨ 魔法光环效果");
    println!("   紫色到蓝色的渐变");
    println!("   径向加速: {}", system.get_config().radial_accel);
    println!("   切向加速: {}", system.get_config().tangential_accel);
    println!();
}

/// 粒子生命周期演示
fn particle_lifecycle_demo() {
    println!("【粒子系统生命周期】");
    println!("────────────────────────────────────────");
    
    let mut fire = ParticlePresets::create_fire();
    
    println!("1. 初始状态");
    println!("   活跃: {}", fire.is_active());
    println!("   可见: {}", fire.is_visible());
    println!("   粒子数: {}", fire.get_particle_count());
    println!();
    
    println!("2. 启动粒子系统");
    fire.start();
    println!("   活跃: {}", fire.is_active());
    println!();
    
    println!("3. 模拟更新 (60 FPS)");
    for frame in 1..=10 {
        fire.update(1.0 / 60.0);
        if frame % 3 == 0 {
            println!("   帧 {:2}: {} 个活跃粒子", 
                frame, fire.get_particle_count());
        }
    }
    println!();
    
    println!("4. 停止发射");
    fire.stop();
    println!("   活跃: {}", fire.is_active());
    println!();
    
    println!("5. 重置系统");
    fire.reset();
    println!("   粒子数: {}", fire.get_particle_count());
}

/// 粒子效果对比分析
#[allow(dead_code)]
fn compare_effects() {
    println!("【效果对比分析】");
    println!("────────────────────────────────────────");
    
    let effects = vec![
        ("火焰", ParticlePresets::create_fire()),
        ("烟花", ParticlePresets::create_fireworks()),
    ];
    
    println!("{:<10} {:>8} {:>8} {:>8} {:>10}", 
        "效果", "粒子数", "发射率", "生命期", "重力");
    println!("{}", "─".repeat(50));
    
    for (name, system) in effects {
        let config = system.get_config();
        println!("{:<10} {:>8} {:>8.0} {:>8.1} {:>10.0}", 
            name, 
            system.get_capacity(), 
            config.emission_rate,
            config.life,
            config.gravity.y
        );
    }
}

/// 性能基准测试
#[allow(dead_code)]
fn performance_benchmark() {
    println!("【性能基准测试】");
    println!("────────────────────────────────────────");
    
    let mut system = ParticlePresets::create_fireworks();  // 1500 粒子
    system.start();
    
    let frames = 1000;
    let dt = 1.0 / 60.0;
    
    println!("模拟 {} 帧 (60 FPS)", frames);
    
    let start = std::time::Instant::now();
    for _ in 0..frames {
        system.update(dt);
    }
    let duration = start.elapsed();
    
    println!("总耗时: {:.2} 秒", duration.as_secs_f64());
    println!("平均帧时间: {:.2} 毫秒", duration.as_secs_f64() * 1000.0 / frames as f64);
    println!("FPS: {:.0}", frames as f64 / duration.as_secs_f64());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_presets_create() {
        let effects = vec![
            ParticlePresets::create_fire(),
            ParticlePresets::create_smoke(),
            ParticlePresets::create_explosion(),
            ParticlePresets::create_snow(),
            ParticlePresets::create_rain(),
            ParticlePresets::create_galaxy(),
            ParticlePresets::create_fireworks(),
            ParticlePresets::create_spiral(),
        ];
        
        for system in effects {
            assert!(system.get_capacity() > 0);
        }
    }

    #[test]
    fn test_particle_lifecycle() {
        let mut system = ParticlePresets::create_fire();
        
        assert!(!system.is_active());
        
        system.start();
        assert!(system.is_active());
        
        system.update(0.016);
        assert!(system.get_particle_count() > 0 || system.is_active());
        
        system.stop();
        assert!(!system.is_active());
        
        system.reset();
        assert_eq!(system.get_particle_count(), 0);
    }

    #[test]
    fn test_custom_config() {
        let mut system = ParticleSystem::new();
        let mut config = ParticleEmitterConfig::default();
        
        config.total_particles = 500;
        config.life = 3.0;
        config.speed = 100.0;
        
        system.set_config(config);
        
        assert_eq!(system.get_capacity(), 500);
        assert_eq!(system.get_config().life, 3.0);
        assert_eq!(system.get_config().speed, 100.0);
    }
}

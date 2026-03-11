/// 独立测试新添加的功能
/// 
/// 测试 MotionStreak, ParticlePresets, Easing 功能

#[cfg(test)]
mod integration_tests {
    use cocos2d_rust::*;

    // ===== MotionStreak 测试 =====
    
    #[test]
    fn test_motion_streak_creation_and_config() {
        let streak = MotionStreak::new(
            2.0, 5.0, 3.0,
            Color4F::WHITE,
            None
        );
        
        assert_eq!(streak.get_fade_time(), 2.0);
        assert_eq!(streak.get_min_seg(), 5.0);
        assert_eq!(streak.get_stroke(), 3.0);
        assert_eq!(streak.get_point_count(), 0);
    }

    #[test]
    fn test_motion_streak_update() {
        let mut streak = MotionStreak::create(1.0, 10.0, 1.0);
        
        // 第一帧：初始化，建立起始位置（不添加点）
        streak.update(0.016, Vec2::new(0.0, 0.0));
        assert_eq!(streak.get_point_count(), 0);
        
        // 第二帧：移动超过 min_seg（10.0），位移为 20 > 10
        streak.update(0.016, Vec2::new(20.0, 0.0));
        assert_eq!(streak.get_point_count(), 1);
        
        // 第三帧：继续移动
        streak.update(0.016, Vec2::new(40.0, 0.0));
        assert_eq!(streak.get_point_count(), 2);
    }

    #[test]
    fn test_motion_streak_add_point() {
        let mut streak = MotionStreak::create(0.5, 1.0, 1.0);
        
        streak.add_point(Vec2::ZERO);
        assert_eq!(streak.get_point_count(), 1);
        
        streak.reset();
        assert_eq!(streak.get_point_count(), 0);
    }

    #[test]
    fn test_motion_streak_color_change() {
        let mut streak = MotionStreak::create(1.0, 1.0, 1.0);
        
        streak.set_color(Color4F::new(1.0, 0.0, 0.0, 1.0));
        let c = streak.get_color();
        assert!((c.r - 1.0).abs() < 0.01);
        assert!((c.g - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_motion_streak_reset() {
        let mut streak = MotionStreak::create(1.0, 1.0, 1.0);
        
        streak.add_point(Vec2::new(10.0, 10.0));
        streak.add_point(Vec2::new(20.0, 20.0));
        assert!(streak.get_point_count() > 0);
        
        streak.reset();
        assert_eq!(streak.get_point_count(), 0);
        assert!(!streak.is_starting_position_initialized());
    }

    // ===== ParticlePresets 测试 =====
    
    #[test]
    fn test_all_particle_presets() {
        let presets = vec![
            ("fire", ParticlePresets::create_fire(), 250u32),
            ("smoke", ParticlePresets::create_smoke(), 200),
            ("explosion", ParticlePresets::create_explosion(), 700),
            ("snow", ParticlePresets::create_snow(), 700),
            ("rain", ParticlePresets::create_rain(), 1000),
            ("galaxy", ParticlePresets::create_galaxy(), 200),
            ("fireworks", ParticlePresets::create_fireworks(), 1500),
            ("spiral", ParticlePresets::create_spiral(), 500),
        ];
        
        for (name, system, expected_capacity) in presets {
            assert_eq!(
                system.get_capacity(), 
                expected_capacity,
                "{} preset should have {} particles",
                name,
                expected_capacity
            );
        }
    }

    #[test]
    fn test_particle_system_lifecycle() {
        let mut system = ParticlePresets::create_fire();
        
        // 初始状态
        assert!(!system.is_active());
        assert_eq!(system.get_particle_count(), 0);
        
        // 启动
        system.start();
        assert!(system.is_active());
        
        // 更新
        for _ in 0..10 {
            system.update(0.016);
        }
        
        // 停止
        system.stop();
        assert!(!system.is_active());
        
        // 重置
        system.reset();
        assert_eq!(system.get_particle_count(), 0);
    }

    #[test]
    fn test_particle_config_modification() {
        let mut system = ParticleSystem::new();
        let mut config = ParticleEmitterConfig::default();
        
        config.total_particles = 500;
        config.life = 3.0;
        config.speed = 100.0;
        config.start_color = Color4F::new(1.0, 0.0, 0.0, 1.0);
        
        system.set_config(config);
        
        assert_eq!(system.get_capacity(), 500);
        assert_eq!(system.config.life, 3.0);
        assert_eq!(system.config.speed, 100.0);
    }

    // ===== Easing 函数测试 =====
    
    #[test]
    fn test_ease_in_boundary() {
        let ease = EaseIn::new(2.0);
        
        assert_eq!(ease.ease(0.0), 0.0);
        assert_eq!(ease.ease(1.0), 1.0);
        assert!(ease.ease(0.5) < 0.5, "EaseIn at 0.5 should be < 0.5");
    }

    #[test]
    fn test_ease_out_boundary() {
        let ease = EaseOut::new(2.0);
        
        assert_eq!(ease.ease(0.0), 0.0);
        assert_eq!(ease.ease(1.0), 1.0);
        assert!(ease.ease(0.5) > 0.5, "EaseOut at 0.5 should be > 0.5");
    }

    #[test]
    fn test_ease_in_out_boundary() {
        let ease = EaseInOut::new(2.0);
        
        assert_eq!(ease.ease(0.0), 0.0);
        assert_eq!(ease.ease(1.0), 1.0);
    }

    #[test]
    fn test_sine_easing() {
        let ease_in = EaseSineIn;
        let ease_out = EaseSineOut;
        let ease_in_out = EaseSineInOut;
        
        assert_eq!(ease_in.ease(0.0), 0.0);
        assert!((ease_in.ease(1.0) - 1.0).abs() < 0.0001);
        
        assert_eq!(ease_out.ease(0.0), 0.0);
        assert!((ease_out.ease(1.0) - 1.0).abs() < 0.0001);
        
        assert_eq!(ease_in_out.ease(0.0), 0.0);
        assert!((ease_in_out.ease(1.0) - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_exponential_easing() {
        let ease_in = EaseExponentialIn;
        let ease_out = EaseExponentialOut;
        
        assert_eq!(ease_in.ease(0.0), 0.0);
        assert_eq!(ease_out.ease(1.0), 1.0);
    }

    #[test]
    fn test_elastic_easing() {
        let ease_in = EaseElasticIn::default();
        let ease_out = EaseElasticOut::default();
        let ease_in_out = EaseElasticInOut::default();
        
        assert_eq!(ease_in.ease(0.0), 0.0);
        assert_eq!(ease_in.ease(1.0), 1.0);
        
        assert_eq!(ease_out.ease(0.0), 0.0);
        assert_eq!(ease_out.ease(1.0), 1.0);
        
        assert_eq!(ease_in_out.ease(0.0), 0.0);
        assert_eq!(ease_in_out.ease(1.0), 1.0);
    }

    #[test]
    fn test_elastic_with_custom_period() {
        let ease1 = EaseElasticOut::new(0.3);
        let ease2 = EaseElasticOut::new(0.6);
        
        // 不同周期应该产生不同的值
        let v1 = ease1.ease(0.7);
        let v2 = ease2.ease(0.7);
        
        assert_ne!(v1, v2);
    }

    #[test]
    fn test_bounce_easing() {
        let ease_in = EaseBounceIn;
        let ease_out = EaseBounceOut;
        let ease_in_out = EaseBounceInOut;
        
        assert!(ease_in.ease(0.0) < 0.1);
        assert!((ease_in.ease(1.0) - 1.0).abs() < 0.01);
        
        assert!(ease_out.ease(0.0) < 0.1);
        assert!((ease_out.ease(1.0) - 1.0).abs() < 0.01);
        
        assert!(ease_in_out.ease(0.0) < 0.1);
        assert!((ease_in_out.ease(1.0) - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_back_easing() {
        let ease_in = EaseBackIn;
        let ease_out = EaseBackOut;
        let ease_in_out = EaseBackInOut;
        
        assert_eq!(ease_in.ease(0.0), 0.0);
        assert!(ease_in.ease(1.0) > 0.99);
        
        assert!((ease_out.ease(1.0) - 1.0).abs() < 0.01);
        
        assert_eq!(ease_in_out.ease(0.0), 0.0);
        assert!((ease_in_out.ease(1.0) - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_easing_monotonic() {
        let ease = EaseIn::new(2.0);
        let mut prev = 0.0;
        
        for i in 0..=10 {
            let t = i as f32 / 10.0;
            let value = ease.ease(t);
            
            assert!(
                value >= prev - f32::EPSILON,
                "Easing should be monotonic: {} >= {}",
                value,
                prev
            );
            prev = value;
        }
    }

    #[test]
    fn test_all_easing_in_range() {
        let easings: Vec<Box<dyn EasingFunction>> = vec![
            Box::new(EaseIn::new(2.0)),
            Box::new(EaseOut::new(2.0)),
            Box::new(EaseInOut::new(2.0)),
            Box::new(EaseSineIn),
            Box::new(EaseSineOut),
            Box::new(EaseSineInOut),
            Box::new(EaseExponentialIn),
            Box::new(EaseExponentialOut),
            Box::new(EaseBounceOut),
            Box::new(EaseBackOut),
        ];
        
        for ease in easings {
            for i in 0..=10 {
                let t = i as f32 / 10.0;
                let value = ease.ease(t);
                
                // 大多数缓动函数值应该在合理范围内
                // 注意：某些函数（如 Back, Elastic）可能会超出 [0, 1]
                assert!(
                    value >= -0.5 && value <= 1.5,
                    "Easing value {} at t={} out of reasonable range",
                    value,
                    t
                );
            }
        }
    }
}

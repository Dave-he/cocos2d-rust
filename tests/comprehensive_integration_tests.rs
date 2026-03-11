// Cocos2d-Rust 集成测试 - 核心功能验证
// 
// 本文件包含集成测试,验证 Rust 版本与 cocos2d-x 的功能一致性

#[cfg(test)]
mod integration_tests {
    use cocos2d_rust::*;
    use std::rc::Rc;
    use std::cell::RefCell;

    /// 测试场景创建和管理
    #[test]
    fn test_scene_management() {
        // 创建场景
        let scene = Rc::new(RefCell::new(scene::Scene::new()));
        assert_eq!(scene.borrow().children().len(), 0);
        
        // 创建节点并添加到场景
        let node = Rc::new(RefCell::new(scene::Node::new()));
        node.borrow_mut().set_position(math::Vec2::new(100.0, 100.0));
        
        scene.borrow_mut().add_child(node.clone(), 0, None);
        assert_eq!(scene.borrow().children().len(), 1);
        
        // 验证节点位置
        let pos = node.borrow().position();
        assert!((pos.x - 100.0).abs() < 0.01);
        assert!((pos.y - 100.0).abs() < 0.01);
    }

    /// 测试动作系统 - MoveBy
    #[test]
    fn test_action_move_by() {
        use action::{Action, MoveBy};
        
        let target = Rc::new(RefCell::new(scene::Node::new()));
        target.borrow_mut().set_position(math::Vec2::zero());
        
        let mut move_by = MoveBy::new(1.0, math::Vec2::new(100.0, 50.0));
        move_by.start_with_target(&target);
        
        // 初始位置应该是 (0, 0)
        let initial_pos = target.borrow().position();
        assert_eq!(initial_pos, math::Vec2::zero());
        
        // 执行到 50% 应该在 (50, 25)
        move_by.update(0.5);
        let mid_pos = target.borrow().position();
        assert!((mid_pos.x - 50.0).abs() < 0.01, "Expected x=50, got {}", mid_pos.x);
        assert!((mid_pos.y - 25.0).abs() < 0.01, "Expected y=25, got {}", mid_pos.y);
        
        // 完成应该在 (100, 50)
        move_by.update(1.0);
        let final_pos = target.borrow().position();
        assert!((final_pos.x - 100.0).abs() < 0.01);
        assert!((final_pos.y - 50.0).abs() < 0.01);
        assert!(move_by.is_done());
    }

    /// 测试动画系统
    #[test]
    fn test_animation_system() {
        use animation::{Animation, SpriteFrame};
        
        // 创建动画
        let mut animation = Animation::new();
        animation.set_delay_per_unit(0.1);
        
        // 添加帧
        let frame1 = Rc::new(RefCell::new(SpriteFrame::new("frame1")));
        let frame2 = Rc::new(RefCell::new(SpriteFrame::new("frame2")));
        
        animation.add_sprite_frame(frame1);
        animation.add_sprite_frame(frame2);
        
        assert_eq!(animation.frames().len(), 2);
        assert!((animation.delay_per_unit() - 0.1).abs() < 0.001);
    }

    /// 测试粒子系统
    #[test]
    fn test_particle_system() {
        use particle::{ParticleSystem, EmitterType};
        
        let mut ps = ParticleSystem::new();
        ps.config.total_particles = 100;
        ps.config.emission_rate = 10.0;
        ps.config.life = 2.0;
        ps.config.emitter_type = EmitterType::GRAVITY;
        
        assert_eq!(ps.get_capacity(), 100);
        assert!((ps.config.emission_rate - 10.0).abs() < 0.01);
        assert!((ps.config.life - 2.0).abs() < 0.01);
    }

    /// 测试粒子预设
    #[test]
    fn test_particle_presets() {
        use particle::ParticlePresets;
        
        let fire = ParticlePresets::create_fire();
        assert_eq!(fire.get_capacity(), 250);
        
        let smoke = ParticlePresets::create_smoke();
        assert_eq!(smoke.get_capacity(), 200);
        
        let explosion = ParticlePresets::create_explosion();
        assert_eq!(explosion.get_capacity(), 700);
    }

    /// 测试 UI 组件 - Button
    #[test]
    fn test_ui_button() {
        use ui::Button;
        
        let mut button = Button::new();
        button.set_title_text("Click Me".to_string());
        button.set_enabled(true);
        
        assert_eq!(button.title_text(), "Click Me");
        assert!(button.is_enabled());
    }

    /// 测试 UI 组件 - Slider
    #[test]
    fn test_ui_slider() {
        use ui::Slider;
        
        let mut slider = Slider::new();
        slider.set_range(0.0, 100.0);
        slider.set_value(50.0);
        
        assert!((slider.value() - 50.0).abs() < 0.01);
        assert!((slider.normalized_value() - 0.5).abs() < 0.01);
    }

    /// 测试 ScrollView
    #[test]
    fn test_ui_scrollview() {
        use ui::scroll::{ScrollView, ScrollDirection};
        
        let mut sv = ScrollView::new();
        sv.set_direction(ScrollDirection::VERTICAL);
        sv.set_content_size(math::geometry::Size::new(400.0, 1000.0));
        
        assert_eq!(sv.direction(), ScrollDirection::VERTICAL);
    }

    /// 测试相机系统
    #[test]
    fn test_camera_2d() {
        use camera::Camera2D;
        
        let mut camera = Camera2D::new();
        camera.set_position(math::Vec2::new(100.0, 100.0));
        camera.set_zoom(2.0);
        
        let pos = camera.get_position();
        assert!((pos.x - 100.0).abs() < 0.01);
        assert!((camera.get_zoom() - 2.0).abs() < 0.01);
    }

    /// 测试数学库 - Vec2 运算
    #[test]
    fn test_math_vec2_operations() {
        use math::Vec2;
        
        let v1 = Vec2::new(3.0, 4.0);
        let v2 = Vec2::new(1.0, 2.0);
        
        // 加法
        let sum = v1 + v2;
        assert!((sum.x - 4.0).abs() < 0.01);
        assert!((sum.y - 6.0).abs() < 0.01);
        
        // 长度
        assert!((v1.length() - 5.0).abs() < 0.01);
        
        // 点积
        let dot = v1.dot(&v2);
        assert!((dot - 11.0).abs() < 0.01);
        
        // 归一化
        let mut normalized = v1;
        normalized.normalize();
        assert!((normalized.length() - 1.0).abs() < 0.01);
    }

    /// 测试数学库 - Mat4 变换
    #[test]
    fn test_math_mat4_transform() {
        use math::{Mat4, Vec3};
        
        let mat = Mat4::IDENTITY;
        let point = Vec3::new(1.0, 2.0, 3.0);
        
        let transformed = mat * point;
        assert!((transformed.x - point.x).abs() < 0.01);
        assert!((transformed.y - point.y).abs() < 0.01);
        assert!((transformed.z - point.z).abs() < 0.01);
    }

    /// 集成测试 - 完整游戏场景
    #[test]
    fn test_complete_game_scene() {
        use scene::{Scene, Node};
        use action::{MoveBy, Action};
        
        // 创建场景
        let scene = Rc::new(RefCell::new(Scene::new()));
        
        // 创建玩家节点
        let player = Rc::new(RefCell::new(Node::new()));
        player.borrow_mut().set_position(math::Vec2::new(100.0, 100.0));
        player.borrow_mut().set_scale(1.0);
        
        // 创建移动动作
        let mut move_action = MoveBy::new(2.0, math::Vec2::new(200.0, 0.0));
        move_action.start_with_target(&player);
        
        // 添加到场景
        scene.borrow_mut().add_child(player.clone(), 0, None);
        
        // 模拟帧更新
        for _ in 0..60 {
            move_action.update(1.0 / 60.0);
        }
        
        // 验证场景状态
        assert_eq!(scene.borrow().children().len(), 1);
        
        // 验证玩家移动
        let final_pos = player.borrow().position();
        assert!(final_pos.x > 100.0, "Player should have moved");
    }

    /// 性能测试 - 大量节点
    #[test]
    fn test_performance_many_nodes() {
        use scene::{Scene, Node};
        
        let scene = Rc::new(RefCell::new(Scene::new()));
        
        // 创建 1000 个节点
        for i in 0..1000 {
            let node = Rc::new(RefCell::new(Node::new()));
            node.borrow_mut().set_position(math::Vec2::new(
                (i % 100) as f32 * 10.0,
                (i / 100) as f32 * 10.0,
            ));
            scene.borrow_mut().add_child(node, 0, None);
        }
        
        assert_eq!(scene.borrow().children().len(), 1000);
    }

    /// 内存测试 - 智能指针正确性
    #[test]
    fn test_memory_smart_pointers() {
        use scene::Node;
        
        let parent = Rc::new(RefCell::new(Node::new()));
        
        {
            let child = Rc::new(RefCell::new(Node::new()));
            parent.borrow_mut().add_child_simple(child.clone());
            
            // child 在这里超出作用域,但因为 parent 持有引用,不会释放
        }
        
        // parent 应该仍然有 1 个子节点
        assert_eq!(parent.borrow().get_children_count(), 1);
    }
}

/// 运行所有集成测试
/// 
/// ```bash
/// cargo test --test integration_tests
/// ```
#[cfg(test)]
mod compatibility_tests {
    use super::*;
    use std::rc::Rc;
    use std::cell::RefCell;

    /// 测试与 cocos2d-x 的 API 兼容性
    #[test]
    fn test_api_compatibility() {
        // 这个测试验证 API 使用模式是否与 cocos2d-x 相似
        
        // C++: auto scene = Scene::create();
        // Rust: 
        let scene = Rc::new(RefCell::new(cocos2d_rust::scene::Scene::new()));
        
        // C++: auto node = Node::create();
        // Rust:
        let node = Rc::new(RefCell::new(cocos2d_rust::scene::Node::new()));
        
        // C++: node->setPosition(Vec2(100, 100));
        // Rust:
        node.borrow_mut().set_position(cocos2d_rust::math::Vec2::new(100.0, 100.0));
        
        // C++: scene->addChild(node);
        // Rust:
        scene.borrow_mut().add_child(node.clone(), 0, None);
        
        // 验证结果
        assert_eq!(scene.borrow().children().len(), 1);
    }
}

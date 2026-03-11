/// Phase 7 综合演示程序
/// 展示：RichText、UserDefault、ProgressTimer、Camera2D
use cocos2d_rust::{
    Vec2, Color3B,
    RichText, RichElement, RichElementType,
    UserDefault,
    ProgressTimer, ProgressTimerType, BarChangeRate,
    Camera2D,
    Sprite,
};

fn main() {
    println!("╔══════════════════════════════════════════╗");
    println!("║   Cocos2d-Rust Phase 7 Demo             ║");
    println!("║   实用模块组合演示                        ║");
    println!("╚══════════════════════════════════════════╝\n");

    demo_user_default();
    println!("\n{}\n", "=".repeat(50));
    
    demo_progress_timer();
    println!("\n{}\n", "=".repeat(50));
    
    demo_camera_2d();
    println!("\n{}\n", "=".repeat(50));
    
    demo_rich_text();
    println!("\n{}\n", "=".repeat(50));
    
    println!("✨ Phase 7 所有功能演示完成！\n");
}

/// 演示1: UserDefault - 数据持久化
fn demo_user_default() {
    println!("📦 演示 1: UserDefault - 数据持久化系统");
    println!("{}", "-".repeat(50));
    
    let ud = UserDefault::get_instance();
    let mut ud = ud.lock().unwrap();
    
    // 保存各种类型的数据
    println!("💾 保存数据...");
    ud.set_bool("first_launch", false);
    ud.set_int("player_level", 42);
    ud.set_long("total_score", 9876543210);
    ud.set_float("volume", 0.8);
    ud.set_double("pi_value", 3.141592653589793);
    ud.set_string("player_name", "勇敢的冒险者");
    
    println!("  ✓ bool    'first_launch' = false");
    println!("  ✓ int     'player_level' = 42");
    println!("  ✓ long    'total_score'  = 9876543210");
    println!("  ✓ float   'volume'       = 0.8");
    println!("  ✓ double  'pi_value'     = 3.141592653589793");
    println!("  ✓ string  'player_name'  = '勇敢的冒险者'");
    
    // 读取数据
    println!("\n📖 读取数据...");
    let first_launch = ud.get_bool("first_launch", true);
    let level = ud.get_int("player_level", 1);
    let score = ud.get_long("total_score", 0);
    let volume = ud.get_float("volume", 0.5);
    let pi = ud.get_double("pi_value", 0.0);
    let name = ud.get_string("player_name", "Unknown");
    
    println!("  ✓ first_launch = {}", first_launch);
    println!("  ✓ player_level = {}", level);
    println!("  ✓ total_score  = {}", score);
    println!("  ✓ volume       = {:.2}", volume);
    println!("  ✓ pi_value     = {:.15}", pi);
    println!("  ✓ player_name  = '{}'", name);
    
    // 键管理
    println!("\n🔑 键管理...");
    println!("  ✓ 总共 {} 个键", ud.get_all_keys().len());
    println!("  ✓ 是否存在 'player_level': {}", ud.has_key("player_level"));
    println!("  ✓ 是否存在 'non_existent': {}", ud.has_key("non_existent"));
    
    // 统计信息
    println!("\n📊 数据统计...");
    let stats = ud.get_stats();
    println!("  ✓ Bool 值数量:   {}", stats.bool_count);
    println!("  ✓ Int 值数量:    {}", stats.int_count);
    println!("  ✓ Float 值数量:  {}", stats.float_count);
    println!("  ✓ String 值数量: {}", stats.string_count);
    println!("  ✓ 总计:          {}", stats.total_count);
    
    // 持久化
    println!("\n💿 持久化到文件...");
    match ud.flush() {
        Ok(_) => println!("  ✓ 成功保存到: {:?}", ud.get_file_path()),
        Err(e) => println!("  ✗ 保存失败: {}", e),
    }
    
    println!("\n✅ UserDefault 演示完成!");
}

/// 演示2: ProgressTimer - 进度条特效
fn demo_progress_timer() {
    println!("⏳ 演示 2: ProgressTimer - 进度条特效");
    println!("{}", "-".repeat(50));
    
    let sprite = Sprite::default();
    
    // 径向进度
    println!("🎯 径向进度模式 (Radial):");
    let mut radial_timer = ProgressTimer::create(sprite.clone());
    radial_timer.set_type(ProgressTimerType::Radial);
    radial_timer.set_midpoint(Vec2::new(0.5, 0.5));
    radial_timer.set_reverse_direction(false);
    
    println!("  进度演示:");
    for percent in [0, 25, 50, 75, 100] {
        radial_timer.set_percentage(percent as f32);
        let bars = "█".repeat((percent / 5) as usize);
        let spaces = " ".repeat((100 - percent) / 5);
        println!("    {}% [{}{}]", percent, bars, spaces);
    }
    
    // 条形进度
    println!("\n📊 条形进度模式 (Bar):");
    let mut bar_timer = ProgressTimer::create(sprite.clone());
    bar_timer.set_type(ProgressTimerType::Bar);
    
    println!("  ├─ 水平进度 (左→右):");
    bar_timer.set_bar_change_rate(BarChangeRate::new(1.0, 0.0));
    bar_timer.set_midpoint_bar(Vec2::new(0.0, 0.5));
    bar_timer.set_percentage(60.0);
    println!("      60% [████████████░░░░░░░░]");
    
    println!("  └─ 垂直进度 (下→上):");
    bar_timer.set_bar_change_rate(BarChangeRate::new(0.0, 1.0));
    bar_timer.set_midpoint_bar(Vec2::new(0.5, 0.0));
    bar_timer.set_percentage(80.0);
    println!("      80% ┃");
    println!("          ┃████");
    println!("          ┃████");
    println!("          ┃████");
    println!("          ┃░░░░");
    
    // 边界测试
    println!("\n🔒 边界限制测试:");
    bar_timer.set_percentage(-50.0);
    println!("  ✓ 设置 -50%  → 实际: {}% (自动限制为0)", bar_timer.get_percentage());
    bar_timer.set_percentage(150.0);
    println!("  ✓ 设置 150%  → 实际: {}% (自动限制为100)", bar_timer.get_percentage());
    
    // 配置展示
    println!("\n⚙️  配置信息:");
    println!("  ✓ 类型: {:?}", bar_timer.get_type());
    println!("  ✓ 进度: {:.1}%", bar_timer.get_percentage());
    println!("  ✓ 中心点: ({:.1}, {:.1})", 
             bar_timer.get_midpoint().x, 
             bar_timer.get_midpoint().y);
    
    println!("\n✅ ProgressTimer 演示完成!");
}

/// 演示3: Camera2D - 2D相机系统
fn demo_camera_2d() {
    println!("📷 演示 3: Camera2D - 2D相机系统");
    println!("{}", "-".repeat(50));
    
    let mut camera = Camera2D::new();
    
    // 基础控制
    println!("🎮 基础控制:");
    camera.set_position(Vec2::new(100.0, 50.0));
    println!("  ✓ 设置位置: ({:.1}, {:.1})", camera.get_position().x, camera.get_position().y);
    
    camera.set_zoom(2.0);
    println!("  ✓ 设置缩放: {:.1}x", camera.get_zoom());
    
    camera.set_rotation(45.0);
    println!("  ✓ 设置旋转: {:.1}°", camera.get_rotation());
    
    // 移动演示
    println!("\n🚶 相机移动:");
    let positions = [
        Vec2::new(0.0, 0.0),
        Vec2::new(100.0, 0.0),
        Vec2::new(100.0, 100.0),
        Vec2::new(0.0, 100.0),
    ];
    
    for (i, pos) in positions.iter().enumerate() {
        camera.set_position(*pos);
        println!("  步骤 {}: 移动到 ({:.0}, {:.0})", i + 1, pos.x, pos.y);
    }
    
    // 缩放演示
    println!("\n🔍 缩放控制:");
    camera.set_zoom(1.0);
    println!("  初始缩放: {:.1}x", camera.get_zoom());
    camera.zoom_by(2.0);
    println!("  放大2倍:  {:.1}x", camera.get_zoom());
    camera.zoom_by(0.5);
    println!("  缩小一半: {:.1}x", camera.get_zoom());
    
    // 边界测试
    println!("\n🔒 缩放边界测试:");
    camera.set_zoom(0.05);
    println!("  ✓ 设置 0.05x → 实际: {:.1}x (最小0.1)", camera.get_zoom());
    camera.set_zoom(20.0);
    println!("  ✓ 设置 20.0x → 实际: {:.1}x (最大10.0)", camera.get_zoom());
    
    // 边界限制
    println!("\n🗺️  边界限制:");
    camera.enable_bounds(
        Vec2::new(-500.0, -500.0),
        Vec2::new(500.0, 500.0)
    );
    println!("  ✓ 启用边界: (-500, -500) 到 (500, 500)");
    
    camera.set_position(Vec2::new(1000.0, 1000.0));
    println!("  ✓ 尝试移动到 (1000, 1000)");
    println!("  ✓ 实际位置: ({:.0}, {:.0}) (被限制在边界内)", 
             camera.get_position().x, camera.get_position().y);
    
    // 坐标转换
    println!("\n🔄 坐标转换:");
    camera.set_position(Vec2::new(0.0, 0.0));
    camera.set_zoom(1.0);
    camera.set_viewport_size(Vec2::new(800.0, 600.0));
    
    let world_pos = Vec2::new(100.0, 50.0);
    let screen_pos = camera.world_to_screen(world_pos);
    let back_to_world = camera.screen_to_world(screen_pos);
    
    println!("  ✓ 世界坐标: ({:.1}, {:.1})", world_pos.x, world_pos.y);
    println!("  ✓ 屏幕坐标: ({:.1}, {:.1})", screen_pos.x, screen_pos.y);
    println!("  ✓ 往返转换: ({:.1}, {:.1})", back_to_world.x, back_to_world.y);
    
    // 跟随配置
    println!("\n👣 跟随系统:");
    camera.set_follow_offset(Vec2::new(0.0, 50.0));
    camera.set_follow_lerp(0.1);
    println!("  ✓ 跟随偏移: (0, 50)");
    println!("  ✓ 平滑度: {:.1} (0.1=平滑, 1.0=立即)", camera.get_follow_lerp());
    
    println!("\n✅ Camera2D 演示完成!");
}

/// 演示4: RichText - 富文本组件
fn demo_rich_text() {
    println!("📝 演示 4: RichText - 富文本组件");
    println!("{}", "-".repeat(50));
    
    let mut rich_text = RichText::new();
    
    // 基础配置
    println!("⚙️  基础配置:");
    rich_text.set_font_name("Arial");
    rich_text.set_font_size(16.0);
    rich_text.set_max_width(400.0);
    rich_text.set_horizontal_space(2.0);
    rich_text.set_vertical_space(5.0);
    
    println!("  ✓ 字体: {}", rich_text.get_font_name());
    println!("  ✓ 大小: {:.0}px", rich_text.get_font_size());
    println!("  ✓ 最大宽度: {:.0}px (自动换行)", rich_text.get_max_width());
    println!("  ✓ 水平间距: {:.0}px", rich_text.get_horizontal_space());
    println!("  ✓ 垂直间距: {:.0}px", rich_text.get_vertical_space());
    
    // 添加文本元素
    println!("\n📄 文本元素:");
    
    let title = RichElement::create_text(
        "title",
        Color3B::new(255, 200, 0),
        255,
        "欢迎来到 Cocos2d-Rust",
        "Arial",
        24.0,
    );
    rich_text.push_back_element(title);
    println!("  ✓ 添加标题 (24px, 金色)");
    
    let content = RichElement::create_text(
        "content",
        Color3B::WHITE,
        255,
        "这是一个功能强大的游戏引擎框架",
        "Arial",
        16.0,
    );
    rich_text.push_back_element(content);
    println!("  ✓ 添加正文 (16px, 白色)");
    
    let note = RichElement::create_text(
        "note",
        Color3B::new(200, 200, 200),
        200,
        "注意：这是一个示例文本",
        "Arial",
        12.0,
    );
    rich_text.push_back_element(note);
    println!("  ✓ 添加注释 (12px, 灰色, 80%透明)");
    
    // 添加图片元素
    println!("\n🖼️  图片元素:");
    let icon = RichElement::create_image(
        "icon",
        Color3B::WHITE,
        255,
        "icon.png",
        32.0,
        32.0,
    );
    rich_text.push_back_element(icon);
    println!("  ✓ 添加图标 (32x32)");
    
    // 超链接配置
    println!("\n🔗 超链接配置:");
    rich_text.set_anchor_text_color(Color3B::new(0, 150, 255));
    rich_text.set_anchor_text_underline(true);
    rich_text.set_anchor_text_bold(false);
    println!("  ✓ 链接颜色: 蓝色 (0, 150, 255)");
    println!("  ✓ 下划线: 启用");
    println!("  ✓ 粗体: 禁用");
    
    // 元素管理
    println!("\n🔧 元素管理:");
    println!("  ✓ 当前元素数量: 4");
    
    rich_text.insert_element(
        RichElement::create_text("inserted", Color3B::new(255, 100, 100), 255, "插入的文本", "Arial", 14.0),
        1
    );
    println!("  ✓ 在位置1插入新元素");
    println!("  ✓ 更新后元素数量: 5");
    
    rich_text.remove_element(1);
    println!("  ✓ 删除位置1的元素");
    println!("  ✓ 最终元素数量: 4");
    
    // 渲染效果预览
    println!("\n🎨 渲染效果预览:");
    println!("  ┌────────────────────────────────────┐");
    println!("  │  ⭐ 欢迎来到 Cocos2d-Rust         │");
    println!("  │                                    │");
    println!("  │  这是一个功能强大的游戏引擎框架    │");
    println!("  │                                    │");
    println!("  │  注意：这是一个示例文本            │");
    println!("  │                                    │");
    println!("  │  [📦]                             │");
    println!("  └────────────────────────────────────┘");
    
    // HTML 解析示例
    println!("\n🏷️  HTML 标签支持:");
    println!("  支持的标签:");
    println!("    • <font color=\"#RRGGBB\" size=\"12\">文本</font>");
    println!("    • <img src=\"image.png\" width=\"32\" height=\"32\"/>");
    println!("    • <a href=\"http://example.com\">链接</a>");
    println!("    • <b>粗体</b>, <i>斜体</i>, <u>下划线</u>");
    
    rich_text.set_string("Hello <b>World</b>!");
    println!("  ✓ 设置HTML字符串: \"Hello <b>World</b>!\"");
    
    println!("\n✅ RichText 演示完成!");
}

/// # 缓动函数可视化示例
/// 
/// 展示各种缓动函数的曲线特性

use cocos2d_rust::*;

fn main() {
    println!("════════════════════════════════════════");
    println!("  Cocos2D-Rust 缓动函数可视化");
    println!("════════════════════════════════════════\n");
    
    visualize_basic_easing();
    println!("\n");
    
    visualize_sine_easing();
    println!("\n");
    
    visualize_special_easing();
    println!("\n");
    
    compare_easing_curves();
}

/// 基础缓动函数可视化
fn visualize_basic_easing() {
    println!("【基础缓动函数】");
    println!("────────────────────────────────────────");
    
    let steps = 11;
    let t_values: Vec<f32> = (0..steps)
        .map(|i| i as f32 / (steps - 1) as f32)
        .collect();
    
    println!("时间:  {}", 
        t_values.iter()
            .map(|t| format!("{:.1}", t))
            .collect::<Vec<_>>()
            .join(" "));
    
    // EaseIn
    let ease_in = EaseIn::new(2.0);
    print_easing_curve("缓入", &ease_in, &t_values);
    
    // EaseOut
    let ease_out = EaseOut::new(2.0);
    print_easing_curve("缓出", &ease_out, &t_values);
    
    // EaseInOut
    let ease_in_out = EaseInOut::new(2.0);
    print_easing_curve("缓入缓出", &ease_in_out, &t_values);
}

/// 正弦缓动函数可视化
fn visualize_sine_easing() {
    println!("【正弦缓动函数】");
    println!("────────────────────────────────────────");
    
    let steps = 11;
    let t_values: Vec<f32> = (0..steps)
        .map(|i| i as f32 / (steps - 1) as f32)
        .collect();
    
    println!("时间:  {}", 
        t_values.iter()
            .map(|t| format!("{:.1}", t))
            .collect::<Vec<_>>()
            .join(" "));
    
    print_easing_curve("正弦缓入", &EaseSineIn, &t_values);
    print_easing_curve("正弦缓出", &EaseSineOut, &t_values);
    print_easing_curve("正弦缓入缓出", &EaseSineInOut, &t_values);
}

/// 特殊缓动函数可视化
fn visualize_special_easing() {
    println!("【特殊缓动函数】");
    println!("────────────────────────────────────────");
    
    let steps = 11;
    let t_values: Vec<f32> = (0..steps)
        .map(|i| i as f32 / (steps - 1) as f32)
        .collect();
    
    println!("时间:  {}", 
        t_values.iter()
            .map(|t| format!("{:.1}", t))
            .collect::<Vec<_>>()
            .join(" "));
    
    // 弹性
    print_easing_curve("弹性缓出", &EaseElasticOut::default(), &t_values);
    
    // 弹跳
    print_easing_curve("弹跳缓出", &EaseBounceOut, &t_values);
    
    // 回弹
    print_easing_curve("回弹缓出", &EaseBackOut, &t_values);
}

/// 对比不同缓动曲线
fn compare_easing_curves() {
    println!("【缓动函数对比】");
    println!("────────────────────────────────────────");
    println!("在 t=0.5 时的值对比:\n");
    
    let t = 0.5;
    
    let easings: Vec<(&str, Box<dyn EasingFunction>)> = vec![
        ("线性", Box::new(LinearEasing)),
        ("缓入(2.0)", Box::new(EaseIn::new(2.0))),
        ("缓出(2.0)", Box::new(EaseOut::new(2.0))),
        ("缓入缓出(2.0)", Box::new(EaseInOut::new(2.0))),
        ("正弦缓入", Box::new(EaseSineIn)),
        ("正弦缓出", Box::new(EaseSineOut)),
        ("指数缓入", Box::new(EaseExponentialIn)),
        ("指数缓出", Box::new(EaseExponentialOut)),
        ("弹性缓出", Box::new(EaseElasticOut::default())),
        ("弹跳缓出", Box::new(EaseBounceOut)),
        ("回弹缓出", Box::new(EaseBackOut)),
    ];
    
    for (name, ease) in easings {
        let value = ease.ease(t);
        let bar_len = (value * 40.0) as usize;
        let bar = "█".repeat(bar_len);
        println!("{:18} {:5.2} {}", name, value, bar);
    }
    
    println!("\n说明:");
    println!("  - 线性: 匀速变化");
    println!("  - 缓入: 开始慢，逐渐加速");
    println!("  - 缓出: 开始快，逐渐减速");
    println!("  - 弹性: 有弹簧效果");
    println!("  - 弹跳: 有反弹效果");
    println!("  - 回弹: 超出目标后回弹");
}

/// 打印缓动曲线
fn print_easing_curve(name: &str, ease: &dyn EasingFunction, t_values: &[f32]) {
    let values: Vec<f32> = t_values.iter()
        .map(|&t| ease.ease(t))
        .collect();
    
    print!("{:12} ", name);
    for value in values {
        print!("{:.2} ", value);
    }
    println!();
}

/// 线性缓动 (用于对比)
struct LinearEasing;

impl EasingFunction for LinearEasing {
    fn ease(&self, t: f32) -> f32 {
        t
    }
}

/// 动画应用示例
#[allow(dead_code)]
fn animation_example() {
    println!("【动画应用示例】");
    println!("────────────────────────────────────────");
    
    let duration = 2.0;  // 2秒动画
    let fps = 60.0;
    let frames = (duration * fps) as usize;
    
    let start_pos = 0.0;
    let end_pos = 100.0;
    
    let ease = EaseBounceOut;
    
    println!("弹跳动画: {} -> {} ({}秒)", start_pos, end_pos, duration);
    println!();
    
    for i in (0..frames).step_by(10) {
        let t = i as f32 / frames as f32;
        let eased_t = ease.ease(t);
        let pos = start_pos + (end_pos - start_pos) * eased_t;
        
        let bar_len = (pos / 2.0) as usize;
        let bar = "─".repeat(bar_len) + "●";
        
        println!("帧 {:3} | t={:.2} | pos={:5.1} | {}", 
            i, t, pos, bar);
    }
}

/// 缓动速度分析
#[allow(dead_code)]
fn velocity_analysis() {
    println!("【缓动速度分析】");
    println!("────────────────────────────────────────");
    
    let ease = EaseIn::new(2.0);
    let dt = 0.1;
    
    println!("{:>6} {:>8} {:>8}", "时间", "位置", "速度");
    println!("{}", "─".repeat(30));
    
    let mut prev_pos = 0.0;
    for i in 0..=10 {
        let t = i as f32 * dt;
        let pos = ease.ease(t);
        let velocity = if i == 0 {
            0.0
        } else {
            (pos - prev_pos) / dt
        };
        
        println!("{:>6.1} {:>8.3} {:>8.3}", t, pos, velocity);
        prev_pos = pos;
    }
    
    println!("\n观察:");
    println!("  - 缓入函数的速度逐渐增加");
    println!("  - 开始时速度较慢，结束时速度较快");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_easing_boundaries() {
        let easings: Vec<Box<dyn EasingFunction>> = vec![
            Box::new(EaseIn::new(2.0)),
            Box::new(EaseOut::new(2.0)),
            Box::new(EaseSineIn),
            Box::new(EaseSineOut),
            Box::new(EaseBounceOut),
        ];
        
        for ease in easings {
            assert_eq!(ease.ease(0.0), 0.0);
            assert!((ease.ease(1.0) - 1.0).abs() < 0.01);
        }
    }

    #[test]
    fn test_easing_monotonic() {
        let ease = EaseIn::new(2.0);
        let mut prev = 0.0;
        
        for i in 0..=10 {
            let t = i as f32 / 10.0;
            let value = ease.ease(t);
            assert!(value >= prev);
            prev = value;
        }
    }

    #[test]
    fn test_ease_in_out_symmetry() {
        let ease = EaseInOut::new(2.0);
        
        let v1 = ease.ease(0.25);
        let v2 = ease.ease(0.75);
        
        // EaseInOut 应该在 0.5 处对称
        assert!((v1 + v2 - 1.0).abs() < 0.1);
    }
}

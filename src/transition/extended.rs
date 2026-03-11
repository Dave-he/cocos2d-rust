/// 扩展过场动画效果
///
/// 提供 Split（分屏）、PageTurn（翻页）、Crossfade（交叉淡入淡出）、
/// Morph（变形）、Flash（闪烁）等常用过场效果。
///
/// 所有过场效果都通过 `f32` 的进度值（0.0 → 1.0）来驱动，
/// 便于与任意缓动函数组合。

use crate::math::Vec2;

// ─── 缓动函数 ─────────────────────────────────────────────────────

/// 线性
pub fn ease_linear(t: f32) -> f32 { t.clamp(0.0, 1.0) }
/// 缓入（二次）
pub fn ease_in_quad(t: f32) -> f32 { let t = t.clamp(0.0, 1.0); t * t }
/// 缓出（二次）
pub fn ease_out_quad(t: f32) -> f32 { let t = t.clamp(0.0, 1.0); t * (2.0 - t) }
/// 缓入缓出（二次）
pub fn ease_in_out_quad(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    if t < 0.5 { 2.0 * t * t } else { -1.0 + (4.0 - 2.0 * t) * t }
}
/// 弹性缓出
pub fn ease_out_elastic(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    if t == 0.0 || t == 1.0 { return t; }
    let p = 0.3_f32;
    2.0_f32.powf(-10.0 * t) * ((t - p / 4.0) * (2.0 * std::f32::consts::PI) / p).sin() + 1.0
}
/// 回弹缓出
pub fn ease_out_bounce(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    if t < 1.0 / 2.75 {
        7.5625 * t * t
    } else if t < 2.0 / 2.75 {
        let t = t - 1.5 / 2.75;
        7.5625 * t * t + 0.75
    } else if t < 2.5 / 2.75 {
        let t = t - 2.25 / 2.75;
        7.5625 * t * t + 0.9375
    } else {
        let t = t - 2.625 / 2.75;
        7.5625 * t * t + 0.984375
    }
}

// ─── TransitionProgress: 通用进度控制 ───────────────────────────

/// 通用过场进度追踪器
#[derive(Debug, Clone)]
pub struct TransitionProgress {
    duration: f32,
    elapsed: f32,
    finished: bool,
    looping: bool,
    reverse: bool,
}

impl TransitionProgress {
    pub fn new(duration: f32) -> Self {
        Self {
            duration: duration.max(0.001),
            elapsed: 0.0,
            finished: false,
            looping: false,
            reverse: false,
        }
    }

    /// 前进 delta_time 秒
    pub fn update(&mut self, dt: f32) {
        if self.finished && !self.looping { return; }
        self.elapsed += dt;
        if self.elapsed >= self.duration {
            if self.looping {
                self.elapsed = self.elapsed % self.duration;
            } else {
                self.elapsed = self.duration;
                self.finished = true;
            }
        }
    }

    /// 0.0 ~ 1.0 的归一化进度
    pub fn normalized(&self) -> f32 {
        let t = (self.elapsed / self.duration).clamp(0.0, 1.0);
        if self.reverse { 1.0 - t } else { t }
    }

    pub fn is_finished(&self) -> bool { self.finished }
    pub fn is_looping(&self) -> bool { self.looping }
    pub fn get_elapsed(&self) -> f32 { self.elapsed }
    pub fn get_duration(&self) -> f32 { self.duration }

    pub fn set_looping(&mut self, v: bool) { self.looping = v; }
    pub fn set_reverse(&mut self, v: bool) { self.reverse = v; }

    pub fn reset(&mut self) {
        self.elapsed = 0.0;
        self.finished = false;
    }

    /// 跳转到指定进度（0.0 ~ 1.0）
    pub fn seek(&mut self, t: f32) {
        self.elapsed = (t.clamp(0.0, 1.0) * self.duration).min(self.duration);
        self.finished = self.elapsed >= self.duration && !self.looping;
    }
}

// ─── SplitTransition ────────────────────────────────────────────

/// 分屏劈开过场方向
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitDirection {
    /// 上下各向外
    Vertical,
    /// 左右各向外
    Horizontal,
    /// 四角各向外
    Quad,
}

/// 分屏过场 — 旧场景从中间向外劈开，新场景从中间展开
#[derive(Debug, Clone)]
pub struct SplitTransition {
    pub progress: TransitionProgress,
    pub direction: SplitDirection,
}

impl SplitTransition {
    pub fn new(duration: f32, direction: SplitDirection) -> Self {
        Self {
            progress: TransitionProgress::new(duration),
            direction,
        }
    }

    /// 上半区域偏移比例 (0.0 原位 → 1.0 完全移出)
    pub fn top_offset(&self) -> f32 {
        self.progress.normalized()
    }

    /// 下半区域偏移比例
    pub fn bottom_offset(&self) -> f32 {
        -self.progress.normalized()
    }
}

// ─── PageTurnTransition ──────────────────────────────────────────

/// 翻页效果（3D 感）
#[derive(Debug, Clone)]
pub struct PageTurnTransition {
    pub progress: TransitionProgress,
    /// 翻页方向（true = 从右向左）
    pub right_to_left: bool,
}

impl PageTurnTransition {
    pub fn new(duration: f32, right_to_left: bool) -> Self {
        Self {
            progress: TransitionProgress::new(duration),
            right_to_left,
        }
    }

    /// 返回当前"折叠角度"（弧度，0.0 → π/2）
    pub fn fold_angle(&self) -> f32 {
        self.progress.normalized() * std::f32::consts::FRAC_PI_2
    }

    /// 折叠线的 x 位置（0.0 = 最右侧，1.0 = 最左侧）
    pub fn fold_x(&self) -> f32 {
        if self.right_to_left {
            1.0 - self.progress.normalized()
        } else {
            self.progress.normalized()
        }
    }
}

// ─── CrossfadeTransition ─────────────────────────────────────────

/// 交叉淡入淡出（旧场景渐出，新场景渐入）
#[derive(Debug, Clone)]
pub struct CrossfadeTransition {
    pub progress: TransitionProgress,
    pub use_white_flash: bool,
}

impl CrossfadeTransition {
    pub fn new(duration: f32) -> Self {
        Self {
            progress: TransitionProgress::new(duration),
            use_white_flash: false,
        }
    }

    pub fn with_white_flash(mut self) -> Self {
        self.use_white_flash = true;
        self
    }

    /// 旧场景的 Alpha（1.0 → 0.0）
    pub fn out_alpha(&self) -> f32 {
        1.0 - self.progress.normalized()
    }

    /// 新场景的 Alpha（0.0 → 1.0）
    pub fn in_alpha(&self) -> f32 {
        self.progress.normalized()
    }

    /// 白色遮罩 Alpha（0 → 1 → 0）
    pub fn white_alpha(&self) -> f32 {
        if !self.use_white_flash { return 0.0; }
        let t = self.progress.normalized();
        // 先升后降
        (1.0 - (t * 2.0 - 1.0).powi(2)).max(0.0)
    }
}

// ─── FlashTransition ─────────────────────────────────────────────

/// 闪烁（颜色全屏闪）过场
#[derive(Debug, Clone)]
pub struct FlashTransition {
    pub progress: TransitionProgress,
    pub flash_r: f32,
    pub flash_g: f32,
    pub flash_b: f32,
    /// 闪烁次数
    pub flash_count: u32,
}

impl FlashTransition {
    pub fn new_white(duration: f32, flash_count: u32) -> Self {
        Self {
            progress: TransitionProgress::new(duration),
            flash_r: 1.0, flash_g: 1.0, flash_b: 1.0,
            flash_count: flash_count.max(1),
        }
    }

    pub fn new_black(duration: f32, flash_count: u32) -> Self {
        Self {
            progress: TransitionProgress::new(duration),
            flash_r: 0.0, flash_g: 0.0, flash_b: 0.0,
            flash_count: flash_count.max(1),
        }
    }

    /// 当前闪烁亮度（0.0 ~ 1.0）
    pub fn flash_intensity(&self) -> f32 {
        let t = self.progress.normalized() * self.flash_count as f32;
        let local = t.fract();
        // sin 形成脉冲
        (local * std::f32::consts::PI).sin()
    }

    pub fn is_last_flash(&self) -> bool {
        let t = self.progress.normalized();
        t >= (self.flash_count - 1) as f32 / self.flash_count as f32
    }
}

// ─── MorphTransition ─────────────────────────────────────────────

/// 变形/扭曲过场（像素级位移偏移）
#[derive(Debug, Clone)]
pub struct MorphTransition {
    pub progress: TransitionProgress,
    /// 最大扭曲强度（像素）
    pub distortion_strength: f32,
}

impl MorphTransition {
    pub fn new(duration: f32, distortion_strength: f32) -> Self {
        Self {
            progress: TransitionProgress::new(duration),
            distortion_strength,
        }
    }

    /// 指定像素位置的扭曲偏移
    pub fn distortion_at(&self, uv: Vec2) -> Vec2 {
        let t = self.progress.normalized();
        // 半程扭曲最强，开始/结束无扭曲
        let intensity = (1.0 - (t * 2.0 - 1.0).abs()) * self.distortion_strength;
        Vec2::new(
            (uv.y * 8.0).sin() * intensity,
            (uv.x * 8.0).cos() * intensity,
        )
    }
}

// ─── TransitionBuilder ───────────────────────────────────────────

/// 过场效果类型（枚举式工厂）
#[derive(Debug, Clone)]
pub enum TransitionKind {
    Fade(f32),
    FadeWhite(f32),
    Slide { duration: f32, direction: SlideDir },
    Split { duration: f32, direction: SplitDirection },
    PageTurn { duration: f32, right_to_left: bool },
    Crossfade(f32),
    CrossfadeWhite(f32),
    Flash { duration: f32, color: FlashColor, count: u32 },
    Morph { duration: f32, strength: f32 },
    Zoom(f32),
    Rotate(f32),
    Flip(f32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlideDir { Left, Right, Up, Down }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlashColor { White, Black }

impl TransitionKind {
    /// 过场持续时间
    pub fn duration(&self) -> f32 {
        match self {
            TransitionKind::Fade(d) => *d,
            TransitionKind::FadeWhite(d) => *d,
            TransitionKind::Slide { duration, .. } => *duration,
            TransitionKind::Split { duration, .. } => *duration,
            TransitionKind::PageTurn { duration, .. } => *duration,
            TransitionKind::Crossfade(d) => *d,
            TransitionKind::CrossfadeWhite(d) => *d,
            TransitionKind::Flash { duration, .. } => *duration,
            TransitionKind::Morph { duration, .. } => *duration,
            TransitionKind::Zoom(d) => *d,
            TransitionKind::Rotate(d) => *d,
            TransitionKind::Flip(d) => *d,
        }
    }

    /// 是否是渐变类
    pub fn is_fade_type(&self) -> bool {
        matches!(self, TransitionKind::Fade(_) | TransitionKind::FadeWhite(_)
            | TransitionKind::Crossfade(_) | TransitionKind::CrossfadeWhite(_))
    }

    /// 是否有 3D 效果
    pub fn is_3d(&self) -> bool {
        matches!(self, TransitionKind::PageTurn { .. }
            | TransitionKind::Flip(_)
            | TransitionKind::Zoom(_))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── TransitionProgress ──────────────────────────────────────

    #[test]
    fn test_transition_progress_basic() {
        let mut p = TransitionProgress::new(1.0);
        assert_eq!(p.normalized(), 0.0);
        assert!(!p.is_finished());

        p.update(0.5);
        assert!((p.normalized() - 0.5).abs() < 0.001);

        p.update(0.5);
        assert!((p.normalized() - 1.0).abs() < 0.001);
        assert!(p.is_finished());
    }

    #[test]
    fn test_transition_progress_no_overshoot() {
        let mut p = TransitionProgress::new(1.0);
        p.update(100.0);
        assert_eq!(p.normalized(), 1.0);
        assert!(p.is_finished());
    }

    #[test]
    fn test_transition_progress_looping() {
        let mut p = TransitionProgress::new(1.0);
        p.set_looping(true);
        p.update(1.5);
        assert!(!p.is_finished());
        assert!((p.normalized() - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_transition_progress_reverse() {
        let mut p = TransitionProgress::new(1.0);
        p.set_reverse(true);
        assert_eq!(p.normalized(), 1.0); // 初始倒放 = 1.0

        p.update(0.5);
        assert!((p.normalized() - 0.5).abs() < 0.001);

        p.update(0.5);
        assert_eq!(p.normalized(), 0.0);
    }

    #[test]
    fn test_transition_progress_seek() {
        let mut p = TransitionProgress::new(2.0);
        p.seek(0.75);
        assert!((p.normalized() - 0.75).abs() < 0.001);
        assert!((p.get_elapsed() - 1.5).abs() < 0.001);
    }

    #[test]
    fn test_transition_progress_reset() {
        let mut p = TransitionProgress::new(1.0);
        p.update(1.0);
        assert!(p.is_finished());

        p.reset();
        assert!(!p.is_finished());
        assert_eq!(p.normalized(), 0.0);
    }

    // ── 缓动函数 ─────────────────────────────────────────────────

    #[test]
    fn test_easing_functions_boundaries() {
        for func in [ease_linear, ease_in_quad, ease_out_quad, ease_in_out_quad,
                     ease_out_elastic, ease_out_bounce] as [fn(f32) -> f32; 6] {
            assert!((func(0.0) - 0.0).abs() < 0.001, "start should be ~0");
            assert!((func(1.0) - 1.0).abs() < 0.001, "end should be ~1");
        }
    }

    #[test]
    fn test_ease_in_quad_monotone() {
        let mut prev = ease_in_quad(0.0);
        for i in 1..=10 {
            let cur = ease_in_quad(i as f32 / 10.0);
            assert!(cur >= prev);
            prev = cur;
        }
    }

    // ── SplitTransition ─────────────────────────────────────────

    #[test]
    fn test_split_transition_basic() {
        let mut t = SplitTransition::new(1.0, SplitDirection::Vertical);
        assert_eq!(t.top_offset(), 0.0);
        t.progress.update(0.5);
        assert!((t.top_offset() - 0.5).abs() < 0.001);
        assert!((t.bottom_offset() + 0.5).abs() < 0.001);
    }

    #[test]
    fn test_split_transition_horizontal() {
        let mut t = SplitTransition::new(2.0, SplitDirection::Horizontal);
        t.progress.update(2.0);
        assert!(t.progress.is_finished());
        assert_eq!(t.top_offset(), 1.0);
    }

    // ── PageTurnTransition ──────────────────────────────────────

    #[test]
    fn test_page_turn_basic() {
        let mut t = PageTurnTransition::new(1.0, true);
        assert_eq!(t.fold_angle(), 0.0);
        t.progress.update(1.0);
        assert!((t.fold_angle() - std::f32::consts::FRAC_PI_2).abs() < 0.001);
    }

    #[test]
    fn test_page_turn_fold_x() {
        let mut t = PageTurnTransition::new(1.0, true);
        assert_eq!(t.fold_x(), 1.0);
        t.progress.update(1.0);
        assert_eq!(t.fold_x(), 0.0);
    }

    // ── CrossfadeTransition ─────────────────────────────────────

    #[test]
    fn test_crossfade_alphas() {
        let mut t = CrossfadeTransition::new(1.0);
        assert_eq!(t.out_alpha(), 1.0);
        assert_eq!(t.in_alpha(), 0.0);

        t.progress.update(0.5);
        assert!((t.out_alpha() - 0.5).abs() < 0.001);
        assert!((t.in_alpha() - 0.5).abs() < 0.001);

        t.progress.update(0.5);
        assert_eq!(t.out_alpha(), 0.0);
        assert_eq!(t.in_alpha(), 1.0);
    }

    #[test]
    fn test_crossfade_white_flash() {
        let mut t = CrossfadeTransition::new(1.0).with_white_flash();
        assert_eq!(t.white_alpha(), 0.0); // 开始无闪

        t.progress.update(0.5);
        assert!(t.white_alpha() > 0.0); // 中间有闪光

        t.progress.update(0.5);
        assert!(t.white_alpha() < 0.1); // 结束时近乎无闪
    }

    // ── FlashTransition ─────────────────────────────────────────

    #[test]
    fn test_flash_transition_intensity() {
        let mut t = FlashTransition::new_white(1.0, 3);
        t.progress.update(0.0);
        // 开始时 intensity 约 0
        let i0 = t.flash_intensity();
        assert!(i0 >= 0.0);

        t.progress.update(0.166); // 约 1/(3*2) 进入第一次脉冲高峰前
        let i1 = t.flash_intensity();
        assert!(i1 >= 0.0 && i1 <= 1.0);
    }

    #[test]
    fn test_flash_is_last() {
        let mut t = FlashTransition::new_black(1.0, 2);
        assert!(!t.is_last_flash());
        t.progress.update(0.9);
        assert!(t.is_last_flash());
    }

    // ── MorphTransition ─────────────────────────────────────────

    #[test]
    fn test_morph_distortion_range() {
        let mut t = MorphTransition::new(1.0, 20.0);
        t.progress.update(0.5); // 中间扭曲最强

        let d = t.distortion_at(Vec2::new(0.5, 0.5));
        assert!(d.x.abs() <= 20.0 + 0.001);
        assert!(d.y.abs() <= 20.0 + 0.001);
    }

    #[test]
    fn test_morph_distortion_zero_at_ends() {
        let t_start = MorphTransition::new(1.0, 10.0);
        let d0 = t_start.distortion_at(Vec2::new(0.5, 0.5));
        assert!(d0.x.abs() < 0.001);
        assert!(d0.y.abs() < 0.001);

        let mut t_end = MorphTransition::new(1.0, 10.0);
        t_end.progress.update(1.0);
        let d1 = t_end.distortion_at(Vec2::new(0.5, 0.5));
        assert!(d1.x.abs() < 0.001);
        assert!(d1.y.abs() < 0.001);
    }

    // ── TransitionKind ───────────────────────────────────────────

    #[test]
    fn test_transition_kind_duration() {
        assert_eq!(TransitionKind::Fade(0.5).duration(), 0.5);
        assert_eq!(TransitionKind::Slide { duration: 1.2, direction: SlideDir::Left }.duration(), 1.2);
    }

    #[test]
    fn test_transition_kind_is_fade() {
        assert!(TransitionKind::Fade(1.0).is_fade_type());
        assert!(TransitionKind::Crossfade(1.0).is_fade_type());
        assert!(!TransitionKind::Slide { duration: 1.0, direction: SlideDir::Right }.is_fade_type());
    }

    #[test]
    fn test_transition_kind_is_3d() {
        assert!(TransitionKind::Flip(1.0).is_3d());
        assert!(TransitionKind::PageTurn { duration: 1.0, right_to_left: true }.is_3d());
        assert!(!TransitionKind::Fade(1.0).is_3d());
    }
}

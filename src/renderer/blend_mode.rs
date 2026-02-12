#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendFactor {
    Zero,
    One,
    SrcColor,
    OneMinusSrcColor,
    DstColor,
    OneMinusDstColor,
    SrcAlpha,
    OneMinusSrcAlpha,
    DstAlpha,
    OneMinusDstAlpha,
    ConstantColor,
    OneMinusConstantColor,
    ConstantAlpha,
    OneMinusConstantAlpha,
    SrcAlphaSaturate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendEquation {
    Add,
    Subtract,
    ReverseSubtract,
    Min,
    Max,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AdvancedBlendMode {
    pub src_rgb: BlendFactor,
    pub dst_rgb: BlendFactor,
    pub src_alpha: BlendFactor,
    pub dst_alpha: BlendFactor,
    pub equation_rgb: BlendEquation,
    pub equation_alpha: BlendEquation,
    pub enabled: bool,
}

impl AdvancedBlendMode {
    pub fn new(
        src_rgb: BlendFactor,
        dst_rgb: BlendFactor,
        equation: BlendEquation,
    ) -> Self {
        Self {
            src_rgb,
            dst_rgb,
            src_alpha: src_rgb,
            dst_alpha: dst_rgb,
            equation_rgb: equation,
            equation_alpha: equation,
            enabled: true,
        }
    }

    pub fn with_separate_alpha(
        src_rgb: BlendFactor,
        dst_rgb: BlendFactor,
        src_alpha: BlendFactor,
        dst_alpha: BlendFactor,
        equation_rgb: BlendEquation,
        equation_alpha: BlendEquation,
    ) -> Self {
        Self {
            src_rgb,
            dst_rgb,
            src_alpha,
            dst_alpha,
            equation_rgb,
            equation_alpha,
            enabled: true,
        }
    }

    pub const DISABLED: Self = Self {
        src_rgb: BlendFactor::One,
        dst_rgb: BlendFactor::Zero,
        src_alpha: BlendFactor::One,
        dst_alpha: BlendFactor::Zero,
        equation_rgb: BlendEquation::Add,
        equation_alpha: BlendEquation::Add,
        enabled: false,
    };

    pub const NORMAL: Self = Self {
        src_rgb: BlendFactor::SrcAlpha,
        dst_rgb: BlendFactor::OneMinusSrcAlpha,
        src_alpha: BlendFactor::One,
        dst_alpha: BlendFactor::OneMinusSrcAlpha,
        equation_rgb: BlendEquation::Add,
        equation_alpha: BlendEquation::Add,
        enabled: true,
    };

    pub const ADDITIVE: Self = Self {
        src_rgb: BlendFactor::SrcAlpha,
        dst_rgb: BlendFactor::One,
        src_alpha: BlendFactor::SrcAlpha,
        dst_alpha: BlendFactor::One,
        equation_rgb: BlendEquation::Add,
        equation_alpha: BlendEquation::Add,
        enabled: true,
    };

    pub const MULTIPLY: Self = Self {
        src_rgb: BlendFactor::DstColor,
        dst_rgb: BlendFactor::OneMinusSrcAlpha,
        src_alpha: BlendFactor::DstAlpha,
        dst_alpha: BlendFactor::OneMinusSrcAlpha,
        equation_rgb: BlendEquation::Add,
        equation_alpha: BlendEquation::Add,
        enabled: true,
    };

    pub const SCREEN: Self = Self {
        src_rgb: BlendFactor::One,
        dst_rgb: BlendFactor::OneMinusSrcColor,
        src_alpha: BlendFactor::One,
        dst_alpha: BlendFactor::OneMinusSrcAlpha,
        equation_rgb: BlendEquation::Add,
        equation_alpha: BlendEquation::Add,
        enabled: true,
    };

    pub const PREMULTIPLIED_ALPHA: Self = Self {
        src_rgb: BlendFactor::One,
        dst_rgb: BlendFactor::OneMinusSrcAlpha,
        src_alpha: BlendFactor::One,
        dst_alpha: BlendFactor::OneMinusSrcAlpha,
        equation_rgb: BlendEquation::Add,
        equation_alpha: BlendEquation::Add,
        enabled: true,
    };

    pub const SUBTRACT: Self = Self {
        src_rgb: BlendFactor::SrcAlpha,
        dst_rgb: BlendFactor::One,
        src_alpha: BlendFactor::SrcAlpha,
        dst_alpha: BlendFactor::One,
        equation_rgb: BlendEquation::ReverseSubtract,
        equation_alpha: BlendEquation::ReverseSubtract,
        enabled: true,
    };

    pub const MIN: Self = Self {
        src_rgb: BlendFactor::One,
        dst_rgb: BlendFactor::One,
        src_alpha: BlendFactor::One,
        dst_alpha: BlendFactor::One,
        equation_rgb: BlendEquation::Min,
        equation_alpha: BlendEquation::Min,
        enabled: true,
    };

    pub const MAX: Self = Self {
        src_rgb: BlendFactor::One,
        dst_rgb: BlendFactor::One,
        src_alpha: BlendFactor::One,
        dst_alpha: BlendFactor::One,
        equation_rgb: BlendEquation::Max,
        equation_alpha: BlendEquation::Max,
        enabled: true,
    };

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn has_separate_alpha(&self) -> bool {
        self.src_rgb != self.src_alpha
            || self.dst_rgb != self.dst_alpha
            || self.equation_rgb != self.equation_alpha
    }

    pub fn to_gl_values(&self) -> (u32, u32, u32, u32) {
        (
            blend_factor_to_gl(self.src_rgb),
            blend_factor_to_gl(self.dst_rgb),
            blend_factor_to_gl(self.src_alpha),
            blend_factor_to_gl(self.dst_alpha),
        )
    }

    pub fn to_gl_equation(&self) -> (u32, u32) {
        (
            blend_equation_to_gl(self.equation_rgb),
            blend_equation_to_gl(self.equation_alpha),
        )
    }
}

impl Default for AdvancedBlendMode {
    fn default() -> Self {
        Self::NORMAL
    }
}

fn blend_factor_to_gl(factor: BlendFactor) -> u32 {
    match factor {
        BlendFactor::Zero => 0,
        BlendFactor::One => 1,
        BlendFactor::SrcColor => 0x0300,
        BlendFactor::OneMinusSrcColor => 0x0301,
        BlendFactor::DstColor => 0x0306,
        BlendFactor::OneMinusDstColor => 0x0307,
        BlendFactor::SrcAlpha => 0x0302,
        BlendFactor::OneMinusSrcAlpha => 0x0303,
        BlendFactor::DstAlpha => 0x0304,
        BlendFactor::OneMinusDstAlpha => 0x0305,
        BlendFactor::ConstantColor => 0x8001,
        BlendFactor::OneMinusConstantColor => 0x8002,
        BlendFactor::ConstantAlpha => 0x8003,
        BlendFactor::OneMinusConstantAlpha => 0x8004,
        BlendFactor::SrcAlphaSaturate => 0x0308,
    }
}

fn blend_equation_to_gl(equation: BlendEquation) -> u32 {
    match equation {
        BlendEquation::Add => 0x8006,
        BlendEquation::Subtract => 0x800A,
        BlendEquation::ReverseSubtract => 0x800B,
        BlendEquation::Min => 0x8007,
        BlendEquation::Max => 0x8008,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blend_factor_equality() {
        assert_eq!(BlendFactor::Zero, BlendFactor::Zero);
        assert_eq!(BlendFactor::One, BlendFactor::One);
        assert_ne!(BlendFactor::Zero, BlendFactor::One);
    }

    #[test]
    fn test_blend_equation_equality() {
        assert_eq!(BlendEquation::Add, BlendEquation::Add);
        assert_eq!(BlendEquation::Subtract, BlendEquation::Subtract);
        assert_ne!(BlendEquation::Add, BlendEquation::Subtract);
    }

    #[test]
    fn test_blend_mode_new() {
        let mode = AdvancedBlendMode::new(
            BlendFactor::SrcAlpha,
            BlendFactor::OneMinusSrcAlpha,
            BlendEquation::Add,
        );
        
        assert_eq!(mode.src_rgb, BlendFactor::SrcAlpha);
        assert_eq!(mode.dst_rgb, BlendFactor::OneMinusSrcAlpha);
        assert_eq!(mode.src_alpha, BlendFactor::SrcAlpha);
        assert_eq!(mode.dst_alpha, BlendFactor::OneMinusSrcAlpha);
        assert!(mode.enabled);
    }

    #[test]
    fn test_blend_mode_with_separate_alpha() {
        let mode = AdvancedBlendMode::with_separate_alpha(
            BlendFactor::SrcAlpha,
            BlendFactor::OneMinusSrcAlpha,
            BlendFactor::One,
            BlendFactor::Zero,
            BlendEquation::Add,
            BlendEquation::Add,
        );
        
        assert_eq!(mode.src_rgb, BlendFactor::SrcAlpha);
        assert_eq!(mode.src_alpha, BlendFactor::One);
        assert_eq!(mode.dst_alpha, BlendFactor::Zero);
    }

    #[test]
    fn test_blend_mode_disabled() {
        let mode = AdvancedBlendMode::DISABLED;
        assert!(!mode.is_enabled());
    }

    #[test]
    fn test_blend_mode_normal() {
        let mode = AdvancedBlendMode::NORMAL;
        assert_eq!(mode.src_rgb, BlendFactor::SrcAlpha);
        assert_eq!(mode.dst_rgb, BlendFactor::OneMinusSrcAlpha);
        assert!(mode.is_enabled());
    }

    #[test]
    fn test_blend_mode_additive() {
        let mode = AdvancedBlendMode::ADDITIVE;
        assert_eq!(mode.src_rgb, BlendFactor::SrcAlpha);
        assert_eq!(mode.dst_rgb, BlendFactor::One);
    }

    #[test]
    fn test_blend_mode_multiply() {
        let mode = AdvancedBlendMode::MULTIPLY;
        assert_eq!(mode.src_rgb, BlendFactor::DstColor);
        assert_eq!(mode.dst_rgb, BlendFactor::OneMinusSrcAlpha);
    }

    #[test]
    fn test_blend_mode_screen() {
        let mode = AdvancedBlendMode::SCREEN;
        assert_eq!(mode.src_rgb, BlendFactor::One);
        assert_eq!(mode.dst_rgb, BlendFactor::OneMinusSrcColor);
    }

    #[test]
    fn test_blend_mode_premultiplied_alpha() {
        let mode = AdvancedBlendMode::PREMULTIPLIED_ALPHA;
        assert_eq!(mode.src_rgb, BlendFactor::One);
        assert_eq!(mode.dst_rgb, BlendFactor::OneMinusSrcAlpha);
    }

    #[test]
    fn test_blend_mode_subtract() {
        let mode = AdvancedBlendMode::SUBTRACT;
        assert_eq!(mode.equation_rgb, BlendEquation::ReverseSubtract);
    }

    #[test]
    fn test_blend_mode_min_max() {
        let min_mode = AdvancedBlendMode::MIN;
        assert_eq!(min_mode.equation_rgb, BlendEquation::Min);
        
        let max_mode = AdvancedBlendMode::MAX;
        assert_eq!(max_mode.equation_rgb, BlendEquation::Max);
    }

    #[test]
    fn test_set_enabled() {
        let mut mode = AdvancedBlendMode::NORMAL;
        assert!(mode.is_enabled());
        
        mode.set_enabled(false);
        assert!(!mode.is_enabled());
    }

    #[test]
    fn test_has_separate_alpha() {
        let mode1 = AdvancedBlendMode::NORMAL;
        assert!(mode1.has_separate_alpha());
        
        let mode2 = AdvancedBlendMode::new(
            BlendFactor::One,
            BlendFactor::Zero,
            BlendEquation::Add,
        );
        assert!(!mode2.has_separate_alpha());
    }

    #[test]
    fn test_to_gl_values() {
        let mode = AdvancedBlendMode::NORMAL;
        let (src_rgb, dst_rgb, src_alpha, dst_alpha) = mode.to_gl_values();
        
        assert_eq!(src_rgb, 0x0302);
        assert_eq!(dst_rgb, 0x0303);
    }

    #[test]
    fn test_to_gl_equation() {
        let mode = AdvancedBlendMode::NORMAL;
        let (eq_rgb, eq_alpha) = mode.to_gl_equation();
        
        assert_eq!(eq_rgb, 0x8006);
        assert_eq!(eq_alpha, 0x8006);
    }

    #[test]
    fn test_blend_mode_default() {
        let mode = AdvancedBlendMode::default();
        assert_eq!(mode.src_rgb, AdvancedBlendMode::NORMAL.src_rgb);
        assert!(mode.is_enabled());
    }

    #[test]
    fn test_blend_factor_to_gl() {
        assert_eq!(blend_factor_to_gl(BlendFactor::Zero), 0);
        assert_eq!(blend_factor_to_gl(BlendFactor::One), 1);
        assert_eq!(blend_factor_to_gl(BlendFactor::SrcAlpha), 0x0302);
        assert_eq!(blend_factor_to_gl(BlendFactor::OneMinusSrcAlpha), 0x0303);
    }

    #[test]
    fn test_blend_equation_to_gl() {
        assert_eq!(blend_equation_to_gl(BlendEquation::Add), 0x8006);
        assert_eq!(blend_equation_to_gl(BlendEquation::Subtract), 0x800A);
        assert_eq!(blend_equation_to_gl(BlendEquation::ReverseSubtract), 0x800B);
        assert_eq!(blend_equation_to_gl(BlendEquation::Min), 0x8007);
        assert_eq!(blend_equation_to_gl(BlendEquation::Max), 0x8008);
    }

    #[test]
    fn test_all_blend_factors() {
        let factors = [
            BlendFactor::Zero,
            BlendFactor::One,
            BlendFactor::SrcColor,
            BlendFactor::OneMinusSrcColor,
            BlendFactor::DstColor,
            BlendFactor::OneMinusDstColor,
            BlendFactor::SrcAlpha,
            BlendFactor::OneMinusSrcAlpha,
            BlendFactor::DstAlpha,
            BlendFactor::OneMinusDstAlpha,
            BlendFactor::ConstantColor,
            BlendFactor::OneMinusConstantColor,
            BlendFactor::ConstantAlpha,
            BlendFactor::OneMinusConstantAlpha,
            BlendFactor::SrcAlphaSaturate,
        ];
        
        for factor in &factors {
            let gl_value = blend_factor_to_gl(*factor);
            assert!(gl_value >= 0);
        }
    }

    #[test]
    fn test_all_blend_equations() {
        let equations = [
            BlendEquation::Add,
            BlendEquation::Subtract,
            BlendEquation::ReverseSubtract,
            BlendEquation::Min,
            BlendEquation::Max,
        ];
        
        for equation in &equations {
            let gl_value = blend_equation_to_gl(*equation);
            assert!(gl_value > 0);
        }
    }

    #[test]
    fn test_blend_mode_equality() {
        let mode1 = AdvancedBlendMode::NORMAL;
        let mode2 = AdvancedBlendMode::NORMAL;
        assert_eq!(mode1, mode2);
        
        let mode3 = AdvancedBlendMode::ADDITIVE;
        assert_ne!(mode1, mode3);
    }

    #[test]
    fn test_all_preset_modes() {
        let modes = [
            AdvancedBlendMode::DISABLED,
            AdvancedBlendMode::NORMAL,
            AdvancedBlendMode::ADDITIVE,
            AdvancedBlendMode::MULTIPLY,
            AdvancedBlendMode::SCREEN,
            AdvancedBlendMode::PREMULTIPLIED_ALPHA,
            AdvancedBlendMode::SUBTRACT,
            AdvancedBlendMode::MIN,
            AdvancedBlendMode::MAX,
        ];
        
        for mode in &modes {
            let _ = mode.to_gl_values();
            let _ = mode.to_gl_equation();
        }
    }
}


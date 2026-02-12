use std::f32::consts::PI;

pub trait EasingFunction {
    fn ease(&self, t: f32) -> f32;
}

pub struct EaseIn {
    rate: f32,
}

impl EaseIn {
    pub fn new(rate: f32) -> Self {
        Self { rate }
    }
}

impl EasingFunction for EaseIn {
    fn ease(&self, t: f32) -> f32 {
        t.powf(self.rate)
    }
}

pub struct EaseOut {
    rate: f32,
}

impl EaseOut {
    pub fn new(rate: f32) -> Self {
        Self { rate }
    }
}

impl EasingFunction for EaseOut {
    fn ease(&self, t: f32) -> f32 {
        t.powf(1.0 / self.rate)
    }
}

pub struct EaseInOut {
    rate: f32,
}

impl EaseInOut {
    pub fn new(rate: f32) -> Self {
        Self { rate }
    }
}

impl EasingFunction for EaseInOut {
    fn ease(&self, t: f32) -> f32 {
        let t = t * 2.0;
        if t < 1.0 {
            0.5 * t.powf(self.rate)
        } else {
            1.0 - 0.5 * (2.0 - t).abs().powf(self.rate)
        }
    }
}

pub struct EaseSineIn;

impl EasingFunction for EaseSineIn {
    fn ease(&self, t: f32) -> f32 {
        1.0 - ((t * PI) / 2.0).cos()
    }
}

pub struct EaseSineOut;

impl EasingFunction for EaseSineOut {
    fn ease(&self, t: f32) -> f32 {
        ((t * PI) / 2.0).sin()
    }
}

pub struct EaseSineInOut;

impl EasingFunction for EaseSineInOut {
    fn ease(&self, t: f32) -> f32 {
        -0.5 * ((PI * t).cos() - 1.0)
    }
}

pub struct EaseExponentialIn;

impl EasingFunction for EaseExponentialIn {
    fn ease(&self, t: f32) -> f32 {
        if t == 0.0 {
            0.0
        } else {
            2.0_f32.powf(10.0 * (t - 1.0))
        }
    }
}

pub struct EaseExponentialOut;

impl EasingFunction for EaseExponentialOut {
    fn ease(&self, t: f32) -> f32 {
        if t == 1.0 {
            1.0
        } else {
            1.0 - 2.0_f32.powf(-10.0 * t)
        }
    }
}

pub struct EaseExponentialInOut;

impl EasingFunction for EaseExponentialInOut {
    fn ease(&self, t: f32) -> f32 {
        if t == 0.0 {
            return 0.0;
        }
        if t == 1.0 {
            return 1.0;
        }
        
        let t = t * 2.0;
        if t < 1.0 {
            0.5 * 2.0_f32.powf(10.0 * (t - 1.0))
        } else {
            0.5 * (2.0 - 2.0_f32.powf(-10.0 * (t - 1.0)))
        }
    }
}

pub struct EaseElasticIn {
    period: f32,
}

impl EaseElasticIn {
    pub fn new(period: f32) -> Self {
        Self { period }
    }
}

impl Default for EaseElasticIn {
    fn default() -> Self {
        Self::new(0.3)
    }
}

impl EasingFunction for EaseElasticIn {
    fn ease(&self, t: f32) -> f32 {
        if t == 0.0 || t == 1.0 {
            return t;
        }
        
        let t = t - 1.0;
        let s = self.period / 4.0;
        -(2.0_f32.powf(10.0 * t)) * ((t - s) * (2.0 * PI) / self.period).sin()
    }
}

pub struct EaseElasticOut {
    period: f32,
}

impl EaseElasticOut {
    pub fn new(period: f32) -> Self {
        Self { period }
    }
}

impl Default for EaseElasticOut {
    fn default() -> Self {
        Self::new(0.3)
    }
}

impl EasingFunction for EaseElasticOut {
    fn ease(&self, t: f32) -> f32 {
        if t == 0.0 || t == 1.0 {
            return t;
        }
        
        let s = self.period / 4.0;
        2.0_f32.powf(-10.0 * t) * ((t - s) * (2.0 * PI) / self.period).sin() + 1.0
    }
}

pub struct EaseElasticInOut {
    period: f32,
}

impl EaseElasticInOut {
    pub fn new(period: f32) -> Self {
        Self { period }
    }
}

impl Default for EaseElasticInOut {
    fn default() -> Self {
        Self::new(0.3)
    }
}

impl EasingFunction for EaseElasticInOut {
    fn ease(&self, t: f32) -> f32 {
        if t == 0.0 || t == 1.0 {
            return t;
        }
        
        let mut t = t * 2.0;
        let s = self.period / 4.0;
        
        t -= 1.0;
        if t < 0.0 {
            -0.5 * (2.0_f32.powf(10.0 * t)) * ((t - s) * (2.0 * PI) / self.period).sin()
        } else {
            0.5 * 2.0_f32.powf(-10.0 * t) * ((t - s) * (2.0 * PI) / self.period).sin() + 1.0
        }
    }
}

pub struct EaseBounceIn;

impl EasingFunction for EaseBounceIn {
    fn ease(&self, t: f32) -> f32 {
        1.0 - EaseBounceOut.ease(1.0 - t)
    }
}

pub struct EaseBounceOut;

impl EasingFunction for EaseBounceOut {
    fn ease(&self, t: f32) -> f32 {
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
}

pub struct EaseBounceInOut;

impl EasingFunction for EaseBounceInOut {
    fn ease(&self, t: f32) -> f32 {
        if t < 0.5 {
            0.5 * EaseBounceIn.ease(t * 2.0)
        } else {
            0.5 * EaseBounceOut.ease(t * 2.0 - 1.0) + 0.5
        }
    }
}

pub struct EaseBackIn;

impl EasingFunction for EaseBackIn {
    fn ease(&self, t: f32) -> f32 {
        let overshoot = 1.70158;
        t * t * ((overshoot + 1.0) * t - overshoot)
    }
}

pub struct EaseBackOut;

impl EasingFunction for EaseBackOut {
    fn ease(&self, t: f32) -> f32 {
        let overshoot = 1.70158;
        let t = t - 1.0;
        t * t * ((overshoot + 1.0) * t + overshoot) + 1.0
    }
}

pub struct EaseBackInOut;

impl EasingFunction for EaseBackInOut {
    fn ease(&self, t: f32) -> f32 {
        let overshoot = 1.70158 * 1.525;
        let t = t * 2.0;
        
        if t < 1.0 {
            0.5 * (t * t * ((overshoot + 1.0) * t - overshoot))
        } else {
            let t = t - 2.0;
            0.5 * (t * t * ((overshoot + 1.0) * t + overshoot) + 2.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ease_in() {
        let ease = EaseIn::new(2.0);
        assert_eq!(ease.ease(0.0), 0.0);
        assert_eq!(ease.ease(1.0), 1.0);
        assert!(ease.ease(0.5) < 0.5);
    }

    #[test]
    fn test_ease_out() {
        let ease = EaseOut::new(2.0);
        assert_eq!(ease.ease(0.0), 0.0);
        assert_eq!(ease.ease(1.0), 1.0);
        assert!(ease.ease(0.5) > 0.5);
    }

    #[test]
    fn test_ease_in_out() {
        let ease = EaseInOut::new(2.0);
        assert_eq!(ease.ease(0.0), 0.0);
        assert_eq!(ease.ease(1.0), 1.0);
    }

    #[test]
    fn test_ease_sine_in() {
        let ease = EaseSineIn;
        assert_eq!(ease.ease(0.0), 0.0);
        assert!((ease.ease(1.0) - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_ease_sine_out() {
        let ease = EaseSineOut;
        assert_eq!(ease.ease(0.0), 0.0);
        assert!((ease.ease(1.0) - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_ease_exponential_in() {
        let ease = EaseExponentialIn;
        assert_eq!(ease.ease(0.0), 0.0);
        assert!((ease.ease(1.0) - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_ease_exponential_out() {
        let ease = EaseExponentialOut;
        assert_eq!(ease.ease(1.0), 1.0);
    }

    #[test]
    fn test_ease_elastic_in() {
        let ease = EaseElasticIn::default();
        assert_eq!(ease.ease(0.0), 0.0);
        assert_eq!(ease.ease(1.0), 1.0);
    }

    #[test]
    fn test_ease_elastic_out() {
        let ease = EaseElasticOut::default();
        assert_eq!(ease.ease(0.0), 0.0);
        assert_eq!(ease.ease(1.0), 1.0);
    }

    #[test]
    fn test_ease_bounce_out() {
        let ease = EaseBounceOut;
        assert!(ease.ease(0.0) < 0.1);
        assert!((ease.ease(1.0) - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_ease_bounce_in() {
        let ease = EaseBounceIn;
        assert!(ease.ease(0.0) < 0.1);
        assert!((ease.ease(1.0) - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_ease_back_in() {
        let ease = EaseBackIn;
        assert_eq!(ease.ease(0.0), 0.0);
        assert!(ease.ease(1.0) > 0.99);
    }

    #[test]
    fn test_ease_back_out() {
        let ease = EaseBackOut;
        assert!((ease.ease(1.0) - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_boundary_values() {
        let easing_functions: Vec<Box<dyn EasingFunction>> = vec![
            Box::new(EaseIn::new(2.0)),
            Box::new(EaseOut::new(2.0)),
            Box::new(EaseSineIn),
            Box::new(EaseSineOut),
            Box::new(EaseBounceOut),
        ];

        for ease in easing_functions {
            let result = ease.ease(0.5);
            assert!(result >= 0.0 && result <= 1.5);
        }
    }
}

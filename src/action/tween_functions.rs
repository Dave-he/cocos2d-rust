use std::f32::consts::PI;

const M_PI_2: f32 = PI / 2.0;

pub fn linear(time: f32) -> f32 {
    time
}

pub fn sine_ease_in(time: f32) -> f32 {
    -1.0 * (time * M_PI_2).cos() + 1.0
}

pub fn sine_ease_out(time: f32) -> f32 {
    (time * M_PI_2).sin()
}

pub fn sine_ease_in_out(time: f32) -> f32 {
    -0.5 * ((PI * time).cos() - 1.0)
}

pub fn quad_ease_in(time: f32) -> f32 {
    time * time
}

pub fn quad_ease_out(time: f32) -> f32 {
    -time * (time - 2.0)
}

pub fn quad_ease_in_out(time: f32) -> f32 {
    let mut time = time * 2.0;
    if time < 1.0 {
        time * time / 2.0
    } else {
        time -= 1.0;
        -0.5 * (time * (time - 2.0) - 1.0)
    }
}

pub fn cubic_ease_in(time: f32) -> f32 {
    time * time * time
}

pub fn cubic_ease_out(time: f32) -> f32 {
    let time = time - 1.0;
    time * time * time + 1.0
}

pub fn cubic_ease_in_out(time: f32) -> f32 {
    let mut time = time * 2.0;
    if time < 1.0 {
        0.5 * time * time * time
    } else {
        time -= 2.0;
        0.5 * (time * time * time + 2.0)
    }
}

pub fn quart_ease_in(time: f32) -> f32 {
    time * time * time * time
}

pub fn quart_ease_out(time: f32) -> f32 {
    let time = time - 1.0;
    -(time * time * time * time - 1.0)
}

pub fn quart_ease_in_out(time: f32) -> f32 {
    let mut time = time * 2.0;
    if time < 1.0 {
        0.5 * time * time * time * time
    } else {
        time -= 2.0;
        -0.5 * (time * time * time * time - 2.0)
    }
}

pub fn quint_ease_in(time: f32) -> f32 {
    time * time * time * time * time
}

pub fn quint_ease_out(time: f32) -> f32 {
    let time = time - 1.0;
    time * time * time * time * time + 1.0
}

pub fn quint_ease_in_out(time: f32) -> f32 {
    let mut time = time * 2.0;
    if time < 1.0 {
        0.5 * time * time * time * time * time
    } else {
        time -= 2.0;
        0.5 * (time * time * time * time * time + 2.0)
    }
}

pub fn expo_ease_in(time: f32) -> f32 {
    if time == 0.0 {
        0.0
    } else {
        2.0_f32.powf(10.0 * (time - 1.0)) - 1.0 * 0.001
    }
}

pub fn expo_ease_out(time: f32) -> f32 {
    if time == 1.0 {
        1.0
    } else {
        1.0 - 2.0_f32.powf(-10.0 * time)
    }
}

pub fn expo_ease_in_out(time: f32) -> f32 {
    let mut time = time / 0.5;
    if time < 1.0 {
        0.5 * 2.0_f32.powf(10.0 * (time - 1.0))
    } else {
        time -= 1.0;
        0.5 * (2.0 - 2.0_f32.powf(-10.0 * time))
    }
}

pub fn circ_ease_in(time: f32) -> f32 {
    -(1.0 - time * time).sqrt() + 1.0
}

pub fn circ_ease_out(time: f32) -> f32 {
    let time = time - 1.0;
    (1.0 - time * time).sqrt()
}

pub fn circ_ease_in_out(time: f32) -> f32 {
    let mut time = time * 2.0;
    if time < 1.0 {
        -0.5 * ((1.0 - time * time).sqrt() - 1.0)
    } else {
        time -= 2.0;
        0.5 * ((1.0 - time * time).sqrt() + 1.0)
    }
}

pub fn back_ease_in(time: f32) -> f32 {
    let overshoot = 1.70158;
    time * time * ((overshoot + 1.0) * time - overshoot)
}

pub fn back_ease_out(time: f32) -> f32 {
    let overshoot = 1.70158;
    let time = time - 1.0;
    time * time * ((overshoot + 1.0) * time + overshoot) + 1.0
}

pub fn back_ease_in_out(time: f32) -> f32 {
    let overshoot = 1.70158 * 1.525;
    let mut time = time * 2.0;
    if time < 1.0 {
        0.5 * (time * time * ((overshoot + 1.0) * time - overshoot))
    } else {
        time -= 2.0;
        0.5 * (time * time * ((overshoot + 1.0) * time + overshoot) + 2.0)
    }
}

pub fn bounce_ease_in(time: f32) -> f32 {
    1.0 - bounce_ease_out(1.0 - time)
}

pub fn bounce_ease_out(mut time: f32) -> f32 {
    if time < 1.0 / 2.75 {
        7.5625 * time * time
    } else if time < 2.0 / 2.75 {
        time -= 1.5 / 2.75;
        7.5625 * time * time + 0.75
    } else if time < 2.5 / 2.75 {
        time -= 2.25 / 2.75;
        7.5625 * time * time + 0.9375
    } else {
        time -= 2.625 / 2.75;
        7.5625 * time * time + 0.984375
    }
}

pub fn bounce_ease_in_out(time: f32) -> f32 {
    if time < 0.5 {
        bounce_ease_in(time * 2.0) * 0.5
    } else {
        bounce_ease_out(time * 2.0 - 1.0) * 0.5 + 0.5
    }
}

pub fn ease_in(time: f32, rate: f32) -> f32 {
    time.powf(rate)
}

pub fn ease_out(time: f32, rate: f32) -> f32 {
    (1.0 - time).powf(rate)
}

pub fn ease_in_out(time: f32, rate: f32) -> f32 {
    let mut time = time * 2.0;
    if time < 1.0 {
        0.5 * time.powf(rate)
    } else {
        time = 2.0 - time;
        1.0 - 0.5 * time.powf(rate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear() {
        assert_eq!(linear(0.0), 0.0);
        assert_eq!(linear(0.5), 0.5);
        assert_eq!(linear(1.0), 1.0);
    }

    #[test]
    fn test_sine_ease() {
        assert!((sine_ease_in(0.0) - 0.0).abs() < 0.001);
        assert!((sine_ease_in(1.0) - 1.0).abs() < 0.001);
        
        assert!((sine_ease_out(0.0) - 0.0).abs() < 0.001);
        assert!((sine_ease_out(1.0) - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_bounce() {
        assert!((bounce_ease_out(0.0) - 0.0).abs() < 0.001);
        assert!((bounce_ease_out(1.0) - 1.0).abs() < 0.001);
    }
}

use crate::renderer::shader::{Shader, ShaderProgram, ShaderType};
use crate::renderer::RenderTexture;
use crate::math::Vec2;
use std::rc::Rc;

pub trait PostProcessEffect {
    fn apply(&mut self, input: &RenderTexture, output: &RenderTexture);
    fn name(&self) -> &str;
    fn enabled(&self) -> bool;
    fn set_enabled(&mut self, enabled: bool);
}

pub struct BlurEffect {
    name: String,
    enabled: bool,
    radius: f32,
    iterations: u32,
    shader_program: Option<Rc<ShaderProgram>>,
}

impl BlurEffect {
    pub fn new(radius: f32) -> Self {
        Self {
            name: "Blur".to_string(),
            enabled: true,
            radius,
            iterations: 2,
            shader_program: None,
        }
    }

    pub fn set_radius(&mut self, radius: f32) {
        self.radius = radius;
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn set_iterations(&mut self, iterations: u32) {
        self.iterations = iterations;
    }

    pub fn iterations(&self) -> u32 {
        self.iterations
    }

    fn create_shader_program() -> ShaderProgram {
        let vs_source = r#"
#version 330 core
layout(location = 0) in vec3 a_position;
layout(location = 1) in vec2 a_texCoord;

out vec2 v_texCoord;

void main() {
    gl_Position = vec4(a_position, 1.0);
    v_texCoord = a_texCoord;
}
"#;

        let fs_source = r#"
#version 330 core
in vec2 v_texCoord;
out vec4 FragColor;

uniform sampler2D u_texture;
uniform float u_radius;
uniform vec2 u_resolution;

void main() {
    vec2 texelSize = 1.0 / u_resolution;
    vec4 result = vec4(0.0);
    float total = 0.0;
    
    for (float x = -u_radius; x <= u_radius; x += 1.0) {
        for (float y = -u_radius; y <= u_radius; y += 1.0) {
            vec2 offset = vec2(x, y) * texelSize;
            float weight = 1.0;
            result += texture(u_texture, v_texCoord + offset) * weight;
            total += weight;
        }
    }
    
    FragColor = result / total;
}
"#;

        let vs = Rc::new(Shader::new(ShaderType::Vertex, vs_source.to_string()));
        let fs = Rc::new(Shader::new(ShaderType::Fragment, fs_source.to_string()));
        
        ShaderProgram::new(vs, fs)
    }
}

impl PostProcessEffect for BlurEffect {
    fn apply(&mut self, _input: &RenderTexture, _output: &RenderTexture) {
        if !self.enabled {
            return;
        }

        if self.shader_program.is_none() {
            self.shader_program = Some(Rc::new(Self::create_shader_program()));
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn enabled(&self) -> bool {
        self.enabled
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

pub struct BloomEffect {
    name: String,
    enabled: bool,
    threshold: f32,
    intensity: f32,
    blur_passes: u32,
    shader_program: Option<Rc<ShaderProgram>>,
}

impl BloomEffect {
    pub fn new() -> Self {
        Self {
            name: "Bloom".to_string(),
            enabled: true,
            threshold: 0.8,
            intensity: 1.0,
            blur_passes: 3,
            shader_program: None,
        }
    }

    pub fn set_threshold(&mut self, threshold: f32) {
        self.threshold = threshold.clamp(0.0, 1.0);
    }

    pub fn threshold(&self) -> f32 {
        self.threshold
    }

    pub fn set_intensity(&mut self, intensity: f32) {
        self.intensity = intensity.max(0.0);
    }

    pub fn intensity(&self) -> f32 {
        self.intensity
    }

    pub fn set_blur_passes(&mut self, passes: u32) {
        self.blur_passes = passes;
    }

    pub fn blur_passes(&self) -> u32 {
        self.blur_passes
    }
}

impl PostProcessEffect for BloomEffect {
    fn apply(&mut self, _input: &RenderTexture, _output: &RenderTexture) {
        if !self.enabled {
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn enabled(&self) -> bool {
        self.enabled
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

impl Default for BloomEffect {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ColorGradingEffect {
    name: String,
    enabled: bool,
    brightness: f32,
    contrast: f32,
    saturation: f32,
    hue: f32,
    shader_program: Option<Rc<ShaderProgram>>,
}

impl ColorGradingEffect {
    pub fn new() -> Self {
        Self {
            name: "ColorGrading".to_string(),
            enabled: true,
            brightness: 0.0,
            contrast: 1.0,
            saturation: 1.0,
            hue: 0.0,
            shader_program: None,
        }
    }

    pub fn set_brightness(&mut self, brightness: f32) {
        self.brightness = brightness.clamp(-1.0, 1.0);
    }

    pub fn brightness(&self) -> f32 {
        self.brightness
    }

    pub fn set_contrast(&mut self, contrast: f32) {
        self.contrast = contrast.max(0.0);
    }

    pub fn contrast(&self) -> f32 {
        self.contrast
    }

    pub fn set_saturation(&mut self, saturation: f32) {
        self.saturation = saturation.max(0.0);
    }

    pub fn saturation(&self) -> f32 {
        self.saturation
    }

    pub fn set_hue(&mut self, hue: f32) {
        self.hue = hue;
    }

    pub fn hue(&self) -> f32 {
        self.hue
    }
}

impl PostProcessEffect for ColorGradingEffect {
    fn apply(&mut self, _input: &RenderTexture, _output: &RenderTexture) {
        if !self.enabled {
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn enabled(&self) -> bool {
        self.enabled
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

impl Default for ColorGradingEffect {
    fn default() -> Self {
        Self::new()
    }
}

pub struct VignetteEffect {
    name: String,
    enabled: bool,
    intensity: f32,
    smoothness: f32,
    center: Vec2,
    shader_program: Option<Rc<ShaderProgram>>,
}

impl VignetteEffect {
    pub fn new() -> Self {
        Self {
            name: "Vignette".to_string(),
            enabled: true,
            intensity: 0.5,
            smoothness: 0.5,
            center: Vec2::new(0.5, 0.5),
            shader_program: None,
        }
    }

    pub fn set_intensity(&mut self, intensity: f32) {
        self.intensity = intensity.clamp(0.0, 1.0);
    }

    pub fn intensity(&self) -> f32 {
        self.intensity
    }

    pub fn set_smoothness(&mut self, smoothness: f32) {
        self.smoothness = smoothness.clamp(0.0, 1.0);
    }

    pub fn smoothness(&self) -> f32 {
        self.smoothness
    }

    pub fn set_center(&mut self, center: Vec2) {
        self.center = center;
    }

    pub fn center(&self) -> Vec2 {
        self.center
    }
}

impl PostProcessEffect for VignetteEffect {
    fn apply(&mut self, _input: &RenderTexture, _output: &RenderTexture) {
        if !self.enabled {
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn enabled(&self) -> bool {
        self.enabled
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

impl Default for VignetteEffect {
    fn default() -> Self {
        Self::new()
    }
}

pub struct PostProcessStack {
    effects: Vec<Box<dyn PostProcessEffect>>,
    enabled: bool,
}

impl PostProcessStack {
    pub fn new() -> Self {
        Self {
            effects: Vec::new(),
            enabled: true,
        }
    }

    pub fn add_effect(&mut self, effect: Box<dyn PostProcessEffect>) {
        self.effects.push(effect);
    }

    pub fn remove_effect(&mut self, name: &str) -> Option<Box<dyn PostProcessEffect>> {
        if let Some(index) = self.effects.iter().position(|e| e.name() == name) {
            Some(self.effects.remove(index))
        } else {
            None
        }
    }

    pub fn get_effect_mut(&mut self, name: &str) -> Option<&mut Box<dyn PostProcessEffect>> {
        self.effects.iter_mut().find(|e| e.name() == name)
    }

    pub fn apply_all(&mut self, input: &RenderTexture, output: &RenderTexture) {
        if !self.enabled {
            return;
        }

        for effect in &mut self.effects {
            if effect.enabled() {
                effect.apply(input, output);
            }
        }
    }

    pub fn clear(&mut self) {
        self.effects.clear();
    }

    pub fn effect_count(&self) -> usize {
        self.effects.len()
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

impl Default for PostProcessStack {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blur_effect_creation() {
        let effect = BlurEffect::new(5.0);
        assert_eq!(effect.name(), "Blur");
        assert!(effect.enabled());
        assert_eq!(effect.radius(), 5.0);
    }

    #[test]
    fn test_blur_effect_parameters() {
        let mut effect = BlurEffect::new(5.0);
        effect.set_radius(10.0);
        assert_eq!(effect.radius(), 10.0);
        
        effect.set_iterations(5);
        assert_eq!(effect.iterations(), 5);
    }

    #[test]
    fn test_bloom_effect() {
        let mut effect = BloomEffect::new();
        assert_eq!(effect.name(), "Bloom");
        
        effect.set_threshold(0.5);
        assert_eq!(effect.threshold(), 0.5);
        
        effect.set_intensity(2.0);
        assert_eq!(effect.intensity(), 2.0);
    }

    #[test]
    fn test_color_grading_effect() {
        let mut effect = ColorGradingEffect::new();
        assert_eq!(effect.name(), "ColorGrading");
        
        effect.set_brightness(0.2);
        assert_eq!(effect.brightness(), 0.2);
        
        effect.set_contrast(1.5);
        assert_eq!(effect.contrast(), 1.5);
    }

    #[test]
    fn test_vignette_effect() {
        let mut effect = VignetteEffect::new();
        assert_eq!(effect.name(), "Vignette");
        
        effect.set_intensity(0.8);
        assert_eq!(effect.intensity(), 0.8);
        
        let center = Vec2::new(0.3, 0.7);
        effect.set_center(center);
        assert_eq!(effect.center(), center);
    }

    #[test]
    fn test_post_process_stack() {
        let mut stack = PostProcessStack::new();
        assert_eq!(stack.effect_count(), 0);
        
        stack.add_effect(Box::new(BlurEffect::new(5.0)));
        assert_eq!(stack.effect_count(), 1);
        
        stack.add_effect(Box::new(BloomEffect::new()));
        assert_eq!(stack.effect_count(), 2);
        
        stack.remove_effect("Blur");
        assert_eq!(stack.effect_count(), 1);
    }

    #[test]
    fn test_effect_enable_disable() {
        let mut effect = BlurEffect::new(5.0);
        assert!(effect.enabled());
        
        effect.set_enabled(false);
        assert!(!effect.enabled());
    }

    #[test]
    fn test_blur_effect_shader_creation() {
        let mut effect = BlurEffect::new(3.0);
        let texture = RenderTexture::new(100, 100);
        effect.apply(&texture, &texture);
    }

    #[test]
    fn test_bloom_threshold_clamping() {
        let mut effect = BloomEffect::new();
        
        effect.set_threshold(-0.5);
        assert_eq!(effect.threshold(), 0.0);
        
        effect.set_threshold(1.5);
        assert_eq!(effect.threshold(), 1.0);
    }

    #[test]
    fn test_bloom_intensity_validation() {
        let mut effect = BloomEffect::new();
        
        effect.set_intensity(-1.0);
        assert_eq!(effect.intensity(), 0.0);
        
        effect.set_intensity(5.0);
        assert_eq!(effect.intensity(), 5.0);
    }

    #[test]
    fn test_bloom_blur_passes() {
        let mut effect = BloomEffect::new();
        
        effect.set_blur_passes(5);
        assert_eq!(effect.blur_passes(), 5);
    }

    #[test]
    fn test_color_grading_brightness_range() {
        let mut effect = ColorGradingEffect::new();
        
        effect.set_brightness(-1.5);
        assert_eq!(effect.brightness(), -1.0);
        
        effect.set_brightness(1.5);
        assert_eq!(effect.brightness(), 1.0);
    }

    #[test]
    fn test_color_grading_contrast_validation() {
        let mut effect = ColorGradingEffect::new();
        
        effect.set_contrast(-1.0);
        assert_eq!(effect.contrast(), 0.0);
        
        effect.set_contrast(2.5);
        assert_eq!(effect.contrast(), 2.5);
    }

    #[test]
    fn test_color_grading_saturation() {
        let mut effect = ColorGradingEffect::new();
        
        effect.set_saturation(0.0);
        assert_eq!(effect.saturation(), 0.0);
        
        effect.set_saturation(2.0);
        assert_eq!(effect.saturation(), 2.0);
    }

    #[test]
    fn test_color_grading_hue_rotation() {
        let mut effect = ColorGradingEffect::new();
        
        effect.set_hue(180.0);
        assert_eq!(effect.hue(), 180.0);
        
        effect.set_hue(-90.0);
        assert_eq!(effect.hue(), -90.0);
    }

    #[test]
    fn test_vignette_intensity_range() {
        let mut effect = VignetteEffect::new();
        
        effect.set_intensity(-0.5);
        assert_eq!(effect.intensity(), 0.0);
        
        effect.set_intensity(1.5);
        assert_eq!(effect.intensity(), 1.0);
    }

    #[test]
    fn test_vignette_smoothness_range() {
        let mut effect = VignetteEffect::new();
        
        effect.set_smoothness(-0.5);
        assert_eq!(effect.smoothness(), 0.0);
        
        effect.set_smoothness(1.5);
        assert_eq!(effect.smoothness(), 1.0);
    }

    #[test]
    fn test_vignette_center_positioning() {
        let mut effect = VignetteEffect::new();
        
        let top_left = Vec2::new(0.0, 1.0);
        effect.set_center(top_left);
        assert_eq!(effect.center(), top_left);
        
        let bottom_right = Vec2::new(1.0, 0.0);
        effect.set_center(bottom_right);
        assert_eq!(effect.center(), bottom_right);
    }

    #[test]
    fn test_post_process_stack_clear() {
        let mut stack = PostProcessStack::new();
        
        stack.add_effect(Box::new(BlurEffect::new(5.0)));
        stack.add_effect(Box::new(BloomEffect::new()));
        assert_eq!(stack.effect_count(), 2);
        
        stack.clear();
        assert_eq!(stack.effect_count(), 0);
    }

    #[test]
    fn test_post_process_stack_get_effect() {
        let mut stack = PostProcessStack::new();
        
        stack.add_effect(Box::new(BlurEffect::new(5.0)));
        
        let effect = stack.get_effect_mut("Blur");
        assert!(effect.is_some());
        
        let effect = stack.get_effect_mut("NonExistent");
        assert!(effect.is_none());
    }

    #[test]
    fn test_post_process_stack_enabled() {
        let mut stack = PostProcessStack::new();
        assert!(stack.is_enabled());
        
        stack.set_enabled(false);
        assert!(!stack.is_enabled());
    }

    #[test]
    fn test_multiple_effects_in_stack() {
        let mut stack = PostProcessStack::new();
        
        stack.add_effect(Box::new(BlurEffect::new(3.0)));
        stack.add_effect(Box::new(BloomEffect::new()));
        stack.add_effect(Box::new(ColorGradingEffect::new()));
        stack.add_effect(Box::new(VignetteEffect::new()));
        
        assert_eq!(stack.effect_count(), 4);
    }

    #[test]
    fn test_effect_apply_when_disabled() {
        let mut effect = BlurEffect::new(5.0);
        effect.set_enabled(false);
        
        let texture = RenderTexture::new(100, 100);
        effect.apply(&texture, &texture);
    }

    #[test]
    fn test_bloom_default_values() {
        let effect = BloomEffect::default();
        
        assert_eq!(effect.threshold(), 0.8);
        assert_eq!(effect.intensity(), 1.0);
        assert_eq!(effect.blur_passes(), 3);
        assert!(effect.enabled());
    }

    #[test]
    fn test_color_grading_default_values() {
        let effect = ColorGradingEffect::default();
        
        assert_eq!(effect.brightness(), 0.0);
        assert_eq!(effect.contrast(), 1.0);
        assert_eq!(effect.saturation(), 1.0);
        assert_eq!(effect.hue(), 0.0);
    }

    #[test]
    fn test_vignette_default_values() {
        let effect = VignetteEffect::default();
        
        assert_eq!(effect.intensity(), 0.5);
        assert_eq!(effect.smoothness(), 0.5);
        assert_eq!(effect.center(), Vec2::new(0.5, 0.5));
    }

    #[test]
    fn test_stack_apply_all_when_disabled() {
        let mut stack = PostProcessStack::new();
        stack.add_effect(Box::new(BlurEffect::new(5.0)));
        stack.set_enabled(false);
        
        let texture = RenderTexture::new(100, 100);
        stack.apply_all(&texture, &texture);
    }

    #[test]
    fn test_stack_remove_nonexistent_effect() {
        let mut stack = PostProcessStack::new();
        stack.add_effect(Box::new(BlurEffect::new(5.0)));
        
        let removed = stack.remove_effect("NonExistent");
        assert!(removed.is_none());
        assert_eq!(stack.effect_count(), 1);
    }

    #[test]
    fn test_blur_iterations_modification() {
        let mut effect = BlurEffect::new(5.0);
        
        assert_eq!(effect.iterations(), 2);
        
        effect.set_iterations(10);
        assert_eq!(effect.iterations(), 10);
    }
}

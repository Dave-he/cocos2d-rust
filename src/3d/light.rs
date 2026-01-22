use crate::math::Vec3;
use crate::base::types::Color3B;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LightType {
    DIRECTIONAL,
    POINT,
    SPOT,
    AMBIENT,
}

#[derive(Debug)]
pub struct Light {
    light_type: LightType,
    color: Color3B,
    intensity: f32,
    direction: Vec3,
    position: Vec3,
    range: f32,
    spot_angle: f32,
    spot_exponent: f32,
    cast_shadows: bool,
    shadow_depth: f32,
    shadow_frustum_size: f32,
    enabled: bool,
}

impl Light {
    pub fn new() -> Light {
        Light {
            light_type: LightType::DIRECTIONAL,
            color: Color3B::WHITE,
            intensity: 1.0,
            direction: Vec3::new(0.0, -1.0, 0.0),
            position: Vec3::ZERO,
            range: 1000.0,
            spot_angle: 45.0,
            spot_exponent: 1.0,
            cast_shadows: false,
            shadow_depth: 1.0,
            shadow_frustum_size: 100.0,
            enabled: true,
        }
    }

    pub fn create_directional(direction: Vec3, color: Color3B) -> Light {
        Light {
            light_type: LightType::DIRECTIONAL,
            color,
            intensity: 1.0,
            direction,
            position: Vec3::ZERO,
            range: 0.0,
            spot_angle: 0.0,
            spot_exponent: 0.0,
            cast_shadows: false,
            shadow_depth: 1.0,
            shadow_frustum_size: 100.0,
            enabled: true,
        }
    }

    pub fn create_point(position: Vec3, color: Color3B, range: f32) -> Light {
        Light {
            light_type: LightType::POINT,
            color,
            intensity: 1.0,
            direction: Vec3::ZERO,
            position,
            range,
            spot_angle: 0.0,
            spot_exponent: 0.0,
            cast_shadows: false,
            shadow_depth: 1.0,
            shadow_frustum_size: 100.0,
            enabled: true,
        }
    }

    pub fn create_spot(position: Vec3, direction: Vec3, color: Color3B, angle: f32, range: f32) -> Light {
        Light {
            light_type: LightType::SPOT,
            color,
            intensity: 1.0,
            direction,
            position,
            range,
            spot_angle: angle,
            spot_exponent: 1.0,
            cast_shadows: false,
            shadow_depth: 1.0,
            shadow_frustum_size: 100.0,
            enabled: true,
        }
    }

    pub fn get_type(&self) -> LightType {
        self.light_type
    }

    pub fn get_color(&self) -> Color3B {
        self.color
    }

    pub fn set_color(&mut self, color: Color3B) {
        self.color = color;
    }

    pub fn get_intensity(&self) -> f32 {
        self.intensity
    }

    pub fn set_intensity(&mut self, intensity: f32) {
        self.intensity = intensity;
    }

    pub fn get_direction(&self) -> Vec3 {
        self.direction
    }

    pub fn set_direction(&mut self, direction: Vec3) {
        self.direction = direction;
    }

    pub fn get_position(&self) -> Vec3 {
        self.position
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }

    pub fn get_range(&self) -> f32 {
        self.range
    }

    pub fn set_range(&mut self, range: f32) {
        self.range = range;
    }

    pub fn get_spot_angle(&self) -> f32 {
        self.spot_angle
    }

    pub fn set_spot_angle(&mut self, angle: f32) {
        self.spot_angle = angle;
    }

    pub fn get_spot_exponent(&self) -> f32 {
        self.spot_exponent
    }

    pub fn set_spot_exponent(&mut self, exponent: f32) {
        self.spot_exponent = exponent;
    }

    pub fn is_cast_shadows(&self) -> bool {
        self.cast_shadows
    }

    pub fn set_cast_shadows(&mut self, cast: bool) {
        self.cast_shadows = cast;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

#[derive(Debug)]
pub struct LightManager {
    lights: Vec<Light>,
    ambient_color: Color3B,
}

impl LightManager {
    pub fn new() -> LightManager {
        LightManager {
            lights: Vec::new(),
            ambient_color: Color3B::BLACK,
        }
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn get_lights(&self) -> &Vec<Light> {
        &self.lights
    }

    pub fn get_ambient_color(&self) -> Color3B {
        self.ambient_color
    }

    pub fn set_ambient_color(&mut self, color: Color3B) {
        self.ambient_color = color;
    }
}

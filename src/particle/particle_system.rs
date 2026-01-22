use crate::math::{Vec2, Vec3, Vec4};
use crate::base::types::Color4F;
use std::f32::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendType {
    ADD,
    SUBTRACT,
    SCREEN,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmitterType {
    GRAVITY,
    RADIUS,
}

#[derive(Debug)]
pub struct Particle {
    position: Vec3,
    velocity: Vec3,
    acceleration: Vec3,
    color: Color4F,
    color_delta: Color4F,
    size: f32,
    size_delta: f32,
    rotation: f32,
    rotation_delta: f32,
    life: f32,
    max_life: f32,
    start_size: f32,
    end_size: f32,
    start_color: Color4F,
    end_color: Color4F,
}

impl Particle {
    pub fn new() -> Particle {
        Particle {
            position: Vec3::ZERO,
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            color: Color4F::WHITE,
            color_delta: Color4F::WHITE,
            size: 1.0,
            size_delta: 0.0,
            rotation: 0.0,
            rotation_delta: 0.0,
            life: 0.0,
            max_life: 0.0,
            start_size: 1.0,
            end_size: 1.0,
            start_color: Color4F::WHITE,
            end_color: Color4F::WHITE,
        }
    }

    pub fn reset(&mut self) {
        self.position = Vec3::ZERO;
        self.velocity = Vec3::ZERO;
        self.acceleration = Vec3::ZERO;
        self.color = Color4F::WHITE;
        self.size = self.start_size;
        self.rotation = 0.0;
        self.life = self.max_life;
    }

    pub fn update(&mut self, delta: f32) {
        self.velocity += self.acceleration * delta;
        self.position += self.velocity * delta;
        self.life -= delta;
        self.rotation += self.rotation_delta * delta;

        let life_ratio = self.life / self.max_life;
        self.color.r = self.start_color.r + (self.end_color.r - self.start_color.r) * (1.0 - life_ratio);
        self.color.g = self.start_color.g + (self.end_color.g - self.start_color.g) * (1.0 - life_ratio);
        self.color.b = self.start_color.b + (self.end_color.b - self.start_color.b) * (1.0 - life_ratio);
        self.color.a = self.start_color.a + (self.end_color.a - self.start_color.a) * (1.0 - life_ratio);

        let size_ratio = 1.0 - life_ratio;
        self.size = self.start_size + (self.end_size - self.start_size) * size_ratio;
    }
}

#[derive(Debug)]
pub struct ParticleEmitterConfig {
    pub emitter_type: EmitterType,
    pub blend_type: BlendType,
    pub start_size: f32,
    pub end_size: f32,
    pub start_size_var: f32,
    pub end_size_var: f32,
    pub start_spin: f32,
    pub end_spin: f32,
    pub start_spin_var: f32,
    pub end_spin_var: f32,
    pub emission_rate: f32,
    pub total_particles: u32,
    pub life: f32,
    pub life_var: f32,
    pub angle: f32,
    pub angle_var: f32,
    pub speed: f32,
    pub speed_var: f32,
    pub x_speed: f32,
    pub y_speed: f32,
    pub radial_speed: f32,
    pub radial_accel: f32,
    pub tangential_accel: f32,
    pub gravity: Vec3,
    pub start_color: Color4F,
    pub end_color: Color4F,
    pub start_color_var: Color4F,
    pub end_color_var: Color4F,
    pub position: Vec3,
    pub pos_var: Vec3,
    pub start_radius: f32,
    pub end_radius: f32,
    pub rotate_per_second: f32,
    pub rotate_per_second_var: f32,
}

impl Default for ParticleEmitterConfig {
    fn default() -> Self {
        ParticleEmitterConfig {
            emitter_type: EmitterType::GRAVITY,
            blend_type: BlendType::ADD,
            start_size: 50.0,
            end_size: 0.0,
            start_size_var: 0.0,
            end_size_var: 0.0,
            start_spin: 0.0,
            end_spin: 0.0,
            start_spin_var: 0.0,
            end_spin_var: 0.0,
            emission_rate: 10.0,
            total_particles: 100,
            life: 1.0,
            life_var: 0.0,
            angle: 90.0,
            angle_var: 0.0,
            speed: 100.0,
            speed_var: 0.0,
            x_speed: 0.0,
            y_speed: 0.0,
            radial_speed: 0.0,
            radial_accel: 0.0,
            tangential_accel: 0.0,
            gravity: Vec3::ZERO,
            start_color: Color4F::WHITE,
            end_color: Color4F::WHITE,
            start_color_var: Color4F::BLACK,
            end_color_var: Color4F::BLACK,
            position: Vec3::ZERO,
            pos_var: Vec3::ZERO,
            start_radius: 0.0,
            end_radius: 0.0,
            rotate_per_second: 0.0,
            rotate_per_second_var: 0.0,
        }
    }
}

#[derive(Debug)]
pub struct ParticleSystem {
    config: ParticleEmitterConfig,
    particles: Vec<Particle>,
    emission_count: u32,
    elapsed: f32,
    duration: f32,
    is_active: bool,
    is_visible: bool,
    auto_remove: bool,
    texture: Option<()>,
}

impl ParticleSystem {
    pub fn new() -> ParticleSystem {
        ParticleSystem {
            config: ParticleEmitterConfig::default(),
            particles: Vec::new(),
            emission_count: 0,
            elapsed: 0.0,
            duration: -1.0,
            is_active: false,
            is_visible: true,
            auto_remove: false,
            texture: None,
        }
    }

    pub fn create() -> ParticleSystem {
        ParticleSystem::new()
    }

    pub fn create_with_file(file: &str) -> Option<ParticleSystem> {
        Some(ParticleSystem::new())
    }

    pub fn init(&mut self) {
        self.particles.clear();
        self.particles.reserve(self.config.total_particles as usize);
    }

    pub fn set_texture(&mut self, texture: ()) {
        self.texture = Some(texture);
    }

    pub fn start(&mut self) {
        self.is_active = true;
        self.elapsed = 0.0;
        self.emission_count = 0;
    }

    pub fn stop(&mut self) {
        self.is_active = false;
    }

    pub fn reset(&mut self) {
        self.particles.clear();
        self.elapsed = 0.0;
        self.emission_count = 0;
    }

    pub fn update(&mut self, delta: f32) {
        if !self.is_active {
            return;
        }

        self.elapsed += delta;

        while self.emission_count < self.config.total_particles
            && (self.duration < 0.0 || self.elapsed < self.duration) {
            self.emit_particle();
            self.emission_count += 1;
        }

        for particle in &mut self.particles {
            particle.update(delta);
        }

        self.particles.retain(|p| p.life > 0.0);

        if self.auto_remove && self.particles.is_empty() && self.emission_count >= self.config.total_particles {
            self.is_active = false;
        }
    }

    fn emit_particle(&mut self) {
        let mut particle = Particle::new();
        particle.max_life = self.config.life + self.config.life_var * (rand::random::<f32>() - 0.5);
        particle.life = particle.max_life;
        particle.start_size = self.config.start_size + self.config.start_size_var * (rand::random::<f32>() - 0.5);
        particle.end_size = self.config.end_size + self.config.end_size_var * (rand::random::<f32>() - 0.5);
        particle.size = particle.start_size;

        // Calculate initial velocity based on emitter type
        match self.config.emitter_type {
            EmitterType::GRAVITY => {
                let angle = (self.config.angle + self.config.angle_var * (rand::random::<f32>() - 0.5)) * PI / 180.0;
                let speed = self.config.speed + self.config.speed_var * (rand::random::<f32>() - 0.5);
                particle.velocity.x = angle.cos() * speed;
                particle.velocity.y = angle.sin() * speed;
                particle.velocity.z = 0.0;
                particle.acceleration = self.config.gravity;
            }
            EmitterType::RADIUS => {
                particle.position.x = self.config.start_radius;
                particle.velocity.z = (rand::random::<f32>() - 0.5) * self.config.rotate_per_second * PI / 180.0;
            }
        }

        particle.color = particle.start_color;

        self.particles.push(particle);
    }

    pub fn get_particle_count(&self) -> u32 {
        self.particles.len() as u32
    }

    pub fn get_capacity(&self) -> u32 {
        self.config.total_particles
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn is_visible(&self) -> bool {
        self.is_visible
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.is_visible = visible;
    }
}

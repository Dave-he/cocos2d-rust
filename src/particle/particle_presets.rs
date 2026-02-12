use super::particle_system::{ParticleSystem, ParticleEmitterConfig, EmitterType, BlendType};
use crate::base::types::Color4F;
use crate::math::Vec3;

pub struct ParticlePresets;

impl ParticlePresets {
    pub fn create_fire() -> ParticleSystem {
        let mut system = ParticleSystem::new();
        let mut config = ParticleEmitterConfig::default();
        
        config.emitter_type = EmitterType::GRAVITY;
        config.blend_type = BlendType::ADD;
        config.total_particles = 250;
        config.emission_rate = 250.0;
        config.life = 3.0;
        config.life_var = 1.0;
        
        config.angle = 90.0;
        config.angle_var = 10.0;
        config.speed = 60.0;
        config.speed_var = 20.0;
        
        config.start_size = 54.0;
        config.start_size_var = 10.0;
        config.end_size = 0.0;
        config.end_size_var = 0.0;
        
        config.start_spin = 0.0;
        config.start_spin_var = 0.0;
        config.end_spin = 0.0;
        config.end_spin_var = 0.0;
        
        config.start_color = Color4F::new(0.76, 0.25, 0.12, 1.0);
        config.end_color = Color4F::new(0.0, 0.0, 0.0, 1.0);
        config.start_color_var = Color4F::new(0.0, 0.0, 0.0, 0.0);
        config.end_color_var = Color4F::new(0.0, 0.0, 0.0, 0.0);
        
        config.position = Vec3::new(0.0, 0.0, 0.0);
        config.pos_var = Vec3::new(40.0, 20.0, 0.0);
        
        config.gravity = Vec3::new(0.0, -240.0, 0.0);
        config.radial_accel = 0.0;
        config.tangential_accel = 0.0;
        
        system.set_config(config);
        system
    }

    pub fn create_smoke() -> ParticleSystem {
        let mut system = ParticleSystem::new();
        let mut config = ParticleEmitterConfig::default();
        
        config.emitter_type = EmitterType::GRAVITY;
        config.blend_type = BlendType::ADD;
        config.total_particles = 200;
        config.emission_rate = 200.0;
        config.life = 4.0;
        config.life_var = 1.0;
        
        config.angle = 90.0;
        config.angle_var = 5.0;
        config.speed = 25.0;
        config.speed_var = 10.0;
        
        config.start_size = 60.0;
        config.start_size_var = 10.0;
        config.end_size = 0.0;
        config.end_size_var = 0.0;
        
        config.start_spin = 0.0;
        config.end_spin = 0.0;
        
        config.start_color = Color4F::new(0.8, 0.8, 0.8, 1.0);
        config.end_color = Color4F::new(0.5, 0.5, 0.5, 0.0);
        config.start_color_var = Color4F::new(0.02, 0.02, 0.02, 0.0);
        config.end_color_var = Color4F::new(0.0, 0.0, 0.0, 0.0);
        
        config.position = Vec3::ZERO;
        config.pos_var = Vec3::new(20.0, 0.0, 0.0);
        
        config.gravity = Vec3::new(0.0, -240.0, 0.0);
        config.radial_accel = 0.0;
        config.tangential_accel = 0.0;
        
        system.set_config(config);
        system
    }

    pub fn create_explosion() -> ParticleSystem {
        let mut system = ParticleSystem::new();
        let mut config = ParticleEmitterConfig::default();
        
        config.emitter_type = EmitterType::GRAVITY;
        config.blend_type = BlendType::ADD;
        config.total_particles = 700;
        config.emission_rate = 700.0;
        config.life = 5.0;
        config.life_var = 1.0;
        
        config.angle = 90.0;
        config.angle_var = 360.0;
        config.speed = 70.0;
        config.speed_var = 40.0;
        
        config.start_size = 15.0;
        config.start_size_var = 10.0;
        config.end_size = 60.0;
        config.end_size_var = 10.0;
        
        config.start_spin = 0.0;
        config.end_spin = 0.0;
        
        config.start_color = Color4F::new(0.7, 0.1, 0.2, 1.0);
        config.end_color = Color4F::new(0.5, 0.5, 0.5, 0.0);
        config.start_color_var = Color4F::new(0.0, 0.0, 0.0, 0.0);
        config.end_color_var = Color4F::new(0.0, 0.0, 0.0, 0.0);
        
        config.position = Vec3::ZERO;
        config.pos_var = Vec3::ZERO;
        
        config.gravity = Vec3::ZERO;
        config.radial_accel = 0.0;
        config.tangential_accel = 0.0;
        
        system.set_config(config);
        system
    }

    pub fn create_snow() -> ParticleSystem {
        let mut system = ParticleSystem::new();
        let mut config = ParticleEmitterConfig::default();
        
        config.emitter_type = EmitterType::GRAVITY;
        config.blend_type = BlendType::ADD;
        config.total_particles = 700;
        config.emission_rate = 10.0;
        config.life = 3.0;
        config.life_var = 1.0;
        
        config.angle = -90.0;
        config.angle_var = 5.0;
        config.speed = 130.0;
        config.speed_var = 30.0;
        
        config.start_size = 10.0;
        config.start_size_var = 5.0;
        config.end_size = 0.0;
        config.end_size_var = 0.0;
        
        config.start_spin = 0.0;
        config.end_spin = 0.0;
        
        config.start_color = Color4F::new(1.0, 1.0, 1.0, 1.0);
        config.end_color = Color4F::new(1.0, 1.0, 1.0, 0.0);
        config.start_color_var = Color4F::new(0.0, 0.0, 0.0, 0.0);
        config.end_color_var = Color4F::new(0.0, 0.0, 0.0, 0.0);
        
        config.position = Vec3::new(0.0, 0.0, 0.0);
        config.pos_var = Vec3::new(480.0, 0.0, 0.0);
        
        config.gravity = Vec3::new(0.0, 0.0, 0.0);
        config.radial_accel = 0.0;
        config.tangential_accel = 0.0;
        
        system.set_config(config);
        system
    }

    pub fn create_rain() -> ParticleSystem {
        let mut system = ParticleSystem::new();
        let mut config = ParticleEmitterConfig::default();
        
        config.emitter_type = EmitterType::GRAVITY;
        config.blend_type = BlendType::ADD;
        config.total_particles = 1000;
        config.emission_rate = 10.0;
        config.life = 4.5;
        config.life_var = 0.0;
        
        config.angle = -90.0;
        config.angle_var = 5.0;
        config.speed = 180.0;
        config.speed_var = 50.0;
        
        config.start_size = 4.0;
        config.start_size_var = 2.0;
        config.end_size = 0.0;
        config.end_size_var = 0.0;
        
        config.start_spin = 0.0;
        config.end_spin = 0.0;
        
        config.start_color = Color4F::new(0.7, 0.8, 1.0, 1.0);
        config.end_color = Color4F::new(0.7, 0.8, 1.0, 0.5);
        config.start_color_var = Color4F::new(0.0, 0.0, 0.0, 0.0);
        config.end_color_var = Color4F::new(0.0, 0.0, 0.0, 0.0);
        
        config.position = Vec3::new(0.0, 0.0, 0.0);
        config.pos_var = Vec3::new(480.0, 0.0, 0.0);
        
        config.gravity = Vec3::new(10.0, -10.0, 0.0);
        config.radial_accel = 0.0;
        config.tangential_accel = 0.0;
        
        system.set_config(config);
        system
    }

    pub fn create_galaxy() -> ParticleSystem {
        let mut system = ParticleSystem::new();
        let mut config = ParticleEmitterConfig::default();
        
        config.emitter_type = EmitterType::GRAVITY;
        config.blend_type = BlendType::ADD;
        config.total_particles = 200;
        config.emission_rate = 200.0;
        config.life = 4.0;
        config.life_var = 1.0;
        
        config.angle = 90.0;
        config.angle_var = 360.0;
        config.speed = 60.0;
        config.speed_var = 10.0;
        
        config.start_size = 37.0;
        config.start_size_var = 10.0;
        config.end_size = 0.0;
        config.end_size_var = 0.0;
        
        config.start_spin = 0.0;
        config.end_spin = 0.0;
        
        config.start_color = Color4F::new(0.12, 0.25, 0.76, 1.0);
        config.end_color = Color4F::new(0.0, 0.0, 0.0, 1.0);
        config.start_color_var = Color4F::new(0.0, 0.0, 0.0, 0.0);
        config.end_color_var = Color4F::new(0.0, 0.0, 0.0, 0.0);
        
        config.position = Vec3::ZERO;
        config.pos_var = Vec3::ZERO;
        
        config.gravity = Vec3::ZERO;
        config.radial_accel = -80.0;
        config.tangential_accel = 80.0;
        
        system.set_config(config);
        system
    }

    pub fn create_fireworks() -> ParticleSystem {
        let mut system = ParticleSystem::new();
        let mut config = ParticleEmitterConfig::default();
        
        config.emitter_type = EmitterType::GRAVITY;
        config.blend_type = BlendType::ADD;
        config.total_particles = 1500;
        config.emission_rate = 1500.0;
        config.life = 3.5;
        config.life_var = 1.0;
        
        config.angle = 90.0;
        config.angle_var = 20.0;
        config.speed = 180.0;
        config.speed_var = 50.0;
        
        config.start_size = 8.0;
        config.start_size_var = 2.0;
        config.end_size = 0.0;
        config.end_size_var = 0.0;
        
        config.start_spin = 0.0;
        config.end_spin = 0.0;
        
        config.start_color = Color4F::new(0.5, 0.5, 0.5, 1.0);
        config.end_color = Color4F::new(0.1, 0.1, 0.1, 0.2);
        config.start_color_var = Color4F::new(0.5, 0.5, 0.5, 0.1);
        config.end_color_var = Color4F::new(0.1, 0.1, 0.1, 0.2);
        
        config.position = Vec3::ZERO;
        config.pos_var = Vec3::ZERO;
        
        config.gravity = Vec3::new(0.0, -90.0, 0.0);
        config.radial_accel = 0.0;
        config.tangential_accel = 0.0;
        
        system.set_config(config);
        system
    }

    pub fn create_spiral() -> ParticleSystem {
        let mut system = ParticleSystem::new();
        let mut config = ParticleEmitterConfig::default();
        
        config.emitter_type = EmitterType::GRAVITY;
        config.blend_type = BlendType::ADD;
        config.total_particles = 500;
        config.emission_rate = 500.0;
        config.life = 12.0;
        config.life_var = 0.0;
        
        config.angle = 90.0;
        config.angle_var = 0.0;
        config.speed = 150.0;
        config.speed_var = 0.0;
        
        config.start_size = 20.0;
        config.start_size_var = 0.0;
        config.end_size = 0.0;
        config.end_size_var = 0.0;
        
        config.start_spin = 0.0;
        config.end_spin = 0.0;
        
        config.start_color = Color4F::new(0.5, 0.5, 0.5, 1.0);
        config.end_color = Color4F::new(0.5, 0.5, 0.5, 1.0);
        config.start_color_var = Color4F::new(0.5, 0.5, 0.5, 0.0);
        config.end_color_var = Color4F::new(0.0, 0.0, 0.0, 0.0);
        
        config.position = Vec3::ZERO;
        config.pos_var = Vec3::ZERO;
        
        config.gravity = Vec3::ZERO;
        config.radial_accel = -380.0;
        config.tangential_accel = 45.0;
        
        system.set_config(config);
        system
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_fire() {
        let system = ParticlePresets::create_fire();
        assert_eq!(system.get_capacity(), 250);
    }

    #[test]
    fn test_create_smoke() {
        let system = ParticlePresets::create_smoke();
        assert_eq!(system.get_capacity(), 200);
    }

    #[test]
    fn test_create_explosion() {
        let system = ParticlePresets::create_explosion();
        assert_eq!(system.get_capacity(), 700);
    }

    #[test]
    fn test_create_snow() {
        let system = ParticlePresets::create_snow();
        assert_eq!(system.get_capacity(), 700);
    }

    #[test]
    fn test_create_rain() {
        let system = ParticlePresets::create_rain();
        assert_eq!(system.get_capacity(), 1000);
    }

    #[test]
    fn test_create_galaxy() {
        let system = ParticlePresets::create_galaxy();
        assert_eq!(system.get_capacity(), 200);
    }

    #[test]
    fn test_create_fireworks() {
        let system = ParticlePresets::create_fireworks();
        assert_eq!(system.get_capacity(), 1500);
    }

    #[test]
    fn test_create_spiral() {
        let system = ParticlePresets::create_spiral();
        assert_eq!(system.get_capacity(), 500);
    }

    #[test]
    fn test_all_presets_valid() {
        let presets = vec![
            ParticlePresets::create_fire(),
            ParticlePresets::create_smoke(),
            ParticlePresets::create_explosion(),
            ParticlePresets::create_snow(),
            ParticlePresets::create_rain(),
            ParticlePresets::create_galaxy(),
            ParticlePresets::create_fireworks(),
            ParticlePresets::create_spiral(),
        ];

        for system in presets {
            assert!(system.get_capacity() > 0);
        }
    }
}

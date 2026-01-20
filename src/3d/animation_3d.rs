use crate::base::Ref;
use crate::math::{Vec3, Quaternion};
use std::f32::consts::PI;

#[derive(Debug)]
pub struct AnimationCurve {
    key_frames: Vec<KeyFrame>,
}

#[derive(Debug, Clone)]
pub struct KeyFrame {
    time: f32,
    value: f32,
    in_tangent: f32,
    out_tangent: f32,
}

impl AnimationCurve {
    pub fn new() -> AnimationCurve {
        AnimationCurve {
            key_frames: Vec::new(),
        }
    }

    pub fn add_key(&mut self, time: f32, value: f32) {
        self.key_frames.push(KeyFrame {
            time,
            value,
            in_tangent: 0.0,
            out_tangent: 0.0,
        });
    }

    pub fn get_value(&self, time: f32) -> f32 {
        if self.key_frames.is_empty() {
            return 0.0;
        }
        // Simple linear interpolation
        let mut prev = &self.key_frames[0];
        for key in &self.key_frames {
            if key.time >= time {
                break;
            }
            prev = key;
        }
        prev.value
    }
}

#[derive(Debug)]
pub struct Animation3D {
    name: String,
    duration: f32,
    position_curves: Vec<AnimationCurve>,
    rotation_curves: Vec<AnimationCurve>,
    scale_curves: Vec<AnimationCurve>,
}

impl Animation3D {
    pub fn new() -> Animation3D {
        Animation3D {
            name: String::new(),
            duration: 0.0,
            position_curves: Vec::new(),
            rotation_curves: Vec::new(),
            scale_curves: Vec::new(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn get_duration(&self) -> f32 {
        self.duration
    }

    pub fn set_duration(&mut self, duration: f32) {
        self.duration = duration;
    }
}

#[derive(Debug)]
pub struct Animate3D {
    animation: Ref<Animation3D>,
    speed: f32,
    current_time: f32,
}

impl Animate3D {
    pub fn new(animation: Ref<Animation3D>) -> Animate3D {
        Animate3D {
            animation,
            speed: 1.0,
            current_time: 0.0,
        }
    }

    pub fn get_speed(&self) -> f32 {
        self.speed
    }

    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed;
    }

    pub fn get_current_time(&self) -> f32 {
        self.current_time
    }

    pub fn set_current_time(&mut self, time: f32) {
        self.current_time = time;
    }
}

#[derive(Debug)]
pub struct AnimationClip {
    name: String,
    start_time: f32,
    end_time: f32,
    loops: bool,
}

impl AnimationClip {
    pub fn new(name: &str) -> AnimationClip {
        AnimationClip {
            name: name.to_string(),
            start_time: 0.0,
            end_time: 1.0,
            loops: false,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_time_range(&mut self, start: f32, end: f32) {
        self.start_time = start;
        self.end_time = end;
    }

    pub fn is_loop(&self) -> bool {
        self.loops
    }

    pub fn set_loop(&mut self, loop_enabled: bool) {
        self.loops = loop_enabled;
    }
}

#[derive(Debug)]
pub struct AnimationComponent {
    animations: Vec<Ref<Animation3D>>,
    current_animation: Option<Ref<Animation3D>>,
    current_time: f32,
    speed: f32,
    playing: bool,
}

impl AnimationComponent {
    pub fn new() -> AnimationComponent {
        AnimationComponent {
            animations: Vec::new(),
            current_animation: None,
            current_time: 0.0,
            speed: 1.0,
            playing: false,
        }
    }

    pub fn add_animation(&mut self, animation: Ref<Animation3D>) {
        self.animations.push(animation);
    }

    pub fn play(&mut self, animation_name: &str) {
        for anim in &self.animations {
            if anim.get_name() == animation_name {
                self.current_animation = Some(anim.clone());
                self.current_time = 0.0;
                self.playing = true;
                break;
            }
        }
    }

    pub fn stop(&mut self) {
        self.playing = false;
    }

    pub fn pause(&mut self) {
        self.playing = false;
    }

    pub fn resume(&mut self) {
        self.playing = true;
    }

    pub fn is_playing(&self) -> bool {
        self.playing
    }

    pub fn get_speed(&self) -> f32 {
        self.speed
    }

    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed;
    }

    pub fn update(&mut self, delta: f32) {
        if !self.playing || self.current_animation.is_none() {
            return;
        }

        self.current_time += delta * self.speed;

        if let Some(anim) = &self.current_animation {
            if self.current_time >= anim.get_duration() {
                self.current_time = 0.0;
            }
        }
    }

    pub fn get_bone_transform(&self, bone_name: &str, position: &mut Vec3, rotation: &mut Quaternion, scale: &mut Vec3) {
        *position = Vec3::ZERO;
        *rotation = Quaternion::identity();
        *scale = Vec3::new(1.0, 1.0, 1.0);
    }
}

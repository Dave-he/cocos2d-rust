pub mod math;
pub mod base;
pub mod platform;
pub mod sprite;
pub mod scene;
pub mod action;
pub mod renderer;
pub mod audio; // 恢复
pub mod network;
pub mod physics;
pub mod ui;
pub mod particle;
pub mod tilemap;
pub mod backend;
pub mod _3d;
pub mod label;
pub mod menu;
pub mod input;
pub mod animation;
pub mod shader;
pub mod transition;

pub use base::{Director, Scene, Node, Color3B, Color4B, Color4F, Size, Rect};
pub use math::{Vec2, Vec3, Vec4, Mat4, Quaternion};
pub use scene::{Layer, LayerColor};
pub use renderer::{Renderer, Texture, Material, RenderTexture};
pub use audio::AudioEngine; // 恢复
pub use network::network::HttpClient;
pub use physics::{
    PhysicsWorld, PhysicsBody, PhysicsShape, PhysicsJoint, PhysicsContact,
    PhysicsBodyType, PhysicsShapeType, JointType, PhysicsMaterial,
    RayCastInfo, QueryInfo,
    Physics3DWorld, Physics3DBody, Physics3DShape, Physics3DConstraint,
    Physics3DConstraintType,
};
pub use ui::{Widget, Button, Slider, TextField, Layout};
pub use particle::ParticleSystem;
pub use tilemap::tilemap_layer::TileMap;
pub use label::{Label, LabelTTF, LabelAtlas};
pub use menu::{Menu, MenuItem, MenuItemLabel, MenuItemImage};
pub use input::{Touch, TouchDispatcher, KeyCode, MouseButton};
pub use animation::{Animation, AnimationCache, SpriteFrame, SpriteFrameCache, Animate};
pub use shader::{ShaderProgram, ShaderCache, BuiltInShaders};
pub use transition::{TransitionScene, FadeTransition, SlideTransition, FlipTransition, ZoomTransition, RotateTransition};

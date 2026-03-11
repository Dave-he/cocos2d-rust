pub mod _3d;
pub mod action;
pub mod animation;
pub mod audio;
pub mod backend;
pub mod base;
pub mod input;
pub mod label;
pub mod math;
pub mod menu;
pub mod network;
pub mod particle;
pub mod physics;
pub mod platform;
pub mod renderer;
pub mod scene;
pub mod shader;
pub mod sprite;
pub mod tilemap;
pub mod transition;
pub mod camera;
pub mod effects;
pub mod ui;

// 重新导出常用类型
pub use base::{
    Color3B, Color4B, Color4F, Size, Rect,
    DebugStats, DebugConsole, DebugProfiler, DebugLayer,
    LogLevel, LogEntry, ConsoleCommand,
    ProfilerScope, ProfilerCategory, ProfilerEntry,
    DebugPanel, DebugPanelConfig,
    NotificationCenter, Notification, DefaultNotification,
    NotificationObserver, NotificationPriority,
    AsyncTask, AsyncTaskResult, AsyncTaskHandler,
    TaskStatus, TaskProgress, TaskGroup, AsyncTaskManager, ThreadPool
};
pub use base::director::{Director, Scene};
pub use scene::node::Node;
pub use math::{Vec2, Vec3, Vec4, Mat4, Quaternion};
pub use scene::{Layer, LayerColor};
pub use renderer::{Renderer, Texture, Material, RenderTexture};
pub use audio::AudioEngine;
pub use animation::{Animate, Animation, AnimationCache, SpriteFrame, SpriteFrameCache};
pub use input::{KeyCode, MouseButton, Touch, TouchDispatcher};
pub use label::{Label, LabelAtlas, LabelTTF};
pub use menu::{Menu, MenuItem, MenuItemImage, MenuItemLabel};
pub use network::network::HttpClient;
pub use platform::{UserDefault, UserDefaultStats};
pub use physics::{
    PhysicsWorld, PhysicsBody, PhysicsShape, PhysicsJoint, PhysicsContact,
    PhysicsBodyType, PhysicsShapeType, JointType, PhysicsMaterial,
    RayCastInfo, QueryInfo,
    Physics3DWorld, Physics3DBody, Physics3DShape, Physics3DConstraint,
    Physics3DConstraintType,
};
pub use ui::{
    Widget, Button, Slider, TextField, Layout, RichText,
    ScrollView, ListView, PageView,
    ScrollDirection, ListViewGravity,
    EditBox, EditBoxInputMode, EditBoxInputFlag, EditBoxReturnType,
    VideoPlayer, VideoState, VideoEventType,
    WebView, LoadState, WebViewEvent, ZoomRange,
};
pub use particle::ParticleSystem;
pub use shader::{BuiltInShaders, ShaderCache, ShaderProgram};
pub use tilemap::tilemap_layer::TileMap;
pub use transition::{
    FadeTransition, FlipTransition, RotateTransition, SlideTransition, TransitionScene,
    ZoomTransition,
};
pub use camera::Camera2D;
pub use effects::{ProgressTimer, ProgressTimerType, MotionStreak};

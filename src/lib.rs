pub mod _3d;
pub mod action;
pub mod animation;
pub mod audio;
pub mod backend;
pub mod base;
pub mod component;
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
    TaskStatus, TaskProgress, TaskGroup, AsyncTaskManager, ThreadPool,
    ResourceManager, ResourceType, ResourceState, ResourceMeta, ResourceStats,
    ObjectPool, Poolable,
};
pub use base::event_bus::{
    EventBus, SubscriberId, DeferredEventQueue,
    // 内置游戏事件
    SceneChangeEvent, NodeSceneEvent,
    TouchBeganEvent, TouchMovedEvent, TouchEndedEvent,
    KeyDownEvent, KeyUpEvent, MouseClickEvent,
    FrameEvent, GameEvent, MemoryWarningEvent,
    ResolutionChangeEvent, AudioEvent,
};
pub use base::debug_console::ConsoleHandler;
pub use base::debug_console::CommandResult;
pub use base::director::{Director, Scene};
// Node 从 scene::node 导出（比 base::director::Node 功能更完整）
pub use scene::node::Node;
pub use math::{Vec2, Vec3, Vec4, Mat4, Quaternion};
pub use scene::{Layer, LayerColor};
pub use scene::{DrawNode, DrawCommand};
pub use scene::ParallaxNode;
pub use scene::ClippingNode;
pub use renderer::{Renderer, Texture, Material, RenderTexture};
pub use audio::AudioEngine;
pub use animation::{Animate, Animation, AnimationCache, SpriteFrame, SpriteFrameCache};
pub use animation::{
    Skeleton, SkeletonData, AnimationState, TrackEntry, SpineAnimation,
    Bone, BoneData, Skin, Slot, SlotData, SpineEventData,
};
pub use action::{
    Action, FiniteTimeAction, Speed, Follow, INVALID_TAG,
    ActionIntervalImpl,
    MoveBy, MoveTo, RotateBy, RotateTo, ScaleBy, ScaleTo, SkewBy,
    Blink, DelayTime, FadeTo, FadeIn, FadeOut,
    BezierBy, BezierConfig, JumpBy,
    CameraFollow, FollowAxis, FollowMode,
    EasingFunction,
    EaseIn, EaseOut, EaseInOut,
    EaseSineIn, EaseSineOut, EaseSineInOut,
    EaseExponentialIn, EaseExponentialOut, EaseExponentialInOut,
    EaseElasticIn, EaseElasticOut, EaseElasticInOut,
    EaseBounceIn, EaseBounceOut, EaseBounceInOut,
    EaseBackIn, EaseBackOut, EaseBackInOut,
};
pub use input::{
    KeyCode, MouseButton, Touch, TouchDispatcher,
    GamepadButton, GamepadAxis, GamepadManager, GamepadState, GamepadVibration, ButtonState, GamepadEvent,
};
pub use label::{Label, LabelAtlas, LabelTTF};
pub use menu::{Menu, MenuItem, MenuItemImage, MenuItemLabel};
pub use network::network::HttpClient;
pub use network::{
    WebSocket, WebSocketConfig, WebSocketDelegate, WebSocketEvent,
    WebSocketManager, WebSocketMessage, WebSocketState, WebSocketStats,
};
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
pub use ui::rich_text::{RichElement, RichElementType};
pub use particle::ParticleSystem;
pub use particle::ParticlePresets;
pub use particle::{ParticleEmitterConfig, EmitterType, BlendType};
pub use shader::{BuiltInShaders, ShaderCache, ShaderProgram};
pub use tilemap::tilemap_layer::TileMap;
pub use tilemap::{
    TmxParser, TmxMap, TmxMapBuilder, TmxTileset, TmxLayerRaw, TmxObject, TmxError,
};
pub use transition::{
    FadeTransition, FlipTransition, RotateTransition, SlideTransition, TransitionScene,
    ZoomTransition,
    // 扩展过场
    TransitionProgress, TransitionKind,
    SplitTransition, SplitDirection,
    PageTurnTransition, CrossfadeTransition, FlashTransition, MorphTransition,
};
pub use camera::Camera2D;
pub use effects::{ProgressTimer, ProgressTimerType, MotionStreak};
pub use effects::progress_timer::BarChangeRate;
pub use effects::{
    OffscreenTarget, RenderTextureFormat,
    PostProcessPipeline, PostProcessEffect,
};
pub use sprite::{Sprite, SpriteBatchNode, BatchData, SpriteQuad};
pub use backend::{
    PipelineState, PipelineCache,
    VertexLayout, VertexAttribute, VertexFormat,
    BlendDescriptor, BlendFactor, BlendOp,
    DepthDescriptor, CullMode, FillMode,
    ShaderRef,
};
pub use component::{
    Component, ComponentContainer, ComponentBase,
    TimerComponent, ScriptComponent, StateMachineComponent, PropertyComponent,
};

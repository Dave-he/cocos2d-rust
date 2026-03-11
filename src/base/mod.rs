pub mod async_task;
pub mod autorelease_pool;
pub mod debug_console;
pub mod debug_layer;
pub mod debug_profiler;
pub mod debug_stats;
pub mod director;
pub mod error;
pub mod event;
pub mod notification_center;
pub mod ref_count;
pub mod resource_manager;
pub mod scheduler;
pub mod script_binding;
pub mod types;

pub use async_task::{
    AsyncTask, AsyncTaskHandler, AsyncTaskManager, AsyncTaskResult, TaskGroup, TaskProgress,
    TaskStatus, ThreadPool,
};
pub use debug_console::{CommandResult, ConsoleCommand, ConsoleHandler, DebugConsole, LogEntry, LogLevel};
pub use debug_layer::{DebugLayer, DebugPanel, DebugPanelConfig};
pub use debug_profiler::{DebugProfiler, ProfilerCategory, ProfilerEntry, ProfilerScope};
pub use debug_stats::{DebugStats, StatValue};
// 只导出 Director 和 Scene，Node 由 scene::node 导出
pub use director::{Director, Node, Scene};
pub use error::{
    AudioError, AudioResult, EngineError, EngineResult, NetworkError, NetworkResult,
    PhysicsError, PhysicsResult, RenderError, RenderResult, ResourceError, ResourceResult,
    SceneError, SceneResult,
};
pub use notification_center::{
    DefaultNotification, Notification, NotificationCenter, NotificationObserver,
    NotificationPriority,
};
pub use ref_count::{Clonable, Ref, RefPtr};
pub use resource_manager::{
    ObjectPool, Poolable, PoolGuard, Resource, ResourceMeta, ResourceManager,
    ResourceState, ResourceStats, ResourceType,
};
pub use script_binding::{
    JSScriptEngine, LuaScriptEngine, NativeFunction, ScriptCallback,
    ScriptEngine, ScriptEngineConfig, ScriptEngineStats, ScriptError,
    ScriptErrorKind, ScriptLanguage, ScriptManager, ScriptResult, ScriptValue,
};
pub use types::{Color3B, Color4B, Color4F, Point, Rect, Size};
// 为方便起见，re-export Node (从 scene 模块)
// 注意: 不从 director 中导出 Node，避免与 scene::node::Node 冲突

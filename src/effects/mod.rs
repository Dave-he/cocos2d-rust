/// Special effects for cocos2d-rust
pub mod progress_timer;
pub mod motion_streak;
pub mod render_texture;

pub use progress_timer::{ProgressTimer, ProgressTimerType, BarChangeRate};
pub use motion_streak::MotionStreak;
pub use render_texture::{
    OffscreenTarget, RenderTextureFormat,
    PostProcessPipeline, PostProcessEffect,
    apply_grayscale, apply_invert, apply_blur,
    apply_bloom, apply_brightness_contrast,
    apply_hue_shift, apply_vignette,
};

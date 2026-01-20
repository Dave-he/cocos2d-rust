/// Platform abstraction layer for cocos2d-rust
pub mod file_utils;
pub mod application;
pub mod types;

pub use file_utils::FileUtils;
pub use application::Application;
pub use types::{Platform, KeyboardState};

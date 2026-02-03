/// Platform abstraction layer for cocos2d-rust
pub mod file_utils;
pub mod application;
pub mod types;
pub mod image;
pub mod user_default;

pub use file_utils::FileUtils;
pub use application::Application;
pub use types::{Platform, KeyboardState};
pub use self::image::Image;
pub use user_default::{UserDefault, UserDefaultStats};

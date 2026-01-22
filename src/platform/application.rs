/// Platform types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    Unknown,
    Windows,
    Linux,
    MacOS,
    iOS,
    Android,
}

impl Platform {
    /// Gets the current platform
    pub fn get_current_platform() -> Platform {
        #[cfg(target_os = "windows")]
        return Platform::Windows;
        #[cfg(target_os = "linux")]
        return Platform::Linux;
        #[cfg(target_os = "macos")]
        return Platform::MacOS;
        #[cfg(target_os = "ios")]
        return Platform::iOS;
        #[cfg(target_os = "android")]
        return Platform::Android;
        #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos", target_os = "ios", target_os = "android")))]
        return Platform::Unknown;
    }
}

/// Keyboard state
#[derive(Debug, Clone)]
pub struct KeyboardState {
    keys: std::collections::HashMap<u32, bool>,
}

impl KeyboardState {
    pub fn new() -> KeyboardState {
        KeyboardState {
            keys: std::collections::HashMap::new(),
        }
    }

    pub fn is_key_down(&self, key_code: u32) -> bool {
        self.keys.get(&key_code).cloned().unwrap_or(false)
    }

    pub fn set_key_down(&mut self, key_code: u32, down: bool) {
        self.keys.insert(key_code, down);
    }

    pub fn clear(&mut self) {
        self.keys.clear();
    }
}

/// Application delegate for platform-specific initialization
pub trait ApplicationDelegate {
    fn application_did_finish_launching(&mut self) -> bool;
    fn application_did_enter_background(&mut self);
    fn application_will_enter_foreground(&mut self);
    fn application_will_resign_active(&mut self);
    fn application_did_become_active(&mut self);
}

/// Application manages the main application lifecycle
pub struct Application {
    delegate: Option<Box<dyn ApplicationDelegate>>,
    running: bool,
    paused: bool,
}

impl Application {
    /// Creates a new Application
    pub fn new() -> Application {
        Application {
            delegate: None,
            running: false,
            paused: false,
        }
    }

    /// Runs the application
    pub fn run(&mut self) {
        self.running = true;
        self.paused = false;
    }

    /// Sets the application delegate
    pub fn set_delegate(&mut self, delegate: Box<dyn ApplicationDelegate>) {
        self.delegate = Some(delegate);
    }

    /// Gets the application delegate
    pub fn get_delegate(&mut self) -> Option<&mut Box<dyn ApplicationDelegate>> {
        self.delegate.as_mut()
    }

    /// Gets the application instance
    pub fn get_instance() -> &'static mut Application {
        static mut APPLICATION: Option<Application> = None;
        unsafe {
            if APPLICATION.is_none() {
                APPLICATION = Some(Application::new());
            }
            APPLICATION.as_mut().unwrap()
        }
    }

    /// Checks if the application is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Stops the application
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Pauses the application
    pub fn pause(&mut self) {
        self.paused = true;
    }

    /// Resumes the application
    pub fn resume(&mut self) {
        self.paused = false;
    }

    /// Gets the application name
    pub fn get_application_name() -> String {
        "cocos2d-rust".to_string()
    }

    /// Gets the application version
    pub fn get_application_version() -> String {
        "1.0.0".to_string()
    }

    /// Gets the current language
    pub fn get_current_language() -> String {
        "en".to_string()
    }

    /// Opens a URL
    pub fn open_url(url: &str) -> bool {
        // In a real implementation, this would use platform-specific APIs
        println!("Opening URL: {}", url);
        true
    }

    /// Gets the target platform
    pub fn get_target_platform() -> Platform {
        Platform::get_current_platform()
    }

    /// Gets the bundle identifier
    pub fn get_bundle_identifier() -> String {
        "org.cocos2d-rust".to_string()
    }

    /// Gets the resource path
    pub fn get_resource_path() -> String {
        "./Resources".to_string()
    }
}

use glutin::{
    config::{ConfigTemplateBuilder, GlConfig},
    context::{ContextAttributesBuilder, GlProfile, NotCurrentGlContext},
    display::GetGlDisplay,
    prelude::*,
    surface::{GlSurface, SurfaceAttributesBuilder, WindowSurface},
};
use glutin_winit::DisplayBuilder;
use raw_window_handle::HasRawWindowHandle;
use std::ffi::CString;
use std::num::NonZeroU32;
use std::rc::Rc;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::backend::opengl::OpenGLBackend;
use crate::base::Director;

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

/// Platform types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
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
        #[cfg(not(any(
            target_os = "windows",
            target_os = "linux",
            target_os = "macos",
            target_os = "ios",
            target_os = "android"
        )))]
        return Platform::Unknown;
    }
}

/// Application delegate for platform-specific initialization
pub trait ApplicationDelegate {
    fn application_did_finish_launching(&mut self) -> bool;
    fn application_did_enter_background(&mut self);
    fn application_will_enter_foreground(&mut self);
}

/// Application manages the main application lifecycle
pub struct Application {
    delegate: Option<Box<dyn ApplicationDelegate>>,
}

impl Application {
    /// Creates a new Application
    pub fn new() -> Application {
        Application { delegate: None }
    }

    /// Sets the application delegate
    pub fn set_delegate(&mut self, delegate: Box<dyn ApplicationDelegate>) {
        self.delegate = Some(delegate);
    }

    /// Runs the application
    pub fn run(&mut self) {
        let event_loop = EventLoop::new().unwrap();
        let window_builder = WindowBuilder::new()
            .with_title("Cocos2d-Rust")
            .with_inner_size(winit::dpi::LogicalSize::new(960.0, 640.0));

        let template = ConfigTemplateBuilder::new()
            .with_alpha_size(8)
            .with_transparency(true);

        let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));

        let (window, gl_config) = display_builder
            .build(&event_loop, template, |configs| {
                configs
                    .reduce(|accum, config| {
                        if config.num_samples() > accum.num_samples() {
                            config
                        } else {
                            accum
                        }
                    })
                    .unwrap()
            })
            .unwrap();

        let window = window.unwrap();

        let raw_window_handle = window.raw_window_handle();
        let gl_display = gl_config.display();

        let context_attributes = ContextAttributesBuilder::new()
            .with_profile(GlProfile::Core)
            .build(Some(raw_window_handle));

        let not_current_gl_context = unsafe {
            gl_display
                .create_context(&gl_config, &context_attributes)
                .expect("failed to create context")
        };

        let attrs = window.inner_size();
        let width = NonZeroU32::new(attrs.width).unwrap_or(NonZeroU32::new(1).unwrap());
        let height = NonZeroU32::new(attrs.height).unwrap_or(NonZeroU32::new(1).unwrap());
        let surface_attributes = SurfaceAttributesBuilder::<WindowSurface>::new().build(
            raw_window_handle,
            width,
            height,
        );

        let gl_surface = unsafe {
            gl_display
                .create_window_surface(&gl_config, &surface_attributes)
                .unwrap()
        };

        let gl_context = not_current_gl_context.make_current(&gl_surface).unwrap();

        // Initialize glow
        let glow_context = unsafe {
            glow::Context::from_loader_function(|s| {
                gl_display.get_proc_address(&CString::new(s).unwrap())
            })
        };
        let glow_context = Rc::new(glow_context);

        // Initialize Director with context
        let mut director = Director::get_instance();
        director.borrow_mut().set_gl_context(glow_context.clone());

        // Notify delegate
        if let Some(delegate) = &mut self.delegate {
            if !delegate.application_did_finish_launching() {
                return;
            }
        }

        event_loop
            .run(move |event, target| {
                target.set_control_flow(ControlFlow::Poll);

                match event {
                    Event::WindowEvent { event, .. } => match event {
                        WindowEvent::CloseRequested => target.exit(),
                        WindowEvent::Resized(size) => {
                            if size.width != 0 && size.height != 0 {
                                gl_surface.resize(
                                    &gl_context,
                                    NonZeroU32::new(size.width).unwrap(),
                                    NonZeroU32::new(size.height).unwrap(),
                                );
                                // director.update_view_port(...)
                            }
                        }
                        WindowEvent::RedrawRequested => {
                            // Main Loop
                            director.borrow_mut().main_loop();

                            // Swap buffers
                            gl_surface.swap_buffers(&gl_context).unwrap();

                            // Request next frame
                            window.request_redraw();
                        }
                        _ => (),
                    },
                    Event::AboutToWait => {
                        window.request_redraw();
                    }
                    _ => (),
                }
            })
            .unwrap();
    }

    pub fn get_target_platform() -> Platform {
        Platform::get_current_platform()
    }
}

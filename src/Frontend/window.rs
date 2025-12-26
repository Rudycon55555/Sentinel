// Sentinel/src/Frontend/window.rs

//! Sentinel Window Module
//!
//! Provides a clean, safe abstraction over Tauri's window system.
//! This allows Sentinel apps to create windows without touching
//! low-level Tauri APIs directly.
//!
//! Features:
//! - WindowOptions for simple configuration
//! - WindowBuilder for advanced customization
//! - create_window() wrapper for Tauri integration

use tauri::{Window, WindowBuilder as TauriWindowBuilder, Manager};

/// Basic window configuration.
///
/// This is intentionally simple so beginners can use it easily.
#[derive(Clone)]
pub struct WindowOptions {
    pub label: String,
    pub title: String,
    pub width: f64,
    pub height: f64,
    pub resizable: bool,
    pub fullscreen: bool,
}

impl Default for WindowOptions {
    fn default() -> Self {
        Self {
            label: "main".into(),
            title: "Sentinel App".into(),
            width: 900.0,
            height: 600.0,
            resizable: true,
            fullscreen: false,
        }
    }
}

/// Advanced window builder.
///
/// This gives developers more control than WindowOptions alone.
pub struct WindowBuilder {
    opts: WindowOptions,
}

impl WindowBuilder {
    /// Start a new builder with default options.
    pub fn new() -> Self {
        Self {
            opts: WindowOptions::default(),
        }
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.opts.label = label.into();
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.opts.title = title.into();
        self
    }

    pub fn size(mut self, width: f64, height: f64) -> Self {
        self.opts.width = width;
        self.opts.height = height;
        self
    }

    pub fn resizable(mut self, value: bool) -> Self {
        self.opts.resizable = value;
        self
    }

    pub fn fullscreen(mut self, value: bool) -> Self {
        self.opts.fullscreen = value;
        self
    }

    /// Finalize and create the window.
    pub fn build(self, app: &tauri::AppHandle) -> Result<Window, tauri::Error> {
        create_window(app, &self.opts)
    }
}

/// Create a window using Sentinel's safe wrapper.
///
/// This is what App.rs will call internally.
pub fn create_window(app: &tauri::AppHandle, opts: &WindowOptions) -> Result<Window, tauri::Error> {
    let builder = TauriWindowBuilder::new(app, &opts.label, tauri::WindowUrl::App("index.html".into()))
        .title(&opts.title)
        .inner_size(opts.width, opts.height)
        .resizable(opts.resizable)
        .fullscreen(opts.fullscreen);

    builder.build()
}

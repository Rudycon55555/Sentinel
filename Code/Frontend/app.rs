// Sentinel/src/Frontend/app.rs

//! Sentinel App Builder
//!
//! This module ties together all Frontend systems:
//! - Window creation
//! - GUI rendering
//! - Script injection
//! - Event bridging
//!
//! It provides a clean, beginner‑friendly API for launching
//! a Sentinel application.

use crate::Frontend::{
    window::{WindowOptions, WindowBuilder},
    events::attach_js_bridge,
    GUI::{Renderable, GUIContainer},
    Scripting::ScriptEngine,
};

use tauri::{AppHandle, Builder as TauriBuilder, Manager};

/// The main Sentinel application.
///
/// Developers will use this to configure and launch their app.
pub struct App {
    window_opts: WindowOptions,
    root_gui: GUIContainer,
}

impl App {
    /// Create a new Sentinel app with default settings.
    pub fn new() -> Self {
        Self {
            window_opts: WindowOptions::default(),
            root_gui: GUIContainer::new(),
        }
    }

    /// Set the window title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.window_opts.title = title.into();
        self
    }

    /// Set the window size.
    pub fn size(mut self, width: f64, height: f64) -> Self {
        self.window_opts.width = width;
        self.window_opts.height = height;
        self
    }

    /// Add a GUI component to the root container.
    pub fn mount(mut self, component: impl Renderable + 'static) -> Self {
        self.root_gui = self.root_gui.add(component);
        self
    }

    /// Launch the Sentinel application.
    ///
    /// This initializes Tauri, creates the window, injects GUI + scripts,
    /// and attaches the JS <-> Rust event bridge.
    pub fn run(self) {
        let window_opts = self.window_opts.clone();
        let root_gui = self.root_gui;

        TauriBuilder::default()
            .setup(move |app| {
                // Create the main window
                let window = WindowBuilder::new()
                    .label(&window_opts.label)
                    .title(&window_opts.title)
                    .size(window_opts.width, window_opts.height)
                    .resizable(window_opts.resizable)
                    .fullscreen(window_opts.fullscreen)
                    .build(app)?;

                // Attach JS <-> Rust event bridge
                attach_js_bridge(&window);

                // Inject GUI HTML
                let html = root_gui.render();
                let js = format!(
                    "document.body.innerHTML = `{}`;",
                    html.replace('`', "\\`")
                );
                let engine = ScriptEngine::new(&window);
                engine.execute(js);

                Ok(())
            })
            .run(tauri::generate_context!())
            .expect("Failed to run Sentinel app");
    }
}

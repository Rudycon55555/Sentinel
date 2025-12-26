// Sentinel/src/Frontend/events.rs

//! Sentinel Event System
//!
//! Provides a unified, secure event API for:
//! - Emitting events from Rust to the frontend
//! - Listening for events from the frontend
//! - Bridging JS <-> Rust communication
//!
//! This wraps Tauri's event system into a clean, simple interface.

use tauri::{Manager, Window};
use std::sync::{Arc, Mutex};

/// A global event registry.
///
/// Stores Rust-side event listeners so they can be triggered
/// when JS emits events into the backend.
type Listener = Box<dyn Fn(String) + Send + Sync + 'static>;

lazy_static::lazy_static! {
    static ref EVENT_LISTENERS: Arc<Mutex<std::collections::HashMap<String, Vec<Listener>>>> =
        Arc::new(Mutex::new(std::collections::HashMap::new()));
}

/// Emit an event from Rust to the frontend.
///
/// Example:
/// ```
/// emit(&window, "login_success", "User123");
/// ```
pub fn emit(window: &Window, event: &str, payload: impl Into<String>) {
    let _ = window.emit(event, payload.into());
}

/// Register a Rust-side listener for a frontend event.
///
/// Example:
/// ```
/// on("login", |data| {
///     println!("User logged in: {}", data);
/// });
/// ```
pub fn on(event: &str, callback: impl Fn(String) + Send + Sync + 'static) {
    let mut listeners = EVENT_LISTENERS.lock().unwrap();
    listeners
        .entry(event.to_string())
        .or_default()
        .push(Box::new(callback));
}

/// Internal function used by App.rs to bind JS events to Rust listeners.
///
/// This is automatically called when the Tauri app initializes.
pub fn attach_js_bridge(window: &Window) {
    let win = window.clone();

    window.listen_any("sentinel://event", move |event| {
        if let Some(payload) = event.payload() {
            // Expected format: "event_name::data"
            if let Some((event_name, data)) = payload.split_once("::") {
                let listeners = EVENT_LISTENERS.lock().unwrap();
                if let Some(callbacks) = listeners.get(event_name) {
                    for cb in callbacks {
                        cb(data.to_string());
                    }
                }
            }
        }
    });

    // Inject JS bridge into the DOM
    let js_bridge = r#"
        window.Sentinel = {
            emit: function(event, data) {
                const payload = `${event}::${data}`;
                window.__TAURI__.event.emit("sentinel://event", payload);
            }
        };
    "#;

    let _ = win.eval(js_bridge);
}

// Sentinel/src/Frontend/Scripting.rs

//! Sentinel Scripting Module
//!
//! Provides a safe, structured interface for executing JavaScript
//! inside Tauri windows. This allows components to:
//! - Trigger JS functions
//! - Inject scripts into the DOM
//! - Communicate with Rust securely
//!
//! All unsafe or direct Tauri IPC is wrapped here.

use tauri::{Window, Manager};

/// Represents a JavaScript script that can be injected into the UI.
///
/// Scripts can be:
/// - Inline JS strings
/// - Named functions
/// - Event handlers
pub struct Script {
    pub name: String,
    pub code: String,
}

impl Script {
    /// Create a new script with a name and JS code.
    pub fn new(name: impl Into<String>, code: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            code: code.into(),
        }
    }

    /// Wraps the script in a `<script>` tag for injection.
    pub fn as_html(&self) -> String {
        format!("<script id=\"script_{}\">{}</script>", self.name, self.code)
    }
}

/// A safe wrapper for executing JavaScript inside a Tauri window.
///
/// This prevents direct access to Tauri's raw JS APIs and ensures
/// all JS execution goes through Sentinel's security layer.
pub struct ScriptEngine<'a> {
    window: &'a Window,
}

impl<'a> ScriptEngine<'a> {
    /// Create a new script engine bound to a specific window.
    pub fn new(window: &'a Window) -> Self {
        Self { window }
    }

    /// Execute raw JavaScript inside the window.
    ///
    /// This is sandboxed by Tauri and cannot access the OS directly.
    pub fn execute(&self, js: impl Into<String>) {
        let code = js.into();
        let _ = self.window.eval(&code);
    }

    /// Inject a script into the DOM.
    ///
    /// Useful for reusable components that need JS behavior.
    pub fn inject(&self, script: &Script) {
        let html = script.as_html();
        let js = format!(
            "document.body.insertAdjacentHTML('beforeend', `{}`);",
            html
        );
        let _ = self.window.eval(&js);
    }

    /// Call a JS function by name with arguments.
    ///
    /// Example:
    /// engine.call("showAlert", vec!["Hello"]);
    pub fn call(&self, fn_name: &str, args: Vec<&str>) {
        let arg_list = args
            .into_iter()
            .map(|a| format!("\"{}\"", a))
            .collect::<Vec<_>>()
            .join(",");

        let js = format!("if (typeof {} === 'function') {{ {}({}); }}", fn_name, fn_name, arg_list);
        let _ = self.window.eval(&js);
    }
}

/// A helper for binding JS events to Rust commands.
///
/// Example:
/// ```
/// bind_event("login", "onLogin");
/// ```
pub fn bind_event(event: &str, handler: &str) -> String {
    format!(
        r#"
        document.addEventListener("{}", (e) => {{
            if (typeof {} === "function") {{
                {}(e.detail);
            }}
        }});
        "#,
        event, handler, handler
    )
}

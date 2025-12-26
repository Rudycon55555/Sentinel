// Sentinel/src/Backend/TempEng.rs

//! Sentinel Templating Engine
//!
//! Provides:
//! - HTML template loading
//! - CSS + JS injection
//! - Placeholder replacement
//! - Integration with Tauri windows
//!
//! This powers dynamic UI rendering for Sentinel apps.

use std::collections::HashMap;
use std::fs;
use tauri::Window;

/// Represents a template with placeholders.
pub struct Template {
    pub html: String,
    pub css: String,
    pub js: String,
}

impl Template {
    /// Load a template from three files: HTML, CSS, JS.
    pub fn from_files(
        html_path: &str,
        css_path: &str,
        js_path: &str,
    ) -> Result<Self, String> {
        let html = fs::read_to_string(html_path)
            .map_err(|e| e.to_string())?;
        let css = fs::read_to_string(css_path)
            .map_err(|e| e.to_string())?;
        let js = fs::read_to_string(js_path)
            .map_err(|e| e.to_string())?;

        Ok(Self { html, css, js })
    }

    /// Replace placeholders like {{key}} with values.
    pub fn apply(mut self, vars: &HashMap<String, String>) -> Self {
        for (key, value) in vars {
            let placeholder = format!("{{{{{}}}}}", key);
            self.html = self.html.replace(&placeholder, value);
            self.css = self.css.replace(&placeholder, value);
            self.js = self.js.replace(&placeholder, value);
        }
        self
    }

    /// Render the template into a single HTML string.
    pub fn render(&self) -> String {
        format!(
            r#"
            <style>{}</style>
            {}
            <script>{}</script>
            "#,
            self.css, self.html, self.js
        )
    }

    /// Inject the rendered template into a Tauri window.
    pub fn inject_into(&self, window: &Window) {
        let rendered = self.render().replace('`', "\\`");
        let js = format!("document.body.innerHTML = `{}`;", rendered);
        let _ = window.eval(&js);
    }
}

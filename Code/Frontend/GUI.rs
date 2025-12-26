// Sentinel/src/Frontend/GUI.rs

//! Sentinel GUI Core
//!
//! This module defines the foundational GUI abstractions used by all
//! Sentinel components. It provides:
//! - A `Renderable` trait for any UI element
//! - A `GUIComponent` struct for reusable, customizable widgets
//! - Support for custom HTML and CSS injection
//! - A unified rendering pipeline for Tauri-based UIs
//!
//! JavaScript integration is handled separately in `Scripting.rs`.

use std::collections::HashMap;

/// Trait implemented by all GUI elements.
///
/// Anything that can appear in the UI must implement `render()`,
/// which returns HTML as a string.
pub trait Renderable {
    fn render(&self) -> String;
}

/// Represents a reusable GUI component.
///
/// Components can:
/// - Have an ID
/// - Contain HTML templates
/// - Inject CSS styles
/// - Accept dynamic properties
pub struct GUIComponent {
    pub id: String,
    pub html: String,
    pub css: String,
    pub props: HashMap<String, String>,
}

impl GUIComponent {
    /// Create a new empty component.
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            html: String::new(),
            css: String::new(),
            props: HashMap::new(),
        }
    }

    /// Set the HTML template for this component.
    pub fn with_html(mut self, html: impl Into<String>) -> Self {
        self.html = html.into();
        self
    }

    /// Set the CSS for this component.
    pub fn with_css(mut self, css: impl Into<String>) -> Self {
        self.css = css.into();
        self
    }

    /// Add a property (e.g., title="Dashboard").
    pub fn with_prop(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.props.insert(key.into(), value.into());
        self
    }

    /// Render the component with props injected.
    fn render_with_props(&self) -> String {
        let mut rendered = self.html.clone();

        for (key, value) in &self.props {
            let placeholder = format!("{{{{{}}}}}", key);
            rendered = rendered.replace(&placeholder, value);
        }

        rendered
    }
}

impl Renderable for GUIComponent {
    fn render(&self) -> String {
        let html = self.render_with_props();

        if self.css.is_empty() {
            html
        } else {
            format!(
                "<style id=\"{}_style\">{}</style>\n{}",
                self.id, self.css, html
            )
        }
    }
}

/// A container for multiple components.
///
/// Useful for panels, dashboards, or grouped UI.
pub struct GUIContainer {
    pub children: Vec<Box<dyn Renderable>>,
}

impl GUIContainer {
    pub fn new() -> Self {
        Self { children: vec![] }
    }

    pub fn add(mut self, component: impl Renderable + 'static) -> Self {
        self.children.push(Box::new(component));
        self
    }
}

impl Renderable for GUIContainer {
    fn render(&self) -> String {
        self.children
            .iter()
            .map(|c| c.render())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

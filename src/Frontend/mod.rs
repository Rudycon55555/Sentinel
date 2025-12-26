// Sentinel/src/Frontend/mod.rs

//! Sentinel Frontend Module
//!
//! This module provides high‑level abstractions for building
//! secure, multi‑panel GUIs using Tauri under the hood.
//!
//! Structure:
//! - app.rs          → App builder + entrypoint
//! - window.rs       → Window helpers
//! - events.rs       → Event system
//! - components/     → Reusable UI components
//! - tauri_bridge/   → Safe wrappers around Tauri APIs

pub mod app;
pub mod window;
pub mod events;
pub mod components;
pub mod tauri_bridge;

// Re‑exports for a cleaner public API
pub use app::App;
pub use window::WindowOptions;
pub use events::{on, emit};

// Sentinel/src/Backend/mod.rs

//! Sentinel Backend Module
//!
//! Provides authentication, data handling, and backend logic
//! for system‑level applications.

pub mod Auth;
pub mod DataHandler;

// Re‑exports for cleaner API
pub use Auth::*;
pub use DataHandler::*;

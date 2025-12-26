// Sentinel/src/Middlend/Extensions.rs

//! Sentinel Extensions System
//!
//! Allows developers to create "extensions" — special functions
//! that wrap or modify the behavior of other functions.
//!
//! Extensions can:
//! - Run before a function
//! - Run after a function
//! - Modify inputs or outputs
//! - Block execution
//! - Log, transform, validate, etc.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Context passed into extensions.
#[derive(Clone)]
pub struct ExtensionContext {
    pub name: String,
    pub metadata: HashMap<String, String>,
}

/// Type alias for an extension function.
///
/// The extension receives:
/// - A context
/// - A "next" function to call the original logic
pub type ExtensionFn = Arc<
    dyn Fn(
        ExtensionContext,
        Arc<dyn Fn(ExtensionContext) -> ExtensionResult + Send + Sync>,
    ) -> ExtensionResult
        + Send
        + Sync,
>;

/// Result type for extensions.
pub type ExtensionResult = Result<String, String>;

/// Registry of all extensions.
pub struct Extensions {
    registry: Arc<Mutex<HashMap<String, ExtensionFn>>>,
}

impl Extensions {
    /// Create an empty extension registry.
    pub fn new() -> Self {
        Self {
            registry: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Register a new extension.
    pub fn register(
        &self,
        name: impl Into<String>,
        func: ExtensionFn,
    ) {
        let mut reg = self.registry.lock().unwrap();
        reg.insert(name.into(), func);
    }

    /// Call an extension by name.
    pub fn call(
        &self,
        name: &str,
        ctx: ExtensionContext,
        original: impl Fn(ExtensionContext) -> ExtensionResult + Send + Sync + 'static,
    ) -> ExtensionResult {
        let reg = self.registry.lock().unwrap();

        if let Some(ext) = reg.get(name) {
            let next = Arc::new(move |c: ExtensionContext| original(c));
            ext(ctx, next)
        } else {
            Err(format!("Extension '{}' not found", name))
        }
    }
}

// Sentinel/src/Backend/Roles.rs

//! Sentinel Role-Based View Resolver
//!
//! Provides:
//! - Role definitions
//! - Mapping roles to UI views/components
//! - Resolving which view a user should see
//!
//! This integrates with the Frontend GUI system.

use crate::Backend::Auth::User;
use crate::Frontend::GUI::{Renderable, GUIContainer};

use std::collections::HashMap;

/// Represents a role → view mapping.
pub struct RoleViews {
    pub map: HashMap<String, GUIContainer>,
}

impl RoleViews {
    /// Create an empty role-view map.
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    /// Assign a GUIContainer to a role.
    pub fn register_role_view(
        mut self,
        role: impl Into<String>,
        view: GUIContainer,
    ) -> Self {
        self.map.insert(role.into(), view);
        self
    }

    /// Resolve the correct view for a user.
    pub fn resolve(&self, user: &User) -> Option<&GUIContainer> {
        self.map.get(&user.role)
    }

    /// Resolve with fallback.
    pub fn resolve_or_default(
        &self,
        user: &User,
        default: &GUIContainer,
    ) -> &GUIContainer {
        self.map.get(&user.role).unwrap_or(default)
    }
}

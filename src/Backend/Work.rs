// Sentinel/src/Backend/Work.rs

//! Sentinel Backend Work Module
//!
//! This module ties together all backend systems:
//! - Authentication
//! - Cryptography
//! - Data handling (JSON/YAML/XML/SQL)
//! - Role-based view resolution
//! - Templating engine
//!
//! It provides a unified API for backend operations,
//! similar to how Frontend/App.rs orchestrates the UI layer.

use crate::Backend::{
    Auth::{self, User},
    Cryptography,
    DataHandler::{CRUD, JSONHandler, YAMLHandler, XMLHandler, SQLHandler},
    Roles::RoleViews,
    TempEng::Template,
};

use std::collections::HashMap;

/// The main backend orchestrator.
///
/// Developers use this to configure backend behavior,
/// load users, resolve views, and perform secure operations.
pub struct Work {
    pub roles: RoleViews,
}

impl Work {
    /// Create a new backend orchestrator.
    pub fn new() -> Self {
        Self {
            roles: RoleViews::new(),
        }
    }

    /// Register a role → view mapping.
    pub fn register_role_view(
        mut self,
        role: impl Into<String>,
        view: crate::Frontend::GUI::GUIContainer,
    ) -> Self {
        self.roles = self.roles.register_role_view(role, view);
        self
    }

    // -------------------------
    // AUTHENTICATION OPERATIONS
    // -------------------------

    /// Register a new user using chosen hashing algorithm.
    pub fn register_user(
        &self,
        username: &str,
        password: &str,
        role: &str,
        salt: &str,
        use_argon2: bool,
    ) -> Result<User, String> {
        Auth::register_user(username, password, role, salt, use_argon2)
    }

    /// Attempt login and return session token.
    pub fn login_user(
        &self,
        user: &User,
        password: &str,
        salt: &str,
        use_argon2: bool,
    ) -> Option<String> {
        Auth::login_user(user, password, salt, use_argon2)
    }

    /// Validate a session token.
    pub fn validate_session(&self, token: &str) -> bool {
        Auth::validate_session(token)
    }

    /// Check if a user has a required role.
    pub fn user_has_role(&self, user: &User, role: &str) -> bool {
        Auth::user_has_role(user, role)
    }

    // -------------------------
    // DATA HANDLING OPERATIONS
    // -------------------------

    pub fn save_json<T>(&self, data: &T) -> Result<String, String>
    where
        T: serde::Serialize + for<'de> serde::Deserialize<'de>,
    {
        JSONHandler.create(data)
    }

    pub fn load_json<T>(&self, source: &str) -> Result<T, String>
    where
        T: serde::Serialize + for<'de> serde::Deserialize<'de>,
    {
        JSONHandler.read(source)
    }

    pub fn save_yaml<T>(&self, data: &T) -> Result<String, String>
    where
        T: serde::Serialize + for<'de> serde::Deserialize<'de>,
    {
        YAMLHandler.create(data)
    }

    pub fn load_yaml<T>(&self, source: &str) -> Result<T, String>
    where
        T: serde::Serialize + for<'de> serde::Deserialize<'de>,
    {
        YAMLHandler.read(source)
    }

    pub fn save_xml<T>(&self, data: &T) -> Result<String, String>
    where
        T: serde::Serialize + for<'de> serde::Deserialize<'de>,
    {
        XMLHandler.create(data)
    }

    pub fn load_xml<T>(&self, source: &str) -> Result<T, String>
    where
        T: serde::Serialize + for<'de> serde::Deserialize<'de>,
    {
        XMLHandler.read(source)
    }

    pub fn sql_query(&self, query: &str) -> Result<String, String> {
        SQLHandler.query(query)
    }

    // -------------------------
    // CRYPTOGRAPHY OPERATIONS
    // -------------------------

    pub fn hash_argon2id(&self, password: &str, salt: &str) -> Result<String, String> {
        Cryptography::hash_argon2id(password, salt)
    }

    pub fn hash_sha256(&self, input: &str) -> String {
        Cryptography::hash_sha256(input)
    }

    pub fn encrypt_aes256(
        &self,
        key: &[u8; 32],
        nonce: &[u8; 12],
        plaintext: &str,
    ) -> Result<Vec<u8>, String> {
        Cryptography::encrypt_aes256(key, nonce, plaintext)
    }

    pub fn decrypt_aes256(
        &self,
        key: &[u8; 32],
        nonce: &[u8; 12],
        ciphertext: &[u8],
    ) -> Result<String, String> {
        Cryptography::decrypt_aes256(key, nonce, ciphertext)
    }

    // -------------------------
    // TEMPLATE ENGINE OPERATIONS
    // -------------------------

    pub fn load_template(
        &self,
        html: &str,
        css: &str,
        js: &str,
    ) -> Result<Template, String> {
        Template::from_files(html, css, js)
    }

    pub fn apply_template(
        &self,
        template: Template,
        vars: &HashMap<String, String>,
    ) -> Template {
        template.apply(vars)
    }

    // -------------------------
    // ROLE-BASED VIEW RESOLUTION
    // -------------------------

    pub fn resolve_view(
        &self,
        user: &User,
        fallback: &crate::Frontend::GUI::GUIContainer,
    ) -> crate::Frontend::GUI::GUIContainer {
        self.roles
            .resolve(user)
            .cloned()
            .unwrap_or_else(|| fallback.clone())
    }
}

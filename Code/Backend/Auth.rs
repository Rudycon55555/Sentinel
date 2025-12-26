// Sentinel/src/Backend/Auth.rs

//! Sentinel Authentication Module
//!
//! Provides:
//! - User registration
//! - Login verification
//! - Session management
//! - Role checking
//! - Integration with DataHandler + Cryptography

use crate::Backend::Cryptography::*;
use crate::Backend::DataHandler::{CRUD, JSONHandler};

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Represents a user in the system.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password_hash: String,
    pub role: String,
}

/// Session store (in-memory for now).
static mut SESSIONS: Option<HashMap<String, String>> = None;

/// Initialize session store.
fn ensure_sessions() {
    unsafe {
        if SESSIONS.is_none() {
            SESSIONS = Some(HashMap::new());
        }
    }
}

/// Generate a random session token.
pub fn generate_session_token() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(48)
        .map(char::from)
        .collect()
}

/// Register a new user using chosen hashing algorithm.
pub fn register_user(
    username: &str,
    password: &str,
    role: &str,
    salt: &str,
    use_argon2: bool,
) -> Result<User, String> {
    let password_hash = if use_argon2 {
        hash_argon2id(password, salt)?
    } else {
        hash_sha256(password)
    };

    Ok(User {
        username: username.into(),
        password_hash,
        role: role.into(),
    })
}

/// Attempt login and return session token.
pub fn login_user(
    user: &User,
    password: &str,
    salt: &str,
    use_argon2: bool,
) -> Option<String> {
    let valid = if use_argon2 {
        verify_argon2id(password, &user.password_hash)
    } else {
        hash_sha256(password) == user.password_hash
    };

    if valid {
        let token = generate_session_token();
        unsafe {
            ensure_sessions();
            if let Some(sessions) = &mut SESSIONS {
                sessions.insert(token.clone(), user.username.clone());
            }
        }
        Some(token)
    } else {
        None
    }
}

/// Check if a session token is valid.
pub fn validate_session(token: &str) -> bool {
    unsafe {
        ensure_sessions();
        if let Some(sessions) = &SESSIONS {
            sessions.contains_key(token)
        } else {
            false
        }
    }
}

/// Check if a user has a required role.
pub fn user_has_role(user: &User, required: &str) -> bool {
    user.role == required
}

/// Save a user to JSON using DataHandler.
pub fn save_user_json(user: &User) -> Result<String, String> {
    let handler = JSONHandler;
    handler.create(user)
}

/// Load a user from JSON using DataHandler.
pub fn load_user_json(json: &str) -> Result<User, String> {
    let handler = JSONHandler;
    handler.read(json)
}

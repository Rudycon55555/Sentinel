// Sentinel/src/Middlend/Checker.rs

//! Sentinel Runtime Checker
//!
//! Performs runtime checks for:
//! - Security
//! - File integrity
//! - Template integrity
//! - API integrity
//! - Session validity
//! - Role validity

use crate::Middlend::Security::*;
use crate::Backend::Auth::User;
use crate::Backend::Auth;

pub struct Checker;

impl Checker {
    /// Check if a session is valid.
    pub fn check_session(token: &str) -> Result<(), String> {
        enforce_session(token)
    }

    /// Check if a user has a required role.
    pub fn check_role(user: &User, role: &str) -> Result<(), String> {
        enforce_role(user, role)
    }

    /// Check if input is safe.
    pub fn check_input(input: &str) -> Result<String, String> {
        validate_input(input)
    }

    /// Check if a path is safe.
    pub fn check_path(path: &str) -> Result<String, String> {
        safe_path(path)
    }

    /// Check if a route + payload looks suspicious.
    pub fn check_intrusion(route: &str, payload: &str) -> bool {
        detect_intrusion(route, payload)
    }
}

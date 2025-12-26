// Sentinel/src/Middlend/Security.rs

//! Sentinel Security Module
//!
//! Provides:
//! - Input validation
//! - Sanitization
//! - Rate limiting
//! - Session enforcement
//! - Role enforcement
//! - Path safety checks
//! - Basic intrusion detection
//!
//! This is the core security layer used by Backend, Frontend, and Middlend.

use crate::Backend::Auth;
use crate::Backend::Auth::User;
use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};

/// ------------------------------
/// INPUT VALIDATION + SANITIZATION
/// ------------------------------

/// Remove dangerous characters from user input.
pub fn sanitize(input: &str) -> String {
    input
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&#39;")
        .replace("`", "&#96;")
}

/// Check if input contains suspicious patterns.
pub fn is_malicious(input: &str) -> bool {
    let lowered = input.to_lowercase();

    lowered.contains("script")
        || lowered.contains("onerror")
        || lowered.contains("onload")
        || lowered.contains("drop table")
        || lowered.contains("delete from")
        || lowered.contains("--")
        || lowered.contains("/*")
}

/// Validate input and return sanitized version.
pub fn validate_input(input: &str) -> Result<String, String> {
    if is_malicious(input) {
        Err("Potentially malicious input detected".into())
    } else {
        Ok(sanitize(input))
    }
}

/// ------------------------------
/// SESSION + ROLE ENFORCEMENT
/// ------------------------------

/// Ensure a session token is valid.
pub fn enforce_session(token: &str) -> Result<(), String> {
    if Auth::validate_session(token) {
        Ok(())
    } else {
        Err("Invalid or expired session token".into())
    }
}

/// Ensure a user has the required role.
pub fn enforce_role(user: &User, required: &str) -> Result<(), String> {
    if Auth::user_has_role(user, required) {
        Ok(())
    } else {
        Err("User does not have required role".into())
    }
}

/// ------------------------------
/// PATH SAFETY
/// ------------------------------

/// Prevent directory traversal attacks.
pub fn safe_path(path: &str) -> Result<String, String> {
    if path.contains("..") || path.contains("//") || path.contains("\\\\") {
        Err("Unsafe path detected".into())
    } else {
        Ok(path.into())
    }
}

/// ------------------------------
/// RATE LIMITING
/// ------------------------------

/// Simple in-memory rate limiter.
pub struct RateLimiter {
    pub max_requests: usize,
    pub window: Duration,
    pub requests: HashMap<String, VecDeque<Instant>>,
}

impl RateLimiter {
    pub fn new(max_requests: usize, window_seconds: u64) -> Self {
        Self {
            max_requests,
            window: Duration::from_secs(window_seconds),
            requests: HashMap::new(),
        }
    }

    /// Check if a client is allowed to proceed.
    pub fn allow(&mut self, client_id: &str) -> bool {
        let now = Instant::now();
        let entry = self.requests.entry(client_id.into()).or_default();

        // Remove old timestamps
        while let Some(&front) = entry.front() {
            if now.duration_since(front) > self.window {
                entry.pop_front();
            } else {
                break;
            }
        }

        if entry.len() < self.max_requests {
            entry.push_back(now);
            true
        } else {
            false
        }
    }
}

/// ------------------------------
/// BASIC INTRUSION DETECTION
/// ------------------------------

/// Detect suspicious API usage patterns.
pub fn detect_intrusion(route: &str, payload: &str) -> bool {
    let suspicious_routes = ["admin", "root", "system"];
    let suspicious_payload = ["DROP", "DELETE", "INSERT", "EXEC"];

    suspicious_routes.iter().any(|r| route.contains(r))
        && suspicious_payload.iter().any(|p| payload.to_uppercase().contains(p))
}

/// ------------------------------
/// SECURITY POLICY
/// ------------------------------

#[derive(Clone)]
pub struct SecurityPolicy {
    pub require_session: bool,
    pub require_role: Option<String>,
    pub sanitize_inputs: bool,
    pub rate_limit: Option<(usize, u64)>, // (max_requests, window_seconds)
}

impl SecurityPolicy {
    pub fn default() -> Self {
        Self {
            require_session: true,
            require_role: None,
            sanitize_inputs: true,
            rate_limit: None,
        }
    }
}

// Sentinel/src/Middlend/Middleware.rs

//! Sentinel Middleware System
//!
//! Provides a pipeline for:
//! - Security policy enforcement
//! - Input validation
//! - Session checks
//! - Role checks
//! - Rate limiting
//!
//! This is used by Backend routing and API handlers.

use crate::Middlend::Security::*;
use crate::Backend::Auth::User;

pub struct MiddlewareContext<'a> {
    pub session_token: Option<&'a str>,
    pub user: Option<&'a User>,
    pub input: Option<&'a str>,
    pub client_id: &'a str,
}

pub struct Middleware {
    pub policy: SecurityPolicy,
    pub rate_limiter: Option<RateLimiter>,
}

impl Middleware {
    pub fn new(policy: SecurityPolicy) -> Self {
        let rate_limiter = policy.rate_limit.map(|(max, window)| {
            RateLimiter::new(max, window)
        });

        Self {
            policy,
            rate_limiter,
        }
    }

    /// Run the middleware pipeline.
    pub fn run(&mut self, ctx: &MiddlewareContext) -> Result<(), String> {
        // Rate limiting
        if let Some(limiter) = &mut self.rate_limiter {
            if !limiter.allow(ctx.client_id) {
                return Err("Rate limit exceeded".into());
            }
        }

        // Session enforcement
        if self.policy.require_session {
            if let Some(token) = ctx.session_token {
                enforce_session(token)?;
            } else {
                return Err("Session token required".into());
            }
        }

        // Role enforcement
        if let Some(required_role) = &self.policy.require_role {
            if let Some(user) = ctx.user {
                enforce_role(user, required_role)?;
            } else {
                return Err("User required for role enforcement".into());
            }
        }

        // Input sanitization
        if self.policy.sanitize_inputs {
            if let Some(input) = ctx.input {
                validate_input(input)?;
            }
        }

        Ok(())
    }
}

// Sentinel/src/Middlend/Runtime.rs

//! Sentinel Runtime Engine
//!
//! The central execution engine that:
//! - Loads project structure
//! - Validates folders + templates
//! - Loads identity metadata
//! - Runs middleware
//! - Executes extensions
//! - Connects Backend + Frontend
//!
//! This is the heart of Sentinel's Middlend layer.

use crate::Middlend::{
    NeededStruct::ProjectStructure,
    Loader::Loader,
    Validator::Validator,
    Identity::Identity,
    Middleware::{Middleware, MiddlewareContext},
    Extensions::{Extensions, ExtensionContext},
};
use crate::Backend::Work::Work;
use crate::Backend::Auth::User;

pub struct Runtime {
    pub structure: ProjectStructure,
    pub loader: Loader,
    pub validator: Validator,
    pub identity: Option<Identity>,
    pub backend: Work,
    pub extensions: Extensions,
}

impl Runtime {
    /// Create a new runtime from a project root.
    pub fn new(root: impl Into<std::path::PathBuf>) -> Self {
        let structure = ProjectStructure::new(root);
        let loader = Loader::new(structure.root.clone());
        let validator = Validator::new(structure.clone());

        Self {
            structure,
            loader,
            validator,
            identity: None,
            backend: Work::new(),
            extensions: Extensions::new(),
        }
    }

    /// Load identity metadata.
    pub fn load_identity(&mut self) -> Result<(), String> {
        let id = Identity::load(&self.loader)?;
        self.identity = Some(id);
        Ok(())
    }

    /// Validate the project structure.
    pub fn validate(&self) -> Result<(), String> {
        self.validator.validate_folders()?;
        self.validator.validate_index()?;
        self.validator.validate_templates()?;
        Ok(())
    }

    /// Run middleware for an operation.
    pub fn run_middleware(
        &self,
        middleware: &mut Middleware,
        session: Option<&str>,
        user: Option<&User>,
        input: Option<&str>,
        client_id: &str,
    ) -> Result<(), String> {
        let ctx = MiddlewareContext {
            session_token: session,
            user,
            input,
            client_id,
        };

        middleware.run(&ctx)
    }

    /// Run an extension.
    pub fn run_extension(
        &self,
        name: &str,
        metadata: std::collections::HashMap<String, String>,
        original: impl Fn(ExtensionContext) -> Result<String, String> + Send + Sync + 'static,
    ) -> Result<String, String> {
        let ctx = ExtensionContext {
            name: name.into(),
            metadata,
        };

        self.extensions.call(name, ctx, original)
    }
}

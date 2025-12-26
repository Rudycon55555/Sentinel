// Sentinel/src/Middlend/Identity.rs

//! Sentinel Project Identity
//!
//! Extracts metadata from Describe/ folder:
//! - name
//! - version
//! - author
//! - description
//! - custom fields

use crate::Middlend::Loader::Loader;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Identity {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, String>,
}

impl Identity {
    pub fn load(loader: &Loader) -> Result<Self, String> {
        let json = loader.load_config("identity.json")?;
        serde_json::from_str(&json).map_err(|e| e.to_string())
    }
}

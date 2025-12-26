// Sentinel/src/Middlend/NeededStruct.rs

//! Sentinel Project Structure Definitions
//!
//! This module defines the structure of a Sentinel application.
//! It describes where developers place:
//! - Describe/  → configs, metadata, about files
//! - Pages/     → windows, screens, UI pages
//! - Pages/Index.rs → main entry page
//! - APIs/      → API endpoints
//! - Work/      → backend logic for their system
//!
//! The rest of Sentinel uses these structures to load and validate
//! the developer's project layout.

use std::path::PathBuf;

/// Represents the Describe/ folder.
/// Contains configuration files, metadata, and "about" information.
#[derive(Debug, Clone)]
pub struct DescribeFolder {
    pub path: PathBuf,
}

/// Represents the Pages/ folder.
/// Contains all UI pages, windows, and screens.
#[derive(Debug, Clone)]
pub struct PagesFolder {
    pub path: PathBuf,
    pub index_page: PathBuf,
}

/// Represents the APIs/ folder.
/// Contains API endpoint definitions.
#[derive(Debug, Clone)]
pub struct APIsFolder {
    pub path: PathBuf,
}

/// Represents the Work/ folder.
/// Contains backend logic for the developer's system.
#[derive(Debug, Clone)]
pub struct WorkFolder {
    pub path: PathBuf,
}

/// Represents the entire Sentinel project structure.
///
/// This is the core structure that Sentinel uses to understand
/// how a developer's project is organized.
#[derive(Debug, Clone)]
pub struct ProjectStructure {
    pub root: PathBuf,
    pub describe: DescribeFolder,
    pub pages: PagesFolder,
    pub apis: APIsFolder,
    pub work: WorkFolder,
}

impl ProjectStructure {
    /// Create a new project structure from a root directory.
    pub fn new(root: impl Into<PathBuf>) -> Self {
        let root = root.into();

        let describe = DescribeFolder {
            path: root.join("Describe"),
        };

        let pages = PagesFolder {
            path: root.join("Pages"),
            index_page: root.join("Pages").join("Index.rs"),
        };

        let apis = APIsFolder {
            path: root.join("APIs"),
        };

        let work = WorkFolder {
            path: root.join("Work"),
        };

        Self {
            root,
            describe,
            pages,
            apis,
            work,
        }
    }

    /// Validate that all required folders exist.
    pub fn validate(&self) -> Result<(), String> {
        if !self.describe.path.exists() {
            return Err("Missing Describe/ folder".into());
        }
        if !self.pages.path.exists() {
            return Err("Missing Pages/ folder".into());
        }
        if !self.pages.index_page.exists() {
            return Err("Missing Pages/Index.rs".into());
        }
        if !self.apis.path.exists() {
            return Err("Missing APIs/ folder".into());
        }
        if !self.work.path.exists() {
            return Err("Missing Work/ folder".into());
        }

        Ok(())
    }
}

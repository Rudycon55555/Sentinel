// Sentinel/src/Middlend/Loader.rs

//! Sentinel Loader
//!
//! Loads developer project structure:
//! - Describe/
//! - Pages/
//! - APIs/
//! - Work/
//!
//! Also loads templates and page modules.

use crate::Middlend::NeededStruct::ProjectStructure;
use crate::Backend::TempEng::Template;
use std::fs;

pub struct Loader {
    pub structure: ProjectStructure,
}

impl Loader {
    pub fn new(root: impl Into<std::path::PathBuf>) -> Self {
        Self {
            structure: ProjectStructure::new(root),
        }
    }

    /// Validate the structure before loading.
    pub fn validate(&self) -> Result<(), String> {
        self.structure.validate()
    }

    /// Load a template from Pages/.
    pub fn load_page_template(
        &self,
        name: &str,
    ) -> Result<Template, String> {
        let html = self.structure.pages.path.join(format!("{name}.html"));
        let css = self.structure.pages.path.join(format!("{name}.css"));
        let js = self.structure.pages.path.join(format!("{name}.js"));

        Template::from_files(
            html.to_str().unwrap(),
            css.to_str().unwrap(),
            js.to_str().unwrap(),
        )
    }

    /// Load a config file from Describe/.
    pub fn load_config(&self, name: &str) -> Result<String, String> {
        let path = self.structure.describe.path.join(name);
        fs::read_to_string(path).map_err(|e| e.to_string())
    }
}

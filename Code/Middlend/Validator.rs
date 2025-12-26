// Sentinel/src/Middlend/Validator.rs

//! Sentinel Project Validator
//!
//! Performs deep validation of:
//! - Required folders
//! - Required files
//! - Page templates
//! - API structure
//! - Index.rs correctness

use crate::Middlend::NeededStruct::ProjectStructure;
use std::fs;

pub struct Validator {
    pub structure: ProjectStructure,
}

impl Validator {
    pub fn new(structure: ProjectStructure) -> Self {
        Self { structure }
    }

    /// Validate folder structure.
    pub fn validate_folders(&self) -> Result<(), String> {
        self.structure.validate()
    }

    /// Validate that Index.rs exists and is not empty.
    pub fn validate_index(&self) -> Result<(), String> {
        let index = &self.structure.pages.index_page;

        if !index.exists() {
            return Err("Missing Pages/Index.rs".into());
        }

        let content = fs::read_to_string(index).map_err(|e| e.to_string())?;
        if content.trim().is_empty() {
            return Err("Pages/Index.rs is empty".into());
        }

        Ok(())
    }

    /// Validate that all templates have HTML/CSS/JS.
    pub fn validate_templates(&self) -> Result<(), String> {
        let pages = &self.structure.pages.path;

        for entry in fs::read_dir(pages).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();

            if let Some(ext) = path.extension() {
                if ext == "html" {
                    let name = path.file_stem().unwrap().to_str().unwrap();

                    let css = pages.join(format!("{name}.css"));
                    let js = pages.join(format!("{name}.js"));

                    if !css.exists() {
                        return Err(format!("Missing CSS for page {name}"));
                    }
                    if !js.exists() {
                        return Err(format!("Missing JS for page {name}"));
                    }
                }
            }
        }

        Ok(())
    }
}

// Sentinel/src/Backend/DataHandler.rs

//! Sentinel Data Handler
//!
//! Provides unified CRUD operations for:
//! - JSON
//! - YAML
//! - XML
//! - SQL (via connector abstraction)

use serde::{Serialize, Deserialize};
use serde_json;
use serde_yaml;
use quick_xml::de::from_str as xml_from_str;
use quick_xml::se::to_string as xml_to_string;

/// Generic CRUD trait for all data formats.
pub trait CRUD<T> {
    fn create(&self, data: &T) -> Result<String, String>;
    fn read(&self, source: &str) -> Result<T, String>;
    fn update(&self, source: &str, data: &T) -> Result<String, String>;
    fn delete(&self, source: &str) -> Result<String, String>;
}

/// JSON handler
pub struct JSONHandler;

impl<T> CRUD<T> for JSONHandler
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    fn create(&self, data: &T) -> Result<String, String> {
        serde_json::to_string_pretty(data).map_err(|e| e.to_string())
    }

    fn read(&self, source: &str) -> Result<T, String> {
        serde_json::from_str(source).map_err(|e| e.to_string())
    }

    fn update(&self, _source: &str, data: &T) -> Result<String, String> {
        self.create(data)
    }

    fn delete(&self, _source: &str) -> Result<String, String> {
        Ok("{}".into())
    }
}

/// YAML handler
pub struct YAMLHandler;

impl<T> CRUD<T> for YAMLHandler
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    fn create(&self, data: &T) -> Result<String, String> {
        serde_yaml::to_string(data).map_err(|e| e.to_string())
    }

    fn read(&self, source: &str) -> Result<T, String> {
        serde_yaml::from_str(source).map_err(|e| e.to_string())
    }

    fn update(&self, _source: &str, data: &T) -> Result<String, String> {
        self.create(data)
    }

    fn delete(&self, _source: &str) -> Result<String, String> {
        Ok("".into())
    }
}

/// XML handler
pub struct XMLHandler;

impl<T> CRUD<T> for XMLHandler
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    fn create(&self, data: &T) -> Result<String, String> {
        xml_to_string(data).map_err(|e| e.to_string())
    }

    fn read(&self, source: &str) -> Result<T, String> {
        xml_from_str(source).map_err(|e| e.to_string())
    }

    fn update(&self, _source: &str, data: &T) -> Result<String, String> {
        self.create(data)
    }

    fn delete(&self, _source: &str) -> Result<String, String> {
        Ok("<deleted/>".into())
    }
}

/// SQL handler (placeholder for real DB integration)
pub struct SQLHandler;

impl SQLHandler {
    pub fn query(&self, sql: &str) -> Result<String, String> {
        Ok(format!("Executed SQL: {}", sql))
    }
}

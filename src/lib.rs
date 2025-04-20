//! [![License: BSD-3-Clause](https://img.shields.io/badge/license-BSD--3--Clause-blue)](./LICENSE)
//! [![cdumay_error_yaml on crates.io](https://img.shields.io/crates/v/cdumay_error_yaml)](https://crates.io/crates/cdumay_error_yaml)
//! [![cdumay_error_yaml on docs.rs](https://docs.rs/cdumay_error_yaml/badge.svg)](https://docs.rs/cdumay_error_yaml)
//! [![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/cdumay/cdumay_error_yaml)
//!
//! Error wrapper for YAML serialization and deserialization.
//!
//! This crate provides structured error handling for operations involving
//! YAML encoding and decoding using `serde_yaml`. It converts raw
//! `serde_yaml::Error` values into application-friendly typed errors with
//! metadata support via `cdumay_error`.

use cdumay_error::{AsError, Error, ErrorConverter, define_errors, define_kinds};
use std::collections::BTreeMap;

/// Define custom error kinds related to YAML operations.
define_kinds! {
    YamlData = ("YAML-00001", 400, "Invalid YAML data")
}

/// Create typed error structs associated with each defined error kind.
define_errors! {
    DataError = YamlData,
}

/// Struct providing helper functions to convert `serde_yaml::Error`
/// into typed application errors.
pub struct YamlError;

impl ErrorConverter for YamlError {
    type Error = serde_yaml::Error;
    /// Converts a `serde_yaml::Error` into a structured application `Error`.
    ///
    /// # Parameters
    /// - `err`: The original `serde_yaml::Error` returned from a YAML operation.
    /// - `text`: Custom error message you wish to associate with the failure.
    /// - `context`: A context to enrich the error with metadata.
    ///
    /// # Returns
    /// A typed `Error` with metadata and details included.
    fn convert(err: &serde_yaml::Error, text: String, context: BTreeMap<String, serde_value::Value>) -> Error {
        DataError::new().set_message(text).set_details(context).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_value::Value;
    use std::collections::BTreeMap;

    #[test]
    fn test_invalid_yaml_returns_custom_error_with_message() {
        let invalid_yaml = "invalid: yaml: :"; // malformed input
        let parse_result = serde_yaml::from_str::<serde_yaml::Value>(invalid_yaml);
        assert!(parse_result.is_err());

        let err = parse_result.unwrap_err();
        let mut context = BTreeMap::new();
        context.insert("file".to_string(), Value::String("config.yaml".to_string()));

        let custom_error = YamlError::convert_error(&err, Some("Custom YAML parsing failed".to_string()), context.clone());

        assert_eq!(custom_error.kind.message_id(), "YAML-00001");
        assert_eq!(custom_error.message, "Custom YAML parsing failed");

        let details = custom_error.details.unwrap();
        assert!(details.contains_key("file"));
        assert!(details.contains_key("origin"));
    }

    #[test]
    fn test_invalid_yaml_returns_error_with_default_message() {
        let invalid_yaml = "---\ninvalid_yaml: [unterminated"; // bad structure
        let result = serde_yaml::from_str::<serde_yaml::Value>(invalid_yaml);
        assert!(result.is_err());

        let err = result.unwrap_err();
        let context = BTreeMap::new();

        let custom_error = YamlError::convert_error(&err, None, context.clone());

        assert_eq!(custom_error.kind.message_id(), "YAML-00001");
        assert_eq!(custom_error.message, err.to_string());

        let details = custom_error.details.unwrap();
        assert!(details.is_empty()); // no context added
    }

    #[test]
    fn test_valid_yaml_does_not_trigger_error() {
        let valid_yaml = "---\nkey: value";
        let result = serde_yaml::from_str::<serde_yaml::Value>(valid_yaml);
        assert!(result.is_ok());
    }
}

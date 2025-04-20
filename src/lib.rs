//! [![License: BSD-3-Clause](https://img.shields.io/badge/license-BSD--3--Clause-blue)](./LICENSE)
//! [![cdumay_error_yaml on crates.io](https://img.shields.io/crates/v/cdumay_error_yaml)](https://crates.io/crates/cdumay_error_yaml)
//! [![cdumay_error_yaml on docs.rs](https://docs.rs/cdumay_error_yaml/badge.svg)](https://docs.rs/cdumay_error_yaml)
//! [![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/cdumay/cdumay_error_yaml)
//!
use cdumay_error::{AsError, Error, define_errors, define_kinds};
use std::collections::BTreeMap;

define_kinds! {
    YamlData = ("YAML-00001", 400, "Invalid YAML data")
}

define_errors! {
    DataError = YamlData,
}

pub struct YamlError;

impl YamlError {
    pub fn yaml_error(err: &serde_yaml::Error, text: Option<String>, context: BTreeMap<String, serde_value::Value>) -> Error {
        match text {
            Some(data) => DataError::new().set_message(data).set_details({
                let mut ctx = context.clone();
                ctx.insert("origin".to_string(), serde_value::Value::String(err.to_string()));
                ctx
            }),
            None => DataError::new().set_message(err.to_string()).set_details(context),
        }
        .into()
    }
}

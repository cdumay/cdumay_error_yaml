# cdumay_error_yaml

[![License: BSD-3-Clause](https://img.shields.io/badge/license-BSD--3--Clause-blue)](./LICENSE)
[![cdumay_error_yaml on crates.io](https://img.shields.io/crates/v/cdumay_error_yaml)](https://crates.io/crates/cdumay_error_yaml)
[![cdumay_error_yaml on docs.rs](https://docs.rs/cdumay_error_yaml/badge.svg)](https://docs.rs/cdumay_error_yaml)
[![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/cdumay/cdumay_error_yaml)

Here's the documentation for your code in a `README.md` format:

A lightweight utility crate that converts YAML serialization and deserialization errors (`serde_yaml::Error`) into structured, typed errors using the [`cdumay_core`](https://!docs.rs/cdumay_core/) framework.

This helps standardize error handling for Rust applications that deal with YAML configuration or data files, while enriching error details with structured context.

### Features

- Converts YAML-related errors into a standardized error format
- Provides unique error codes, HTTP status codes, and descriptions
- Supports rich contextual error metadata via `BTreeMap`
- Integrates easily with the `cdumay_core::ErrorConverter` trait
- Provides a convenient `convert_result!` macro for error conversion

### Usage Example

#### Dependencies

```toml
[dependencies]
cdumay_core = "1.0"
serde = { version = "1.0", features = ["derive"] }
serde-value = "0.7"
serde_yaml = "0.8"
```

#### Code sample

Using the `YamlErrorConverter` directly:
```rust
use cdumay_core::ErrorConverter;
use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};
use cdumay_error_yaml::YamlErrorConverter;

#[derive(Serialize, Deserialize)]
struct Config {
    name: String,
    debug: bool,
}

fn serialize_config(config: &Config) -> Result<String, cdumay_core::Error> {
    serde_yaml::to_string(config).map_err(|e| {
        let mut ctx = BTreeMap::new();
        ctx.insert("config_name".into(), serde_value::Value::String(config.name.clone()));
        YamlErrorConverter::convert(&e, "Failed to serialize YAML config".into(), ctx)
    })
}

fn deserialize_config(input: &str) -> Result<Config, cdumay_core::Error> {
    serde_yaml::from_str::<Config>(input).map_err(|e| {
        let mut ctx = BTreeMap::new();
        ctx.insert("input".into(), serde_value::Value::String(input.to_string()));
        YamlErrorConverter::convert(&e, "Failed to deserialize YAML config".into(), ctx)
    })
}
```

### Example Output

```json
{
  "code": "YAML-00001",
  "status": 400,
  "kind": "Invalid YAML data",
  "message": "Failed to deserialize YAML config",
  "context": {
    "input": "invalid: yaml"
  }
}
```

Using the `convert_result!` macro:

```rust
use cdumay_core::ErrorConverter;
use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};
use cdumay_error_yaml::convert_result;

#[derive(Serialize, Deserialize)]
struct Config {
    name: String,
    debug: bool,
}

fn serialize_config(config: &Config) -> Result<String, cdumay_core::Error> {
    let mut ctx = BTreeMap::new();
    ctx.insert("config_name".into(), serde_value::Value::String(config.name.clone()));
    convert_result!(serde_yaml::to_string(config), ctx, "Failed to serialize YAML config")
}

fn deserialize_config(input: &str) -> Result<Config, cdumay_core::Error> {
    convert_result!(serde_yaml::from_str::<Config>(input))
}
```

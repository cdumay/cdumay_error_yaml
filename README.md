# cdumay_error_yaml

[![License: BSD-3-Clause](https://img.shields.io/badge/license-BSD--3--Clause-blue)](./LICENSE)
[![cdumay_error_yaml on crates.io](https://img.shields.io/crates/v/cdumay_error_yaml)](https://crates.io/crates/cdumay_error_yaml)
[![cdumay_error_yaml on docs.rs](https://docs.rs/cdumay_error_yaml/badge.svg)](https://docs.rs/cdumay_error_yaml)
[![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/cdumay/cdumay_error_yaml)

Error wrapper for YAML serialization and deserialization.

This crate provides structured error handling for operations involving
YAML encoding and decoding using `serde_yaml`. It converts raw
`serde_yaml::Error` values into application-friendly typed errors with
metadata support via `cdumay_error`.

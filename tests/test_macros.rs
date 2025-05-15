use cdumay_error::ErrorConverter;
use cdumay_error_yaml::convert_result;
use std::collections::BTreeMap;

#[test]
fn test_convert_result_with_context() {
    let result = serde_yaml::from_str::<serde_yaml::Value>("invalid: yaml: :");
    let mut context = BTreeMap::new();
    context.insert("test".to_string(), serde_value::Value::String("value".to_string()));

    let converted = convert_result!(result, context, "Test error");
    assert!(converted.is_err());

    let err = converted.unwrap_err();
    assert_eq!(err.kind.message_id(), "YAML-00001");
    assert!(err.message.contains("Test error"));
}

#[test]
fn test_convert_result_without_text() {
    let result = serde_yaml::from_str::<serde_yaml::Value>("invalid: yaml: :");
    let mut context = BTreeMap::new();
    context.insert("test".to_string(), serde_value::Value::String("value".to_string()));
    let converted = convert_result!(result, context);
    assert!(converted.is_err());

    let err = converted.unwrap_err();
    assert_eq!(err.kind.message_id(), "YAML-00001");
}

#[test]
fn test_convert_result_minimal() {
    let result = serde_yaml::from_str::<serde_yaml::Value>("invalid: yaml: :");
    let converted = convert_result!(result);
    assert!(converted.is_err());

    let err = converted.unwrap_err();
    assert_eq!(err.kind.message_id(), "YAML-00001");
}

#[test]
fn test_convert_result_success() {
    let result = serde_yaml::from_str::<serde_yaml::Value>("valid: yaml");
    let converted = convert_result!(result);
    assert!(converted.is_ok());
}

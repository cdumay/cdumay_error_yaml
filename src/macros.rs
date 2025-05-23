/// Macro to convert a [`Result<T, serde_yaml::Error>`] into a [`Result<T, cdumay_core::Error>`]
#[macro_export]
macro_rules! convert_result {
    ($result:expr, $context:expr, $text:expr) => {
        $result.map_err(|err| {
            cdumay_error_yaml::YamlErrorConverter::convert_error(&err, Some($text.to_string()), $context)
        })
    };
    ($result:expr, $context:expr) => {
        $result.map_err(|err| {
            cdumay_error_yaml::YamlErrorConverter::convert_error(&err, None, $context)
        })
    };
    ($result:expr) => {
        $result.map_err(|err| {
            cdumay_error_yaml::YamlErrorConverter::convert_error(&err, None, std::collections::BTreeMap::new())
        })
    };
}
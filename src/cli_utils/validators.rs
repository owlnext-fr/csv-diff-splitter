use std::path::Path;

use crate::cli_utils::error_codes;

/// Handles source file path validation.
/// 
/// It will check that the path exists, and the given path is a real file.
pub fn validate_source_file(source_path: &Path) {
    if !source_path.exists() || !source_path.is_file() {
        crate::crash!(format!("Cannot find source file {}", source_path.as_os_str().to_string_lossy()), error_codes::ERROR_SOURCE_FILE_NOT_FOUND);
    }

    info!("Source file location validated !");
}

/// Handles target file path validation.
/// 
/// It will check that the path exists, and the given path is a real file.
pub fn validate_target_file(target_path: &Path) {
    if !target_path.exists() || !target_path.is_file() {
        crate::crash!(format!("Cannot find source file {}", target_path.as_os_str().to_string_lossy()), error_codes::ERROR_TARGET_FILE_NOT_FOUND);
    }

    info!("Target file location validated !");
}

/// Handles config file path validation.
/// 
/// It will check that the path exists, and the given path is a real file.
pub fn validate_config_file(config_file: &Path) {
    if !config_file.exists() || !config_file.is_file() {
        crate::crash!(format!("Cannot find source file {}", config_file.as_os_str().to_string_lossy()), error_codes::ERROR_CONFIG_FILE_NOT_FOUND);
    }

    info!("Config file location validated !");
}
//! This module contains the configuration options for the application.
//! # Examples:
//! ```
//! use cli_utils::config::Logging;
//! let config = Logging::new();
//! ```
//!
/// This enum represents the different levels of logging that can be used in the application.
///
/// It derives `Debug` for printing and `PartialEq` for comparison in tests.
/// # Examples:
/// ```
/// use cli_utils::config::LogLevel;
/// let level = LogLevel::Info; // Represents an informational log level.
/// assert_eq!(format!("{:?}", level), "Info");
///
/// let another_level = LogLevel::Info;
/// assert_eq!(level, another_level);
///
/// let different_level = LogLevel::Debug;
/// assert_ne!(level, different_level);
/// ```
#[derive(Debug, PartialEq)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}
/// This enum represents the output destination for logs.
///
/// It can be standard output, standard error, or a file.
/// It derives `Debug` for printing and `PartialEq` for comparison in tests.
/// # Examples:
/// ```
/// use cli_utils::config::LogOutput;
/// let output = LogOutput::File("app.log".to_string());
///
/// // Using if let to check the variant and its value
/// if let LogOutput::File(path) = &output {
///     assert_eq!(path, "app.log");
/// } else {
///     panic!("Expected LogOutput::File");
/// }
///
/// assert_eq!(output, LogOutput::File("app.log".to_string()));
/// assert_ne!(output, LogOutput::Stdout);
/// ```
#[derive(Debug, PartialEq)]
pub enum LogOutput {
    Stdout,
    Stderr,
    File(String),
}

/// This struct contains configuration options for the application.
/// # Examples:
///
/// Creating a new default instance
/// ```
/// use cli_utils::config::Logging;
/// let config = Logging::new();
/// ```
///
/// Creating a new instance of the Logging struct:
/// ```
/// use cli_utils::config::{Logging, LogLevel, LogOutput};
/// let config = Logging{ enabled: true, level: LogLevel::Info, destination: LogOutput::Stdout };
/// ```
pub struct Logging {
    pub enabled: bool,
    pub level: LogLevel,
    pub destination: LogOutput,
}
                              
impl Logging {
    pub fn new() -> Self {
        Self {
            enabled: false,
            level: LogLevel::Info,
            destination: LogOutput::Stdout,
        }
    }
    // Getter for `enabled` field
    pub fn get_enabled(&self) -> bool {
        self.enabled
    }
    // Setter for `enabled` field
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    // Getter for `level` field
    pub fn get_level(&self) -> &LogLevel {
        &self.level
    }
    // Setter for `level` field
    pub fn set_level(&mut self, level: LogLevel) {
        self.level = level;
    }
    // Getter for `destination` field
    pub fn get_destination(&self) -> &LogOutput {
        &self.destination
    }
    // Setter for `destination` field
    pub fn set_destination(&mut self, destination: LogOutput) {
        self.destination = destination;
    }
}

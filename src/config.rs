//! this module contains the configuration for the application
/// this enum represents the possible log levels
/// # Examples
/// ```
/// use cli_utils::config::LogLevel;
/// let level = LogLevel::Debug;
/// ```
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

/// this enum represents the possible destinations for log output
/// # Examples
/// ```
/// use cli_utils::config::LogOutput;
/// let output = LogOutput::Stdout;
/// ```
pub enum LogOutput {
    Stdout,
    Stderr,
    File(String),
}

/// this struct contains the configuration for logging
/// # Examples
/// ```
/// use cli_utils::config::{ Logging, LogLevel, LogOutput };
/// let config = Logging::new( true, LogLevel::Debug, LogOutput::Stdout );
/// ```
/// Create a new 'Logging' instance with custom values
/// ```
/// use cli_utils::config::{ Logging, LogLevel, LogOutput };
/// let config = Logging {
///     enabled: true,
///     level: LogLevel::Debug,
///     destination: LogOutput::Stdout
/// };
/// ```
pub struct Logging {
    pub enabled: bool,
    pub level: LogLevel,
    pub destination: LogOutput,
}

impl Logging {
    pub fn new(enabled: bool, level: LogLevel, destination: LogOutput) -> Self {
        Self {
            enabled,
            level,
            destination,
        }
    }
}

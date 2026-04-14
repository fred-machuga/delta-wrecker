// **Compliance Note**
// This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.

//! # Logging utilities
//!
//! This module provides logging functionality for debugging and monitoring
//! the orbital mechanics calculations.

/// Log levels for controlling the verbosity of logging output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    /// Error messages - critical issues that prevent operation
    Error,
    /// Warning messages - potential issues that should be noted
    Warn,
    /// Info messages - general information about operation
    Info,
    /// Debug messages - detailed information for debugging
    Debug,
    /// Trace messages - very detailed information for tracing execution
    Trace,
}

/// A simple logger for debugging and monitoring.
#[derive(Debug, Clone)]
pub struct Logger {
    level: LogLevel,
}

impl Logger {
    /// Creates a new logger with the specified minimum log level.
    ///
    /// # Arguments
    ///
    /// * `level` - The minimum log level to output
    ///
    /// # Example
    ///
    /// ```
    /// use delta_wrecker::logging::{Logger, LogLevel};
    /// let logger = Logger::new(LogLevel::Info);
    /// ```
    pub fn new(level: LogLevel) -> Self {
        Logger { level }
    }

    /// Logs an error message.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to log
    pub fn error(&self, message: &str) {
        if self.level >= LogLevel::Error {
            eprintln!("[ERROR] {}", message);
        }
    }

    /// Logs a warning message.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to log
    pub fn warn(&self, message: &str) {
        if self.level >= LogLevel::Warn {
            eprintln!("[WARN] {}", message);
        }
    }

    /// Logs an info message.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to log
    pub fn info(&self, message: &str) {
        if self.level >= LogLevel::Info {
            println!("[INFO] {}", message);
        }
    }

    /// Logs a debug message.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to log
    pub fn debug(&self, message: &str) {
        if self.level >= LogLevel::Debug {
            println!("[DEBUG] {}", message);
        }
    }

    /// Logs a trace message.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to log
    pub fn trace(&self, message: &str) {
        if self.level >= LogLevel::Trace {
            println!("[TRACE] {}", message);
        }
    }
}

impl Default for Logger {
    fn default() -> Self {
        Logger::new(LogLevel::Info)
    }
}

// **Compliance Note**
// This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.
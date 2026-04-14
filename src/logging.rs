// **Compliance Note**
// This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.

//! # Logging Module
//!
//! Simple logging utilities for the Delta Wrecker library.

/// Log levels for different types of messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

/// A simple logger struct.
#[derive(Debug)]
pub struct Logger {
    level: LogLevel,
}

impl Logger {
    /// Creates a new logger with the specified minimum log level.
    pub fn new(level: LogLevel) -> Self {
        Self { level }
    }

    /// Logs a message at the specified level.
    pub fn log(&self, level: LogLevel, message: &str) {
        if level as u8 >= self.level as u8 {
            println!("[{:?}] {}", level, message);
        }
    }

    /// Logs a debug message.
    pub fn debug(&self, message: &str) {
        self.log(LogLevel::Debug, message);
    }

    /// Logs an info message.
    pub fn info(&self, message: &str) {
        self.log(LogLevel::Info, message);
    }

    /// Logs a warning message.
    pub fn warn(&self, message: &str) {
        self.log(LogLevel::Warn, message);
    }

    /// Logs an error message.
    pub fn error(&self, message: &str) {
        self.log(LogLevel::Error, message);
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new(LogLevel::Info)
    }
}

// **Compliance Note**
// This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.
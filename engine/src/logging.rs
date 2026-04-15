// **Compliance Note**
// This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.

//! # Logging utilities
//!
//! Simple, zero-dependency logger for debugging orbital calculations.

/// Log levels for controlling output verbosity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

/// A minimal logger for debugging and monitoring.
#[derive(Debug, Clone)]
pub struct Logger {
    level: LogLevel,
}

impl Logger {
    /// Creates a new logger with the specified minimum log level.
    pub fn new(level: LogLevel) -> Self {
        Logger { level }
    }

    pub fn error(&self, message: &str) {
        if self.level >= LogLevel::Error {
            eprintln!("[ERROR] {}", message);
        }
    }

    pub fn warn(&self, message: &str) {
        if self.level >= LogLevel::Warn {
            eprintln!("[WARN] {}", message);
        }
    }

    pub fn info(&self, message: &str) {
        if self.level >= LogLevel::Info {
            println!("[INFO] {}", message);
        }
    }

    pub fn debug(&self, message: &str) {
        if self.level >= LogLevel::Debug {
            println!("[DEBUG] {}", message);
        }
    }

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
// **Compliance Note**
// This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.

use delta_wrecker::logging::{Logger, LogLevel};

#[test]
fn test_logger_creation() {
    let logger = Logger::new(LogLevel::Debug);
    assert_eq!(logger.level, LogLevel::Debug);
}

#[test]
fn test_default_logger() {
    let logger = Logger::default();
    assert_eq!(logger.level, LogLevel::Info);
}

// Note: Testing println! output is tricky, so we skip functional logging tests
// In a real implementation, you'd use a testing framework that captures output

// **Compliance Note**
// This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.
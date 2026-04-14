// **Compliance Note**
// This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.

use delta_wrecker::logging::{Logger, LogLevel};

fn main() {
    let logger = Logger::new(LogLevel::Info);
    logger.info("Hello, Delta Wrecker!");
    logger.debug("This is a debug message");
    logger.warn("This is a warning");
}

// **Compliance Note**
// This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.
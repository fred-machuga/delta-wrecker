use delta_wrecker::logging::{Logger, LogLevel};

fn main() {
    let logger = Logger::new(LogLevel::Info);
    logger.info("Hello, Delta Wrecker!");
    logger.debug("This is a debug message");
    logger.warn("This is a warning");
}
const TRIGGER_FILE_SIZE: u64 = 10 * 1024 * 1024;
const LOG_FILE_COUNT: u32 = 5;
const FILE_PATH: &str = "./logs/stisty.log";
const ARCHIVE_PATTERN: &str = "./logs/archive/stisty.{}.log";

use log::{LevelFilter, SetLoggerError};
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        rolling_file::policy::compound::{
            roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger, CompoundPolicy,
        },
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};

pub fn setup_logger() -> Result<(), SetLoggerError> {
    let level = log::LevelFilter::Info;

    // Build a stderr logger.
    let stderr = ConsoleAppender::builder().target(Target::Stderr)
        .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)(local)} {l}: {m}{n}")))
        .build();

    // Create a policy to use with the file logging
    let trigger = SizeTrigger::new(TRIGGER_FILE_SIZE);
    let roller = FixedWindowRoller::builder()
        .build(ARCHIVE_PATTERN, LOG_FILE_COUNT)
        .unwrap();
    let policy = CompoundPolicy::new(Box::new(trigger), Box::new(roller));

    // Logging to log file. (with rolling)
    let logfile = log4rs::append::rolling_file::RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)(local)} {l}: {m}{n}")))
        .build(FILE_PATH, Box::new(policy))
        .unwrap();

    // Log Trace level output to file where trace is the default level
    // and the programmatically specified level to stderr.
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(level)))
                .build("stderr", Box::new(stderr)),
        )
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stderr")
                .build(LevelFilter::Trace),
        )
        .unwrap();

    log4rs::init_config(config)?;

    Ok(())
}

pub fn format_title(title: &str) -> String {
    let mut line_length = 50;
    if title.len() < 50 {
        line_length -= title.len();
    } else {
        return title.to_string();
    }
    let prefix = "=".repeat(line_length / 2);
    let suffix = "=".repeat(line_length / 2);
    let mut formatted_title = String::new();
    formatted_title.push_str(prefix.as_str());
    formatted_title.push_str(title);
    formatted_title.push_str(suffix.as_str());
    formatted_title
}
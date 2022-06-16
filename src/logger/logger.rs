use crate::logger;
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::config::{Appender, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::Config;

pub fn init_log() {
    let window_size = 3; // log0, log1, log2
    let fixed_window_roller = FixedWindowRoller::builder()
        .build("logs/app-{}.log", window_size)
        .unwrap();
    let size_limit = 100 * 1024 * 1024; // 100M as max log file size to roll
    let size_trigger = SizeTrigger::new(size_limit);
    let compound_policy =
        CompoundPolicy::new(Box::new(size_trigger), Box::new(fixed_window_roller));
    let rolling_file = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
        .build("logs/app.log", Box::new(compound_policy))
        .unwrap();

    let file_out = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
        .build("logs/app.log")
        .unwrap();
    let sys_file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
        .build("logs/sys.log")
        .unwrap();
    let business_file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
        .build("logs/business.log")
        .unwrap();

    let stdout = ConsoleAppender::builder().build();

    let config = Config::builder()
        .appender(Appender::builder().build("rolling_file", Box::new(rolling_file)))
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("file_out", Box::new(file_out)))
        .appender(Appender::builder().build("sys", Box::new(sys_file)))
        .appender(Appender::builder().build("business", Box::new(business_file)))
        .logger(
            Logger::builder()
                .appender("sys")
                .build("syslog", LevelFilter::Info),
        )
        .logger(
            Logger::builder()
                .appender("business")
                .build("businesslog", LevelFilter::Info),
        )
        .build(
            Root::builder()
                .appender("stdout")
                .appender("file_out")
                .build(LevelFilter::Info),
        )
        .unwrap();

    let _ = log4rs::init_config(config).unwrap();
}

use clap::Command;

pub fn new_use_log_cmd() -> Command<'static> {
    clap::Command::new("uselog")
        .about("use diffrent target log")
        .subcommand(new_use_sys_log_cmd())
        .subcommand(new_use_business_log_cmd())
}

pub fn new_use_sys_log_cmd() -> Command<'static> {
    clap::Command::new("syslog").about("append to syslog")
}

pub fn new_use_business_log_cmd() -> Command<'static> {
    clap::Command::new("businesslog").about("append to business log")
}

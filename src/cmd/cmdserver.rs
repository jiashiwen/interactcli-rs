use clap::{Arg, Command};

pub fn new_server_cmd() -> Command<'static> {
    clap::Command::new("server")
        .about("server")
        .subcommand(server_start_byfork())
        .subcommand(server_start_bydaemonize())
}

pub fn server_start_byfork() -> Command<'static> {
    clap::Command::new("byfork")
        .about("start daemon by fork crate")
        .arg(
            Arg::new("daemon")
                .short('d')
                .long("daemon")
                .help("start as daemon")
                .required(false),
        )
}
pub fn server_start_bydaemonize() -> Command<'static> {
    clap::Command::new("bydaemonize")
        .about("start daemon by daemonize crate")
        .arg(
            Arg::new("daemon")
                .short('d')
                .long("daemon")
                .help("start as daemon")
                .required(false),
        )
}

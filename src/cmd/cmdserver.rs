use clap::{Arg, ArgAction, Command};

pub fn new_server_cmd() -> Command {
    clap::Command::new("server")
        .about("server")
        .subcommand(server_start_byfork())
        .subcommand(server_start_bydaemonize())
}

pub fn server_start_byfork() -> Command {
    clap::Command::new("byfork")
        .about("start daemon by fork crate")
        .arg(
            Arg::new("daemon")
                .short('d')
                .long("daemon")
                .action(ArgAction::SetTrue)
                .help("start as daemon")
                .required(false),
        )
}
pub fn server_start_bydaemonize() -> Command {
    clap::Command::new("bydaemonize")
        .about("start daemon by daemonize crate")
        .arg(
            Arg::new("daemon")
                .short('d')
                .long("daemon")
                .action(ArgAction::SetTrue)
                .help("start as daemon")
                .required(false),
        )
}

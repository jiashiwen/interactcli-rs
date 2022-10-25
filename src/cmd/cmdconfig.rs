use clap::Command;

pub fn new_config_cmd() -> Command {
    clap::Command::new("config")
        .about("config")
        .subcommand(config_show_cmd())
}

fn config_show_cmd() -> Command {
    clap::Command::new("show")
        .about("show some info ")
        .subcommand(config_show_info_cmd())
        .subcommand(config_show_all_cmd())
}

fn config_show_info_cmd() -> Command {
    clap::Command::new("info").about("show info")
}

fn config_show_all_cmd() -> Command {
    clap::Command::new("all").about("show all ")
}

use clap::Command;

pub fn new_multi_cmd() -> Command<'static> {
    clap::Command::new("multi")
        .about("multi")
        .subcommand(config_level2_cmd1())
        .subcommand(config_level2_cmd2())
        .subcommand(abc_cmd())
}

fn config_level2_cmd1() -> Command<'static> {
    clap::Command::new("level2_cmd1")
        .about("level2_cmd1 info ")
        .subcommand(config_show_info_cmd())
}

fn abc_cmd() -> Command<'static> {
    clap::Command::new("abc_cmd").about("abc_cmd info")
}

fn config_level2_cmd2() -> Command<'static> {
    clap::Command::new("level2_cmd2")
        .about("level2_cmd2 info ")
        .subcommand(config_level3_cmd1())
        .subcommand(config_level3_cmd2())
        .subcommand(config_level3_cmd3())
}

fn config_level3_cmd1() -> Command<'static> {
    clap::Command::new("level3_cmd1")
        .about("level3_cmd1 info ")
        .subcommand(config_show_info_cmd())
        .subcommand(config_show_all_cmd())
}

fn config_level3_cmd2() -> Command<'static> {
    clap::Command::new("level3_cmd2")
        .about("level3_cmd2 info ")
        .subcommand(config_show_info_cmd())
        .subcommand(config_show_all_cmd())
}

fn config_level3_cmd3() -> Command<'static> {
    clap::Command::new("level3_cmd3")
        .about("level3_cmd3 info ")
        .subcommand(config_show_info_cmd())
        .subcommand(config_show_all_cmd())
}

fn config_show_info_cmd() -> Command<'static> {
    clap::Command::new("info").about("show info")
}

fn config_show_all_cmd() -> Command<'static> {
    clap::Command::new("all").about("show all ")
}

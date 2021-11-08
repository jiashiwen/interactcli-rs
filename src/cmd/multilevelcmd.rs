use clap::App;

pub fn new_multi_cmd() -> App<'static> {
    clap::App::new("multi")
        .about("multi")
        .subcommand(config_level2_cmd1())
        .subcommand(config_level2_cmd2())
        .subcommand(abc_cmd())
}

fn config_level2_cmd1() -> App<'static> {
    clap::App::new("level2_cmd1")
        .about("level2_cmd1 info ")
        .subcommand(config_show_info_cmd())
}

fn abc_cmd() -> App<'static> {
    clap::App::new("abc_cmd").about("abc_cmd info")
}

fn config_level2_cmd2() -> App<'static> {
    clap::App::new("level2_cmd2")
        .about("level2_cmd2 info ")
        .subcommand(config_level3_cmd1())
        .subcommand(config_level3_cmd2())
        .subcommand(config_level3_cmd3())
}

fn config_level3_cmd1() -> App<'static> {
    clap::App::new("level3_cmd1")
        .about("level3_cmd1 info ")
        .subcommand(config_show_info_cmd())
        .subcommand(config_show_all_cmd())
}

fn config_level3_cmd2() -> App<'static> {
    clap::App::new("level3_cmd2")
        .about("level3_cmd2 info ")
        .subcommand(config_show_info_cmd())
        .subcommand(config_show_all_cmd())
}

fn config_level3_cmd3() -> App<'static> {
    clap::App::new("level3_cmd3")
        .about("level3_cmd3 info ")
        .subcommand(config_show_info_cmd())
        .subcommand(config_show_all_cmd())
}

fn config_show_info_cmd() -> App<'static> {
    clap::App::new("info").about("show info")
}

fn config_show_all_cmd() -> App<'static> {
    clap::App::new("all").about("show all ")
}

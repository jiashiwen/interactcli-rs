use clap::App;

pub fn new_config_cmd() -> App<'static> {
    clap::App::new("config")
        .about("config")
        .subcommand(config_show_cmd())
}

fn config_show_cmd() -> App<'static> {
    clap::App::new("show")
        .about("show some info ")
        .subcommand(config_show_info_cmd())
        .subcommand(config_show_all_cmd())
}

fn config_show_info_cmd() -> App<'static> {
    clap::App::new("info").about("show info")
}

fn config_show_all_cmd() -> App<'static> {
    clap::App::new("all").about("show all ")
}

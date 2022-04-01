use clap::Command;

pub fn new_requestsample_cmd() -> Command<'static> {
    clap::Command::new("requestsample")
        .about("requestsample")
        .subcommand(get_baidu_cmd())
}

pub fn get_baidu_cmd() -> Command<'static> {
    clap::Command::new("baidu").about("request www.baidu.com")
}

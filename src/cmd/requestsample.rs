use clap::App;

pub fn new_requestsample_cmd() -> App<'static> {
    clap::App::new("requestsample")
        .about("requestsample")
        .subcommand(get_baidu_cmd())
}

pub fn get_baidu_cmd() -> App<'static> {
    clap::App::new("baidu").about("request www.baidu.com")
}

use clap::App;

pub fn new_loop_cmd() -> App<'static> {
    clap::App::new("loop").about("loop")
}

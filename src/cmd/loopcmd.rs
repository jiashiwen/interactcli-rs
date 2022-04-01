use clap::Command;

pub fn new_loop_cmd() -> Command<'static> {
    clap::Command::new("loop").about("loop")
}

use clap::Command;

pub fn new_loop_cmd() -> Command {
    clap::Command::new("loop").about("loop")
}

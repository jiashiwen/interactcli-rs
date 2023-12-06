use clap::Command;

pub fn new_spinoff_sample_cmd() -> Command {
    clap::Command::new("spinoff_sample").about("spinoff_sample")
}

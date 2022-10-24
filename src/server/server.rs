use std::{thread, time::Duration};

pub fn start(prefix: String) {
    for i in 0..1000 {
        println!("{}", prefix.clone() + &i.to_string());
        thread::sleep(Duration::from_secs(1));
    }
}

mod command;
use std::io::{self, Write};

use command::systemd_health::systemd_health;

fn main() {
    println!("Hello, world!");
    print_info_for("marisa");
    print_info_for("remote-port-forward");
}

fn print_info_for(system_name: &str) {
    print!("{system_name} status: ");
    io::stdout().flush().expect("flushing to stdout to work!");
    let status = systemd_health(system_name).to_color();
    println!("systemd - {status}");
}

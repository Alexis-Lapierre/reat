use core::panic;
use std::process::Command;

use crossterm::style::Stylize;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Systemd {
    Running,
    Degraded,
}

pub fn systemd_health(host: &str) -> Systemd {
    let command = Command::new("ssh")
        .arg(host)
        .arg("systemctl")
        .arg("status")
        .arg("--failed")
        .output()
        .expect("Failed to execute command");

    if !command.status.success() {
        panic!(
            "Failed to execute command!\nstdout: {} stderr: {}",
            String::from_utf8(command.stdout).unwrap(),
            String::from_utf8(command.stderr).unwrap()
        );
    }

    if command.stdout == b"" {
        Systemd::Running
    } else {
        Systemd::Degraded
    }
}

impl Systemd {
    pub fn to_color(self) -> crossterm::style::StyledContent<&'static str> {
        match self {
            Systemd::Running => "running".green(),
            Systemd::Degraded => "degraded".red(),
        }
    }
}

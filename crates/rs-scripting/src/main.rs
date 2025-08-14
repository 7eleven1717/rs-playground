#!/usr/bin/env -S cargo +nightly -q -Zscript

use std::process::Command;

fn main() -> std::io::Result<()> {
    for (key, value) in std::env::vars() {
        println!("{}: {}", key, value);
    }

    // let _exit_status = Command::new("sudo")
    //     .args(["apt-get", "update"])
    //     .status()?;

    // let _exit_status = Command::new("echo")
    //     .arg("Hello, world!")
    //     .status()?;

    Ok(())
}

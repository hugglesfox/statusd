//! # Statusd
//!
//! Statusd is a xsetroot(1) based status bar daemon.
//!
//! The status line is somewhat modular as each component is a struct which
//! implements
//! [`std::fmt::Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html)
//! in order to control how it's displayed. Although this means that source code
//! changes are required in order to customise the status bar.
use std::thread;
use std::time::Duration;

mod clock;
mod xsetroot;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        // The status line format
        let status = format!("{}", clock::Clock);

        xsetroot::name(status)?;
        thread::sleep(Duration::from_secs(5))
    }
}

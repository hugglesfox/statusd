//! # Statusd
//!
//! Statusd is a xsetroot(1) based status bar daemon.
//!
//! The status line is somewhat modular as each component is a struct which
//! implements
//! [`std::fmt::Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html)
//! in order to control how it's displayed. Although this means that source code
//! changes are required in order to customise the status bar.
extern crate pretty_env_logger;

use log::info;
use std::thread;
use std::time::Duration;

mod battery;
mod clock;
mod notifications;
mod xsetroot;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init_custom_env("STATUSD_LOG");

    info!("Starting statusd...");

    let system = zbus::Connection::new_system()?;
    let session = zbus::Connection::new_session()?;

    let bat = battery::Battery::new(&system)?;
    let notif = notifications::Notifications::new(&session)?;

    loop {
        // The status line format
        let status = format!(" {} | {} | {}", notif, bat, clock::Clock);

        xsetroot::name(status)?;
        thread::sleep(Duration::from_secs(5))
    }
}

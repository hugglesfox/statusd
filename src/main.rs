use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;
use zbus::{ConnectionBuilder, Result};

mod dbus;
mod xsetroot;

pub const BUS_NAME: &str = "org.freedesktop.Notifications";
pub const OBJ_PATH: &str = "/org/freedesktop/Notifications";

pub struct Notification {
    pub summary: String,
    pub body: String,
    pub expire_timeout: i32,
}

#[tokio::main]
async fn main() -> Result<()> {
    let (tx, mut rx) = mpsc::channel::<Notification>(10);

    let interface = dbus::Interface::new(tx);

    let _connection = ConnectionBuilder::session()?
        .name(BUS_NAME)?
        .serve_at(OBJ_PATH, interface)?
        .build()
        .await?;

    while let Some(notif) = rx.recv().await {
        xsetroot::name(format!("{}, {}", notif.summary, notif.body)).ok();

        // Ensure that the status bar can't update until a notification expires
        sleep(Duration::from_millis(notif.expire_timeout as u64)).await;
    }

    Ok(())
}

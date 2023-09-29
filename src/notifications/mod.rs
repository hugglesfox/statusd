use log::{debug, info};
use zbus::{ConnectionBuilder, Result};
use async_std::sync::Arc;

mod dbus;
mod notification;
mod store;

use store::NotificationStore;

pub const BUS_NAME: &str = "org.freedesktop.Notifications";
pub const OBJ_PATH: &str = "/org/freedesktop/Notifications";

pub async fn notifications_task() -> Result<()> {
    let notifications = Arc::new(NotificationStore::new());

    let interface = dbus::Interface::new(notifications.clone());

    let connection = ConnectionBuilder::session()?
        .name(BUS_NAME)?
        .serve_at(OBJ_PATH, interface)?
        .build()
        .await?;

    info!("Listening for notifications on {} {}", BUS_NAME, OBJ_PATH);

    loop {
        for id in notifications.expired_ids().await {
            debug!("Closing expired notification {}", id);
            notifications.remove(id).await;
            connection.emit_signal(None::<&str>, OBJ_PATH, BUS_NAME, "NotificationClosed", &(id, dbus::Reason::Expired)).await.ok();
        }
    }
}

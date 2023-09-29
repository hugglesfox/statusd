use async_std::sync::Arc;
use crate::notifications::notification::{Hints, Notification};
use crate::notifications::store::NotificationStore;
use log::{debug, warn};
use serde_repr::{Serialize_repr, Deserialize_repr};
use zbus::zvariant::Type;
use zbus::{SignalContext, dbus_interface};


/// Notification closure reason
#[derive(Serialize_repr, Deserialize_repr, Type)]
#[repr(u32)]
pub enum Reason {
    Expired = 1,
    Dismissed = 2,
    Closed = 3,
    Undefined = 4,
}

/// DBus notification interface
///
/// See https://specifications.freedesktop.org/notification-spec/notification-spec-latest.html for more information.
pub struct Interface {
    id: u32,
    notifications: Arc<NotificationStore>, 
}

impl Interface {
    pub fn new(notifications: Arc<NotificationStore>) -> Self {
        Self {
            id: 0,
            notifications,
        }
    }
}

#[dbus_interface(name = "org.freedesktop.Notifications")]
impl Interface {
    /// Get the capabilities of the notification daemon
    fn get_capabilities(&self) -> &[&str] {
        &["body", "persistence"]
    }

    /// Create a new notification
    async fn notify(
        &mut self,
        app_name: String,
        replaces_id: u32,
        app_icon: String,
        summary: String,
        body: String,
        actions: Vec<String>,
        hints: Hints,
        expire_timeout: i32,
    ) -> u32 {
        self.id = self.id.wrapping_add(1);

        self.notifications.insert(
            self.id,
            Notification::new(
                app_name,
                replaces_id,
                app_icon,
                summary,
                body,
                actions,
                hints,
                expire_timeout,
            ),
        ).await;

        debug!("Created notification {:?}", self.id);

        self.id
    }

    /// Close a notification
    async fn close_notification(
        &mut self,
        #[zbus(signal_context)] ctxt: SignalContext<'_>,
        id: u32,
    ) {
        if let None = self.notifications.remove(id).await {
            warn!("Tried to close non-existant notification with id {}", id)
        };

        debug!("Notification {} closed", id);

        Self::notification_closed(&ctxt, id, Reason::Closed).await.unwrap();
    }

    /// Get information about the notification server
    fn get_server_information(&self) -> (&str, &str, &str, &str) {
        ("notifyd", "freedesktop.org", env!("CARGO_PKG_VERSION"), "1.2")
    }

    #[dbus_interface(signal)]
    async fn notification_closed(ctx: &SignalContext<'_>, id: u32, reason: Reason) -> zbus::Result<()> {}
}

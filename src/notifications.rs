use crate::Status;
use std::collections::HashMap;
use tokio::sync::mpsc::Sender;
use zbus::zvariant::Value;
use zbus::{dbus_interface, Result, SignalContext, ConnectionBuilder};

/// DBus notification interface
///
/// See [the freedesktop notification
/// spec](https://specifications.freedesktop.org/notification-spec/notification-spec-latest.html)
/// for more information.
pub struct Interface {
    id: u32,
    channel: Sender<Status>,
}

impl Interface {
    pub fn new(channel: Sender<Status>) -> Self {
        Self { id: 0, channel }
    }
}

#[dbus_interface(name = "org.freedesktop.Notifications")]
impl Interface {
    /// Get the capabilities of the notification daemon
    fn get_capabilities(&self) -> &[&str] {
        &["body"]
    }

    /// Create a new notification
    async fn notify(
        &mut self,
        #[zbus(signal_context)]
        ctxt: SignalContext<'_>,
        _app_name: String,
        _replaces_id: u32,
        _app_icon: String,
        summary: String,
        body: String,
        _actions: Vec<String>,
        _hints: HashMap<&str, Value<'_>>,
        expire_timeout: i32,
    ) -> u32 {
        // Close the previous notification, telling the client that the notification expired
        Self::notification_closed(&ctxt, self.id, 1).await.ok();

        self.id = self.id.wrapping_add(1);

        let timeout = match expire_timeout {
            -1 | 0 => 1000,
            _ => expire_timeout,
        };

        self.channel
            .send(Status::new(format!("{}; {}", summary, body), timeout as u64))
            .await
            .unwrap();

        self.id
    }

    /// Close a notification
    async fn close_notification(&self, #[zbus(signal_context)] ctxt: SignalContext<'_>, id: u32) {

        // We can't actually close a notification, so we just pretend that we did
        Self::notification_closed(&ctxt, id, 3).await.ok();
    }

    /// Get information about the notification server
    fn get_server_information(&self) -> (&str, &str, &str, &str) {
        (
            "statusd",
            "freedesktop.org",
            env!("CARGO_PKG_VERSION"),
            "1.2",
        )
    }

    #[dbus_interface(signal)]
    async fn notification_closed(ctx: &SignalContext<'_>, id: u32, reason: u32) -> Result<()> {}
}

/// A server task
pub async fn server(channel: Sender<Status>) -> Result<()> {
    let interface = Interface::new(channel);

    let _connection = ConnectionBuilder::session()?
        .name("org.freedesktop.Notifications")?
        .serve_at("/org/freedesktop/Notifications", interface)?
        .build()
        .await?;

    loop {
        std::future::pending().await
    }
}

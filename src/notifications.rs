use std::fmt;
use zbus::dbus_proxy;
use zbus::fdo;

#[dbus_proxy(
    interface = "org.freedesktop.Notifications",
    default_path = "/org/freedesktop/Notifications"
)]
trait Notifyd {
    fn get_notification_count(&self) -> fdo::Result<u32>;
}

/// A unread notifications module
///
/// Displays the amount of open notifications
pub struct Notifications<'a> {
    proxy: NotifydProxy<'a>,
}

impl Notifications<'_> {
    pub fn new(conn: &zbus::Connection) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            proxy: NotifydProxy::new(&conn)?,
        })
    }
}

impl fmt::Display for Notifications<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.proxy.get_notification_count.unwrap())
    }
}

use chrono::prelude::*;
use log::error;
use std::fmt;
use zbus::dbus_proxy;
use zbus::fdo;

/// UPower dbus interface
#[dbus_proxy(
    interface = "org.freedesktop.UPower.Device",
    default_service = "org.freedesktop.UPower",
    default_path = "/org/freedesktop/UPower/devices/DisplayDevice"
)]
trait Upower {
    #[dbus_proxy(property)]
    fn time_to_empty(&self) -> fdo::Result<i64>;

    #[dbus_proxy(property)]
    fn time_to_full(&self) -> fdo::Result<i64>;
}

/// A battery status module
///
/// Whilst charging it displays the time till charged and whilst discharging it
/// displays the time till empty.
pub struct Battery<'a> {
    proxy: UpowerProxy<'a>,
}

impl Battery<'_> {
    pub fn new(conn: &zbus::Connection) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            proxy: UpowerProxy::new(&conn)?,
        })
    }
}

impl fmt::Display for Battery<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Whilst discharging, time_to_full() is 0 and vice versa
        let time = self.proxy.time_to_full().unwrap_or_else(|e| {
            error!("Unable to connect to upowerd: {}", e);
            0
        }) + self.proxy.time_to_empty().unwrap_or_else(|e| {
            error!("Unable to connect to upowerd, {}", e);
            0
        });
        write!(
            f,
            "{}",
            NaiveTime::from_num_seconds_from_midnight(time as u32, 0).format("%H:%M:%S")
        )
    }
}

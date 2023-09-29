use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;

mod notifications;
mod xsetroot;

pub struct Status {
    /// The text to display
    pub text: String,
    /// The minimum amount of time to display the status
    pub timeout: u64,
}

impl Status {
    pub fn new(text: String, timeout: u64) -> Self {
        Self { text, timeout }
    }
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<Status>(1);

    tokio::spawn(notifications::server(tx.clone()));

    while let Some(status) = rx.recv().await {
        xsetroot::name(status.text).ok();
        sleep(Duration::from_millis(status.timeout)).await;
    }
}

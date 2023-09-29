extern crate pretty_env_logger;

use async_std::task;

mod notifications;

#[async_std::main]
async fn main() {
    task::spawn(notifications::notifications_task());
}


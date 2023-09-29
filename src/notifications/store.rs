use async_std::sync::RwLock;
use std::collections::HashMap;
use crate::notifications::notification::Notification;

/// A thread safe key value store for notifications
pub struct NotificationStore(RwLock<HashMap<u32, Notification>>);

impl NotificationStore {
    pub fn new() -> Self {
        Self(RwLock::new(HashMap::new()))
    }
    pub async fn insert(&self, id: u32, notification: Notification) -> Option<Notification> {
        self.0.write().await.insert(id, notification)
    }

    pub async fn remove(&self, id: u32) -> Option<Notification> {
        self.0.write().await.remove(&id)
    }

    pub async fn expired_ids(&self) -> Vec<u32> {
        self.0.write().await
            .iter()
            .filter(|(_, n)| n.is_expired())
            .map(|(id, _)| *id)
            .collect()
    }
}


use axum::async_trait;

use crate::providers::errors::ProviderError;
use crate::models::EmailNotification;

#[async_trait]
pub trait EmailProvider {
    async fn send(&self, notification: EmailNotification) -> Result<(), ProviderError>;
}

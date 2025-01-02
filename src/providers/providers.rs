use async_trait::async_trait;

use crate::models::EmailNotification;
use crate::providers::errors::ProviderError;

#[async_trait]
pub trait EmailProvider {
    async fn send(&self, notification: EmailNotification) -> Result<(), ProviderError>;
}

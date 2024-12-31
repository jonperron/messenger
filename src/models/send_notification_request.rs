use serde::Deserialize;
use serde_json::Value;

use crate::providers::errors::ProviderError;

#[derive(Debug, Deserialize)]
pub struct SendNotificationRequest {
    pub to: String,
    pub from: String,
    pub subject: String,
    pub template_name: String,
    pub context: Value,
    pub notification_type: String,
    pub provider: String,
}

impl SendNotificationRequest {
    pub fn validate(&self) -> Result<(), ProviderError> {
        match (self.provider.as_str(), self.notification_type.as_str()) {
            ("mailgun", "email") => Ok(()), // Accept mailgun for email notifications
            _ => Err(ProviderError::invalid_config(format!(
                "Invalid configuration: Provider {} is not supported for {} notifications",
                self.provider, self.notification_type
            ))),
        }
    }
}
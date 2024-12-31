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
        match self.notification_type.as_str() {
            "email" => {
                if ["mailgun"].contains(&self.provider.as_str()) {
                    return Err(ProviderError::invalid_config(
                        format!("Provider {} is not supported for email notifications", self.provider)
                    ));
                }
            }
            _ => return Err(ProviderError::invalid_config("Unsupported notification type")),
        }

        Ok(())
    }
}
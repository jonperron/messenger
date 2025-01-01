use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::providers::errors::ProviderError;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_send_notification_request_serialization() {
        let send_notification_request = SendNotificationRequest {
            to: "receiver@example.com".to_string(),
            from: "sender@example.com".to_string(),
            subject: "Test subject".to_string(),
            template_name: "hello_world.html".to_string(),
            context: serde_json::json!({"foo": "bar"}),
            notification_type: "email".to_string(),
            provider: "test".to_string(),
        };
        let json = serde_json::to_string(&send_notification_request).expect("Serialization failed");
        assert!(json.contains("sender@example.com"));
        assert!(json.contains("receiver@example.com"));

        let deserialized_send_notification_request: SendNotificationRequest =
            serde_json::from_str(&json).expect("Deserialization failed");
        assert_eq!(
            deserialized_send_notification_request,
            send_notification_request
        );
    }

    #[test]
    fn test_send_notification_request_validation() {
        // Valid notification type and provider
        let valid_send_notification_request = SendNotificationRequest {
            to: "receiver@example.com".to_string(),
            from: "sender@example.com".to_string(),
            subject: "Test subject".to_string(),
            template_name: "hello_world.html".to_string(),
            context: serde_json::json!({"foo": "bar"}),
            notification_type: "email".to_string(),
            provider: "mailgun".to_string(),
        };

        assert!(valid_send_notification_request.validate().is_ok());

        // Invalid provider
        let invalid_send_notification_request = SendNotificationRequest {
            to: "receiver@example.com".to_string(),
            from: "sender@example.com".to_string(),
            subject: "Test subject".to_string(),
            template_name: "hello_world.html".to_string(),
            context: serde_json::json!({"foo": "bar"}),
            notification_type: "email".to_string(),
            provider: "test".to_string(),
        };

        assert!(invalid_send_notification_request.validate().is_err());
    }
}

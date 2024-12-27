use std::sync::Arc;
use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;

use crate::providers::{
    providers::EmailProvider,
    mailgun::MailgunProvider,
    notifications::EmailNotification,
    errors::ProviderError,
};
use crate::{
    config::Config,
    models::send_notification_request::SendNotificationRequest,
    templates::tera_engine::TemplateEngine,
};


pub fn create_email_notification(
    config: &Config,
    request: &SendNotificationRequest,
    template_engine: &TemplateEngine,
) -> Result<EmailNotification, ProviderError> {
    // Load the template using the template ID
    let template = template_engine
        .load(format!("{}/{}", &config.templates.path, &request.template_name).as_str())
        .map_err(|_| ProviderError::template_error("Template not found"))?;

    // Render the template with the provided data
    let body = template
        .render(request.context.clone())
        .map_err(|_| ProviderError::template_error("Failed to render template"))?;

    // Create and return the Notification object
    Ok(EmailNotification {
        from: request.from.clone(),
        to: request.to.clone(),
        subject: request.subject.clone(), 
        body,
    })
}

pub async fn send_notification(
    Json(request): Json<SendNotificationRequest>,
    template_engine: Arc<TemplateEngine>,
    mailgun_provider: Arc<MailgunProvider>,
    config: Arc<Config>,
) -> impl IntoResponse {
    // Validate the request
    if let Err(e) = request.validate() {
        return (StatusCode::BAD_REQUEST, Json(json!({"error": e.to_string()})));
    }

    // Create notification regarding type
    let notification = match request.notification_type.as_str() {
        "email" => create_email_notification(&config, &request, &template_engine),
        _ => Err(ProviderError::invalid_config("Unsupported notification type")),
    };

    // Find provider based on notification type and provider provided
    let provider_result =  match request.provider.as_str() {
        "mailgun" => match notification {
            Ok(n) => mailgun_provider.send(n).await,
            Err(e) => Err(e),
        },
        _ => Err(ProviderError::invalid_config("Unsupported provider")),
    };

    // Select provider
    match provider_result {
        Ok(_) => (StatusCode::OK, Json(json!({"message": "Notification sent"}))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))),
    }
}
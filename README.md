# Messenger

Messenger is a flexible service built in Rust for sending notifications via multiple providers. It supports both synchronous API calls and asynchronous message consumption, allowing seamless integration into modern architectures. The service is designed to be provider-agnostic, configurable, and extensible for various notification types.

##  Features

* Multi-Provider Support: Easily integrate providers like Mailgun, Mailjet, or others for email notifications. Future-proof for adding push notifications (e.g., Firebase).
* Templated Notifications: Supports dynamic content generation using [Tera](https://keats.github.io/tera/docs/) for customizable templates.
* Synchronous and Asynchronous Modes:
  * Synchronous: Use the /send HTTP endpoint to send notifications via API.
  * Asynchronous: TBA
* Configurable via YAML. Support for k8s to be added
* Health Checks: Includes a /health endpoint for monitoring and readiness probes.
* Tracing and Observability: Built-in support for structured logging and distributed tracing using [tracing](https://github.com/tokio-rs/tracing).

## How It Works

* Configuration: Define provider settings, template paths, and notification preferences in a config.yaml file.
* Templating: Use Tera templates for dynamic content generation. Templates can be organized by language or type (e.g., templates/hello_world.en.html).
* Sending Notifications:
  * API: Send a POST request to /send with the notification payload.
* Extensibility: Add new providers by implementing the EmailProvider trait.

## Supported providers

* Email: Mailgun

## API Example

`/send` Endpoint

Send a notification via API:

* Request: POST /send
* Headers: Content-Type: application/json
* Body:

```json
{
  "notification_type": "email",
  "provider": "mailgun",
  "template_name": "hello_world",
  "from": "sender@example.com",
  "to": "receiver@example.com",
  "subject": "Hello, World!"
}

```

* Response:

```json
{
    "message":"Notification sent"
}
```

## Running the Service

Build and run the service using cargo:

```bash
cargo run
```

The service is also available via Docker:

```bash
docker buildx build -t messenger .
docker run messenger:latest
```

Access the API on <http://localhost:3000>.

## Configuration

See config.yaml:

```yaml
templates:
  path: "templates"
  default_language: "en"

providers:
  mailgun:
    domain: "your-mailgun-domain.com"
    api_key: "your-mailgun-api-key"
    base_url: "https://api.mailgun.net"
```

## Monitoring

Use the /health endpoint to monitor the service's status:

```bash
curl http://localhost:3000/health
```

## Contributing

TBA

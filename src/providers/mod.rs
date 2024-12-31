// Declaration
pub mod errors;
pub mod mailgun;
pub mod providers;

// Limit import to only what is useful
pub use providers::EmailProvider;
pub use mailgun::MailgunProvider;
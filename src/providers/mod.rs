use serde:: {Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmailNotification {
    pub from: String,
    pub to: String,
    pub subject: String,
    pub body: String,
}

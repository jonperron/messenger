use tera::{Tera, Context};
use std::{collections::HashMap, sync::Arc, env};
use serde_json::Value;
use tracing::info;

pub struct TemplateEngine {
    tera: Arc<Tera>,
}

impl TemplateEngine {
    pub fn new(template_path: &str) -> Result<Self, tera::Error> {
        let tera = Tera::new(template_path)?;
        Ok(Self { tera: Arc::new(tera) })
    }

    pub fn load(&self, template_name: &str) -> Result<Template, String>{
        if self.tera.get_template(template_name).is_err() {
            return Err(format!("Template '{}' not found", template_name));
        }
        
        Ok(Template {
            name: template_name.to_string(),
            tera: self.tera.clone(),
        })
    }
}

#[derive(Debug)]
pub struct Template {
    name: String,
    tera: Arc<Tera>,
}

impl Template {

    pub fn render(&self, data: Value) -> Result<String, String> {
        // Convert data from request to hashmap
        let data_map: HashMap<String, Value> = serde_json::from_value(data).map_err(|e| format!("Failed to convert data: {}", e))?;

        let mut context = Context::new();
        for (key, value) in data_map {
            context.insert(key.as_str(), &value);
        }

        self.tera.render(&self.name, &context).map_err(|e| format!("Failed to render template: {}", e))
    }
}
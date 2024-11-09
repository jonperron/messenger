use tera::{Tera, Context};
use std::path::Path;

pub struct TemplateEngine {
    tera: Tera,
}

impl TemplateEngine {
    pub fn new(directory: &Path, extension: &str) -> Result<Self, tera::Error> {
        let pattern = format!("{}{}", directory.display(), extension);
        let tera = Tera::new(&pattern)?;
        Ok(Self { tera })
    }

    pub fn render(&self, template_name: &str, context: &serde_json::Value) -> Result<String, tera::Error> {
        let mut tera_context = Context::new();
        for (key, value) in context.as_object().unwrap_or(&serde_json::Map::new()) {
            tera_context.insert(key, value);
        }
        self.tera.render(template_name, &tera_context)
    }
}

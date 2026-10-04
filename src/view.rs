use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub trait ViewEngine: Send + Sync {
    fn render(&self, template_name: &str, data: HashMap<String, String>) -> Result<String, String>;
}

pub struct BladeEngine {
    views_path: String,
}

impl BladeEngine {
    pub fn new(views_path: impl Into<String>) -> Self {
        Self { views_path: views_path.into() }
    }
}

impl ViewEngine for BladeEngine {
    fn render(&self, template_name: &str, data: HashMap<String, String>) -> Result<String, String> {
        let relative_path = template_name.replace('.', "/") + ".html";
        let full_path = Path::new(&self.views_path).join(relative_path);

        let mut content = fs::read_to_string(&full_path)
            .map_err(|_| format!("View not found: {:?}", full_path))?;

        for (key, value) in data {
            let placeholder = format!("{{{{ {} }}}}", key);
            content = content.replace(&placeholder, &value);
        }

        Ok(content)
    }
}
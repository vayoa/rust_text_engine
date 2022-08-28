#[derive(Debug)]
pub enum FileFormat {
    Json,
    Yaml,
}

impl FileFormat {
    pub fn name(&self) -> &'static str {
        match &self {
            FileFormat::Json => "json",
            FileFormat::Yaml => "yaml",
        }
    }
}

impl Default for FileFormat {
    fn default() -> Self {
        FileFormat::Yaml
    }
}


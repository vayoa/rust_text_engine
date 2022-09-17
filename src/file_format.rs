use serde::de;
use serde_json::from_str as json_from_str;
use serde_yaml::from_str as yaml_from_str;

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

    pub fn deserialize_str<'a, T>(&self, s: &'a str) -> Result<T, FormatError>
    where
        T: de::Deserialize<'a>,
    {
        Ok(match self {
            FileFormat::Json => json_from_str::<T>(s)?,
            FileFormat::Yaml => yaml_from_str::<T>(s)?,
        })
    }
}

impl Default for FileFormat {
    fn default() -> Self {
        FileFormat::Yaml
    }
}

pub enum FormatError {
    YAML(serde_yaml::Error),
    JSON(serde_json::Error),
}

impl FormatError {
    pub fn name(&self) -> String {
        match &self {
            Self::JSON(_) => "json",
            Self::YAML(_) => "YAML",
        }
        .to_string()
    }
}

impl From<serde_json::Error> for FormatError {
    fn from(e: serde_json::Error) -> Self {
        Self::JSON(e)
    }
}
impl From<serde_yaml::Error> for FormatError {
    fn from(e: serde_yaml::Error) -> Self {
        Self::YAML(e)
    }
}

impl ToString for FormatError {
    #[inline]
    fn to_string(&self) -> String {
        match self {
            Self::YAML(e) => e.to_string(),
            Self::JSON(e) => e.to_string(),
        }
    }
}

use std::convert::TryFrom;

use serde_json::Value;

use super::{Schema, SchemaError};

pub struct Audio {
    pub title: String,
    pub project: String,
}

impl Into<Value> for Audio {
    fn into(self) -> Value {
        let mut map = serde_json::Map::new();
        map.insert("title".to_string(), serde_json::Value::String(self.title));
        map.insert("project".to_string(), Value::String(self.project));
        serde_json::Value::Object(map)
    }
}

impl TryFrom<Value> for Audio {
    type Error = SchemaError;
    fn try_from(value: Value) -> Result<Self, SchemaError> {
        let title = value["title"].as_str().ok_or(SchemaError::MissingField("title".to_string()))?;
        let project = value["project"].as_str().ok_or(SchemaError::MissingField("project".to_string()))?;

        Ok(Self {
            title: title.to_string(),
            project: project.to_string(),
        })
    }
}

impl Schema for Audio {
    const NAME: &'static str = "audio";

    fn valid_extensions() -> Vec<&'static str> {
        vec!["mp3", "wav"]
    }

    fn fields() -> Vec<(&'static str, &'static str)> {
        vec![
            ("title", "The title of the piece"),
            ("project", "A short description of the piece, if any")
        ]
    }
}

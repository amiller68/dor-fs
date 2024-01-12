use std::convert::TryFrom;

use serde_json::Value;

use super::{Schema, SchemaError};

pub struct Visual {
    pub title: String,
    pub medium: String,
    pub location: String,
}

impl From<Visual> for Value {
    fn from(val: Visual) -> Self {
        let mut map = serde_json::Map::new();
        map.insert("title".to_string(), serde_json::Value::String(val.title));
        map.insert("medium".to_string(), serde_json::Value::String(val.medium));
        map.insert(
            "location".to_string(),
            serde_json::Value::String(val.location),
        );
        serde_json::Value::Object(map)
    }
}

impl TryFrom<Value> for Visual {
    type Error = SchemaError;
    fn try_from(value: Value) -> Result<Self, SchemaError> {
        let title = value["title"]
            .as_str()
            .ok_or(SchemaError::MissingField("title".to_string()))?;
        let medium = value["medium"]
            .as_str()
            .ok_or(SchemaError::MissingField("medium".to_string()))?;
        let location = value["location"]
            .as_str()
            .ok_or(SchemaError::MissingField("location".to_string()))?;

        Ok(Self {
            title: title.to_string(),
            medium: medium.to_string(),
            location: location.to_string(),
        })
    }
}

impl Schema for Visual {
    const NAME: &'static str = "visual";

    fn valid_extensions() -> Vec<&'static str> {
        vec!["png", "jpg", "jpeg", "gif"]
    }

    fn fields() -> Vec<(&'static str, &'static str)> {
        vec![
            ("title", "The title of the piece"),
            ("medium", "The original medium of the work"),
            ("location", "Where the piece was made"),
        ]
    }
}

use std::convert::TryFrom;
use std::fmt::Display;

use serde_json::Value;

use super::{Schema, SchemaError};

// Note: more than a lil wierd to hardcode projects as an enum,
//  but at the very least this will make it easier to keep track of and
//   sort new projects

// TODO: docuement potential audio projects here
#[derive(Clone)]
pub enum AudioProject {
    /// Short recordings, experiments, and one-offs
    MicTest,
}

impl Display for AudioProject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use AudioProject as AP;
        write!(
            f,
            "{}",
            match self {
                AP::MicTest => "mic_test",
            }
        )
    }
}

impl TryFrom<&str> for AudioProject {
    type Error = SchemaError;

    fn try_from(val: &str) -> Result<Self, SchemaError> {
        let variant = match val {
            "mic_test" => Self::MicTest,
            _ => {
                return Err(SchemaError::InvalidField(
                    "project".to_string(),
                    val.to_string(),
                ))
            }
        };
        Ok(variant)
    }
}

pub struct Audio {
    pub title: String,
    pub project: AudioProject,
}

impl From<Audio> for Value {
    fn from(val: Audio) -> Self {
        let mut map = serde_json::Map::new();
        map.insert("title".to_string(), serde_json::Value::String(val.title));
        map.insert(
            "project".to_string(),
            Value::String(val.project.to_string()),
        );
        serde_json::Value::Object(map)
    }
}

impl TryFrom<Value> for Audio {
    type Error = SchemaError;
    fn try_from(value: Value) -> Result<Self, SchemaError> {
        let title = value["title"]
            .as_str()
            .ok_or(SchemaError::MissingField("title".to_string()))?;
        let project = value["project"]
            .as_str()
            .ok_or(SchemaError::MissingField("project".to_string()))?;

        Ok(Self {
            title: title.to_string(),
            project: AudioProject::try_from(project)?,
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
            ("project", "The project this piece belongs to"),
        ]
    }
}

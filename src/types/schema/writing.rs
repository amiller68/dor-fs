use std::fmt::Display;

use serde_json::Value;

use super::{Schema, SchemaError};

#[derive(Clone, PartialEq, Eq)]
pub enum WritingGenre {
    Poetry,
    Fiction,
    Blog,
    Essay,
}

impl Display for WritingGenre {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use WritingGenre as WG;
        write!(
            f,
            "{}",
            match self {
                WG::Blog => "blog".to_string(),
                WG::Poetry => "poetry".to_string(),
                WG::Fiction => "fiction".to_string(),
                WG::Essay => "essay".to_string(),
            }
        )
    }
}

impl TryFrom<&str> for WritingGenre {
    type Error = SchemaError;

    fn try_from(val: &str) -> Result<Self, SchemaError> {
        let variant = match val {
            "blog" => Self::Blog,
            "poetry" => Self::Poetry,
            "fiction" => Self::Fiction,
            "essay" => Self::Essay,
            _ => {
                return Err(SchemaError::InvalidField(
                    "genre".to_string(),
                    val.to_string(),
                ))
            }
        };
        Ok(variant)
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Writing {
    pub title: String,
    pub description: String,
    pub genre: WritingGenre,
}

impl From<Writing> for Value {
    fn from(val: Writing) -> Self {
        let mut map = serde_json::Map::new();
        map.insert("title".to_string(), serde_json::Value::String(val.title));
        map.insert(
            "description".to_string(),
            serde_json::Value::String(val.description),
        );
        map.insert(
            "genre".to_string(),
            serde_json::Value::String(val.genre.to_string()),
        );
        serde_json::Value::Object(map)
    }
}

impl TryFrom<Value> for Writing {
    type Error = SchemaError;
    fn try_from(value: Value) -> Result<Self, SchemaError> {
        let title = value["title"]
            .as_str()
            .ok_or(SchemaError::MissingField("title".to_string()))?;
        let description = value["description"]
            .as_str()
            .ok_or(SchemaError::MissingField("description".to_string()))?;
        let genre = value["genre"]
            .as_str()
            .ok_or(SchemaError::MissingField("genre".to_string()))?;

        Ok(Self {
            title: title.to_string(),
            description: description.to_string(),
            genre: WritingGenre::try_from(genre)?,
        })
    }
}

impl Schema for Writing {
    const NAME: &'static str = "writing";

    fn valid_extensions() -> Vec<&'static str> {
        vec!["md"]
    }

    fn fields() -> Vec<(&'static str, &'static str)> {
        vec![
            ("title", "The title of the piece"),
            ("desciption", "A short description of the piece"),
            (
                "genre",
                "The genre of the piece. Must be one of (poetry, fiction, blog)",
            ),
        ]
    }
}

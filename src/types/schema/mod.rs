use std::convert::TryFrom;

mod writing;
mod audio;
mod visual;

pub use writing::{Writing, WritingGenre};
pub use audio::Audio;
pub use visual::Visual;

use serde_json::Value;

// NOTE: this whole notion of a schema is poorly concieved
//  For example, how do you migrate schemas if they get updated?
//  How Do you reduce the amount of boilerplate needed to implement one?
//  How would you get to the eventual goal of defining these an ANSII or something?
//  Lots of unanswered question, but I'm just experimenting here, and for now it should be alright :/
// TODO: These could generally benefit from macros

pub trait Schema: Into<Value> + TryFrom<Value> {
    const NAME: &'static str;

    fn name() -> &'static str {
        Self::NAME
    }

    /// Returns a list of valid extensions for this schema.
    fn valid_extensions() -> Vec<&'static str>;

    /// Return the relevant fields for this schema.
    /// Each field is a static str,str tuple where the first 
    /// describes the field name, and the second a short helpful blurb
    fn fields() -> Vec<(&'static str, &'static str)>;

    /// Safe implementation of Into<Value> the also writes the schema name
    fn into_schema_value(self: Self) -> Value {
        let value: Value = self.into();
        // TODO: here we mandate that the value is Object<Map>
        //  but we don't have a good way to require that in the Trait
        match value {
            Value::Object(mut map) => {
                map.insert("type".to_string(), Value::String(Self::NAME.to_string()));
                Value::Object(map)
            }
            _ => panic!("value is not an object")
        }
    }

    // fn try_from_value(value: Value) -> Result<Self, SchemaError>;

    /// Safe implmentation of TryFrom<Value> for trait implementers
    fn schema_from_value(value: Value) -> Result<Self, SchemaError> {
        let r#type = value["type"].as_str().ok_or(SchemaError::MissingType)?;
        if r#type != Self::NAME {
            return Err(SchemaError::MismatchedType(r#type.to_string()))
        }
        // TODO: wow really just losing error info here huh
        Ok(Self::try_from(value).map_err(|_| SchemaError::FailedConversion)?)
    }
}


#[derive(Debug, thiserror::Error)]
pub enum SchemaError {
    #[error("missing schema type")]
    MissingType,
    #[error("mismatched schema type: {0}")]
    MismatchedType(String),
    #[error("missing field in map: {0}")]
    MissingField(String),
    #[error("failed conversion from value")]
    FailedConversion,
    #[error("invalid field in map: field -> {0} | value -> {1}")]
    InvalidField(String, String)
}

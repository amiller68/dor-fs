use serde_json::Value;

use crate::device::DeviceError;

use crate::cli::args::SchemaSubcommand;
use crate::cli::config::{Config, ConfigError};

pub async fn schema_subcommand(
    config: &Config,
    subcommand: &SchemaSubcommand,
) -> Result<(), SchemaSubcommandError> {
    // load the dor_store schema
    let device = config.device()?;
    let mut change_log = config.change_log()?;
    let (_cid, base_dor_store) = change_log.last_version().unwrap();
    let mut dor_store = base_dor_store.clone();

    match subcommand {
        SchemaSubcommand::Create { name, fields } => {
            // add the schema to the dor_store
            dor_store.insert_schema_entry(&name, fields)
        }
        SchemaSubcommand::Remove { name } => {
            // remove the schema from the dor_store
            dor_store.remove_schema_entry(&name)
        }
        SchemaSubcommand::Ls => {
            // list the schemas in the dor_store
            println!("{:?}", dor_store.schema());
        }
        SchemaSubcommand::Tag { name, path } => {
            // Get the schema
            let schema = match dor_store.get_schema_entry(&name) {
                Some(s) => s,
                None => return Err(SchemaSubcommandError::MissingSchemaName(name.clone())),
            };
            // List the schema names, and ask for input
            println!("Schema fields: {:?}", schema);
            println!("Enter values for each field:");

            let mut values = Vec::new();
            for field in schema {
                println!("{}:", field);
                let mut input = String::new();
                if std::io::stdin().read_line(&mut input).is_err() {
                    println!("Error reading input for {}", field);
                    continue;
                }
                values.push(input.trim().to_string());
            }

            // Construct a JSON object from the values
            let mut json_object = serde_json::Map::new();
            for (field, value) in schema.iter().zip(values.iter()) {
                json_object.insert(field.to_string(), serde_json::Value::String(value.clone()));
            }

            // tag the object at path with the schema name
            let json = Value::Object(json_object);
            dor_store.tag_object(&path, &name, &json);
        }
    }
    if base_dor_store != &dor_store {
        let cid = device.hash_dor_store(&dor_store, false).await?;
        let wtf_log = change_log.clone();
        let log = wtf_log.log();
        change_log.update(log, &dor_store, &cid);

        config.set_change_log(change_log)?;
    }
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum SchemaSubcommandError {
    #[error("config error: {0}")]
    Config(#[from] ConfigError),
    #[error("Missing schema name")]
    MissingSchemaName(String),
    #[error("device error: {0}")]
    Device(#[from] DeviceError),
}

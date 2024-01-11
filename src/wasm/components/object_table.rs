use std::path::PathBuf;

use async_trait::async_trait;
use cid::Cid;
use leptos::*;
use leptos_struct_table::*;
use serde::{Deserialize, Serialize};

use crate::types::Object;

use crate::wasm::env::APP_IPFS_GATEWAY_URL;

#[derive(TableComponent, Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct ObjectRow {
    #[table(key, skip)]
    id: Cid,
    item: ObjectLink,
    created_at: String,
    updated_at: String,
}

impl From<(&PathBuf, &Object)> for ObjectRow {
    fn from(item: (&PathBuf, &Object)) -> Self {
        let (path, object) = item;
        let id = object.cid().clone();
        let created_at = object.created_at().to_string();
        let updated_at = object.updated_at().to_string();
        Self {
            id,
            item: ObjectLink {
                path: path.clone(),
                object: object.clone(),
            },
            created_at,
            updated_at,
        }
    }
}

impl IntoView for ObjectLink {
    fn into_view(self) -> View {
        let href = format!("{}/ipfs/{}", APP_IPFS_GATEWAY_URL, self.object.cid());
        let path = format!("{}", self.path.display());
        let elem = view! {
            <a href={href}>{path}</a>
        };
        elem.into_view()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct ObjectLink {
    path: PathBuf,
    object: Object,
}

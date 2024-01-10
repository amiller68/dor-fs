use async_trait::async_trait;
use cid::Cid;
use leptos::*;
use leptos_struct_table::*;
use serde::{Deserialize, Serialize};

use super::InternalLink;
use crate::types::{Manifest, object}

#[derive(TableComponent, Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct ItemRow {
    #[table(key, skip)]
    id: Cid,
    item: ItemLink,
    date: String,
}

impl From<&Item> for ItemRow {
    fn from(item: &Item) -> Self {
        // Create a random id
        let id = item.cid().clone();
        let date = item.date().to_string();
        Self {
            id,
            item: ItemLink(item.clone()),
            date,
        }
    }
}

impl IntoView for ItemLink {
    fn into_view(self) -> View {
        let name = self.0.name().to_string();
        let title = self.0.title().to_string();
        let href = format!("?route=items&query={}", name);
        let html_element = view! {
            <InternalLink query=href msg=title/>
        };
        html_element.into_view()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct ItemLink(Item);

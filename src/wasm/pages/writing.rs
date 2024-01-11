use std::path::PathBuf;

use async_trait::async_trait;
use chrono::naive::NaiveDate;
use cid::Cid;
use leptos::*;
use leptos_struct_table::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::types::{Object, Writing};
use crate::wasm::components::{InternalLink, ObjectLink};

use super::{Page, PageContext};

#[derive(Clone, Serialize, Deserialize)]
pub struct WritingPage(PageContext);

impl Page for WritingPage {
    fn ctx(&self) -> &PageContext {
        &self.0
    }
    fn from_ctx(ctx: PageContext) -> Box<dyn Page> {
        Box::new(Self(ctx))
    }
    fn into_view_ref(&self) -> View {
        self.clone().into_view()
    }
}

impl IntoView for WritingPage {
    fn into_view(self) -> View {
        let items: leptos::RwSignal<Vec<WritingRow>> =
            create_rw_signal(match self.ctx().manifest() {
                // Filter for object with metadata that we can contruct Writng from
                Some(manifest) => manifest
                    .objects()
                    .iter()
                    .filter(|(path, object)| {
                        let metadata = object.metadata();
                        match Writing::try_from(metadata.clone()) {
                            Ok(_) => true,
                            Err(_) => false,
                        }
                    })
                    .map(|(path, object)| object.into())
                    .collect(),
                None => Vec::new(),
            });
        view! {
            <div>
                <h1>
                    Writing
                </h1>
                <p>
                    "I enjoy writing in my free time. Here's a mix of poetry, essays, blog posts, and fiction that I've output over the years"
                </p>
                <WritingRowTable items=items/>
            </div>
        }
        .into_view()
    }
}

#[derive(TableComponent, Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct WritingRow {
    #[table(key, skip)]
    id: Cid,
    title: ObjectLink,
    genre: String,
    date: NaiveDate,
}

impl From<&Object> for WritingRow {
    fn from(object: &Object) -> Self {
        let id = object.cid().clone();
        let metadata = object.metadata();
        // Note: it's gaurnateed that this will succeed at this point
        let writing = Writing::try_from(metadata.clone()).expect("valid writing schema");
        Self {
            id,
            title: ObjectLink {
                cid: object.cid().clone(),
                title: writing.title,
            },
            genre: writing.genre.to_string(),
            date: object.created_at().date_naive(),
        }
    }
}

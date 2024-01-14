use async_trait::async_trait;
use chrono::naive::NaiveDate;
use cid::Cid;
use leptos::*;
use leptos_struct_table::*;
use serde::{Deserialize, Serialize};

use crate::types::{Object, Writing};
use crate::wasm::components::ObjectLink;

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
        let items: leptos::RwSignal<Vec<WritingRow>> = create_rw_signal({
            let manifest = self.ctx().manifest();
            let mut writing = manifest
                .objects()
                .iter()
                .filter(|(_path, object)| {
                    let metadata = object.metadata();
                    match Writing::try_from(metadata.clone()) {
                        Ok(_) => true,
                        Err(_) => false,
                    }
                })
                .map(|(_path, object)| object.into())
                .collect::<Vec<WritingRow>>();
            writing.sort_by(|a, b| b.date.cmp(&a.date));
            writing
        });
        view! {
            <div>
                <h1 class="text-3xl font-bold italic bg-gray-800 p-2">
                    Writing
                    <div class="text-sm font-normal text-gray-200">
                        <p>
                            "Here's a mix of poetry, essays, blog posts, and fiction that I've written."
                        </p>
                    </div>
                </h1>

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
    #[table(head_class = "p-2", cell_class = "p-2")]
    title: ObjectLink,
    #[table(head_class = "p-2", cell_class = "p-2")]
    genre: String,
    #[table(head_class = "p-2", cell_class = "p-2")]
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

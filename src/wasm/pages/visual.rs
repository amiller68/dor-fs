use async_trait::async_trait;
use chrono::naive::NaiveDate;
use cid::Cid;
use leptos::*;
use leptos_struct_table::*;
use serde::{Deserialize, Serialize};

use crate::types::{Object, Visual};
use crate::wasm::components::ObjectLink;

use super::{Page, PageContext};

#[derive(Clone, Serialize, Deserialize)]
pub struct VisualPage(PageContext);

impl Page for VisualPage {
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

impl IntoView for VisualPage {
    fn into_view(self) -> View {
        let items: leptos::RwSignal<Vec<VisualRow>> = create_rw_signal({
            // Filter for object with metadata that we can contruct Writng from
            let manifest = self.ctx().manifest();
            let mut visual = manifest
                .objects()
                .iter()
                .filter(|(_path, object)| {
                    let metadata = object.metadata();
                    match Visual::try_from(metadata.clone()) {
                        Ok(_) => true,
                        Err(_) => false,
                    }
                })
                .map(|(_path, object)| object.into())
                .collect::<Vec<VisualRow>>();
            visual.sort_by(|a, b| b.date.cmp(&a.date));
            visual
        });
        view! {
            <div>
                <h1 class="text-3xl font-bold italic bg-gray-800 p-2">
                    Visual
                    <div class="text-sm font-normal text-gray-200">
                        <p>
                            "Sometimes I sketch stuff, it's not very consistent, but sometimes its neat!"
                        </p>
                    </div>
                </h1>
                <VisualRowTable items=items/>
            </div>
        }
        .into_view()
    }
}

#[derive(TableComponent, Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct VisualRow {
    #[table(key, skip)]
    id: Cid,
    #[table(head_class = "p-2", cell_class = "p-2")]
    title: ObjectLink,
    #[table(head_class = "p-2", cell_class = "p-2")]
    date: NaiveDate,
}

impl From<&Object> for VisualRow {
    fn from(object: &Object) -> Self {
        let id = object.cid().clone();
        let metadata = object.metadata();
        // Note: it's gaurnateed that this will succeed at this point
        let audio = Visual::try_from(metadata.clone()).expect("valid writing schema");
        Self {
            id,
            title: ObjectLink {
                cid: object.cid().clone(),
                title: audio.title,
            },
            date: object.created_at().date_naive(),
        }
    }
}

use std::path::PathBuf;

use async_trait::async_trait;
use chrono::naive::NaiveDate;
use cid::Cid;
use leptos::*;
use leptos_struct_table::*;
use serde::{Deserialize, Serialize};

use crate::types::Object;
use crate::wasm::components::ObjectLink;
use crate::wasm::env::APP_VERSION;
use crate::wasm::utils::gateway_url;

use super::{Page, PageContext};

#[derive(Clone, Serialize, Deserialize)]
pub struct StatusPage(PageContext);

impl Page for StatusPage {
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

impl IntoView for StatusPage {
    fn into_view(self) -> View {
        let items: leptos::RwSignal<Vec<StatusRow>> = create_rw_signal({
            // Filter for object with metadata that we can contruct Writng from
            let manifest = self.ctx().manifest();
            let mut status = manifest
                .objects()
                .iter()
                .map(|item| item.into())
                .collect::<Vec<StatusRow>>();
            status.sort_by(|a, b| b.date.cmp(&a.date));
            status
        });
        view! {
            <div>
                <h1 class="text-3xl font-bold italic bg-gray-800 p-2">
                    Status
                    <div class="text-sm font-normal text-gray-200">
                        <p>
                            "The content of this site is pointed at by a single piece of monolithic metadata, which is in turn pointed at a by Cid published on Ethereum. This page is a list of all the objects that are pointed at by that metadata."
                        </p>
                    </div>
                </h1>

                <div class="px-4 py-3">
                    <p>
                        <strong class="font-bold">App Version:</strong>
                        <span class="block sm:inline">
                            "   "
                            {APP_VERSION}
                        </span>
                    </p>
                    <p>
                        <strong class="font-bold">Chain Id:</strong>
                        <span class="block sm:inline">
                            "   "
                            {self.ctx().chain_id()}
                        </span>
                    </p>
                    <p>
                    <strong class="font-bold">Cid:</strong>
                        <span class="block sm:inline">
                            {
                                let url = gateway_url(self.ctx().root_cid());
                                view! {
                                    <a href=url.clone() target="_blank" class="link">
                                        "   "
                                        {self.ctx().root_cid().to_string()}
                                    </a>
                                }
                            }
                        </span>
                    </p>
                </div>
                <StatusRowTable items=items/>
            </div>
        }
        .into_view()
    }
}

#[derive(TableComponent, Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct StatusRow {
    #[table(key, skip)]
    id: Cid,
    #[table(head_class = "p-2", cell_class = "p-2")]
    path: ObjectLink,
    #[table(head_class = "p-2", cell_class = "p-2")]
    date: NaiveDate,
}

impl From<(&PathBuf, &Object)> for StatusRow {
    fn from(item: (&PathBuf, &Object)) -> Self {
        let (path, object) = item;
        let id = object.cid().clone();
        Self {
            id,
            path: ObjectLink {
                cid: object.cid().clone(),
                title: format!("{}", path.display()),
            },
            date: object.created_at().date_naive(),
        }
    }
}

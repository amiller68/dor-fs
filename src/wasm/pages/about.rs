use leptos::*;
use serde::{Deserialize, Serialize};

use serde_json::Value;

use super::{Page, PageContext};
use crate::wasm::components::StaticMd;

#[derive(Clone, Serialize, Deserialize)]
pub struct AboutPage(PageContext);

impl Page for AboutPage {
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

impl IntoView for AboutPage {
    fn into_view(self) -> View {
        view! {
            <div>
                <h1 class="text-3xl font-bold italic bg-gray-800 p-2">
                    About Me
                </h1>
                <div class="p-2 md">
                    <StaticMd name="about"/>
                </div>
            </div>
        }
        .into_view()
    }
}

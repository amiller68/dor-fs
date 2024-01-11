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
            <StaticMd name="about"/>
        }
        .into_view()
    }
}

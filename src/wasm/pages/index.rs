use leptos::*;
use serde::{Deserialize, Serialize};

use super::{Page, PageContext};
use crate::wasm::components::{Socials, StaticMd};

#[derive(Clone, Serialize, Deserialize)]
pub struct IndexPage(PageContext);

impl Page for IndexPage {
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

impl IntoView for IndexPage {
    fn into_view(self) -> View {
        view! {
            <div class="index">
                <StaticMd name="index"/>
                <Socials/>
            </div>
        }
        .into_view()
    }
}

use leptos::*;
use serde::{Deserialize, Serialize};

use serde_json::Value;

use super::{Page, PageContext};
use crate::wasm::components::StaticMd;

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
            <StaticMd name="index"/>
            <a href="https://github.com/amiller68" class="icon">
                <img src="/static/icons/github.svg" alt="Github" class="icon"/>
            </a>
            <a href="https://twitter.com/lord_krondor" class="icon">
                <img src="/static/icons/twitter.svg" alt="Twitter" class="icon"/>
            </a>
            <a href="mailto:al@krondor.org" class="icon">
                <img src="/static/icons/email.svg" alt="Email" class="icon"/>
            </a>
            <a href="tg://resolve?domain=lord_krondor" class="icon">
                <img src="/static/icons/telegram.svg" alt="Telegram" class="icon"/>
            </a>
        }
        .into_view()
    }
}

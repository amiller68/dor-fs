use leptos::*;

use crate::wasm::utils::{markdown_to_html, origin_url};

#[component]
pub fn StaticMd(name: &'static str) -> impl IntoView {
    let index_content = create_resource(
        || (),
        move |_| async move {
            let origin = origin_url();
            let index_url = format!("{}/static/{}.md", origin, name);
            let content = reqwest::get(index_url)
                .await
                .expect("text")
                .text()
                .await
                .expect("text");
            let content = markdown_to_html(content);
            content
        },
    );
    view! {
        <div class="prose max-w-none" inner_html=index_content/>
    }
}

use leptos::*;
use leptos_use::use_event_listener;

use serde::{Deserialize, Serialize};

use crate::types::{Item, Render};
use crate::utils::web::{fix_src, get_item_text, get_item_url, markdown_to_html};
use crate::web::components::{ItemRow, ItemRowTable};

use super::{Page, PageContext};

#[derive(Serialize, Deserialize, Clone)]
pub struct ItemsPage(PageContext);

impl Page for ItemsPage {
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

impl IntoView for ItemsPage {
    fn into_view(self) -> View {
        let items: leptos::RwSignal<Vec<ItemRow>> = create_rw_signal(
            self.0
                .manifest()
                .items
                .iter()
                .map(|item| {
                    let item_row: ItemRow = item.into();
                    item_row
                })
                .collect(),
        );
        let item = create_rw_signal(
            self.0
                .manifest()
                .item(&self.0.query().clone().unwrap_or_default()),
        );
        let page_view = create_rw_signal(None);
        let _render_view = create_resource(
            || (),
            move |_| async move {
                match item.get() {
                    None => {
                        gloo::console::log!("No item found");
                        page_view.set(Some(view! {
                            <div class="text-center top-0 left-0">
                                <p><strong>"this is just a jumble of stuff i got. i should really organize it but for now ur gonna have to explore ;)"</strong></p>
                                <ItemRowTable items=items/>
                            </div>
                        }.into_view()));
                    }
                    Some(item) => {
                        let iv = render_item_view(&item).await;
                        page_view.set(Some(iv));
                    }
                }
            },
        );

        view! {
            <div>
                {move || match page_view.get() {
                    None => view! { <p>"Loading..."</p> }.into_view(),
                    Some(pv) => pv.into_view()
                }}
            </div>
        }
        .into_view()
    }
}

async fn render_item_view(item: &Item) -> View {
    let content = match item.render() {
        Render::Markdown => {
            let text = get_item_text(item.name()).await.expect("text");
            let markdown = markdown_to_html(text);
            let markdown = fix_src(markdown);
            view! { <div class="prose max-w-none" inner_html=markdown/> }
        }
        Render::Html => {
            let url = get_item_url(item.name()).expect("url");
            view! {
                <div class="iframe-container">
                    <iframe
                        scrolling="no"
                        src={url}
                    ></iframe>
                </div>
            }
        }
        Render::Jpg => {
            let url = get_item_url(item.name()).expect("url");
            let html = format!(r#"<img src="{url}"/>"#, url = url);
            view! { <div class="prose max-w-none"> {html} </div> }
        }
        Render::Mp3 => {
            let url = get_item_url(item.name()).expect("url");
            let audio_ref: NodeRef<html::Audio> = create_node_ref::<html::Audio>();
            let button_ref: NodeRef<html::Button> = create_node_ref::<html::Button>();
            let slider_ref: NodeRef<html::Input> = create_node_ref::<html::Input>();

            audio_ref.on_load(move |_| {
                button_ref.on_load(move |_| {
                    let audio = audio_ref.get().expect("audio");
                    let button = button_ref.get().expect("button");
                    button.set_inner_text("▶");
                    let _ = use_event_listener(button_ref, leptos::ev::click, move |_| {
                        if audio.paused() {
                            let _ = audio.play().expect("play");
                            button.set_inner_text("❚❚");
                        } else {
                            audio.pause().expect("pause");
                            button.set_inner_text("▶");
                        }
                    });
                });
                slider_ref.on_load(move |_| {
                    let slider = slider_ref.get().expect("slider");
                    let audio = audio_ref.get().expect("audio");
                    slider.set_type("range");
                    slider.set_min("0");
                    slider.set_value("0");
                    slider.set_step("any");
                    slider.set_max(&audio.duration().to_string());
                    let _ = use_event_listener(slider_ref, leptos::ev::input, move |_| {
                        let slider = slider_ref.get().expect("slider");
                        let audio = audio_ref.get().expect("audio");
                        audio.set_current_time(slider.value().parse().expect("parse"));
                    });
                    let _ = use_event_listener(audio_ref, leptos::ev::timeupdate, move |_| {
                        let slider = slider_ref.get().expect("slider");
                        let audio = audio_ref.get().expect("audio");
                        slider.set_value(&audio.current_time().to_string());
                    });
                });
            });

            view! {
                <div class="flex flex-col items-center justify-center space-y-4">
                    <audio
                        class="hidden"
                        node_ref=audio_ref
                        src={url}
                    />
                    <div class="flex items-center space-x-2">
                        <button
                            class="bg-teal-500 hover:bg-teal-600 font-bold py-2 px-4 rounded-full"
                            node_ref=button_ref
                        />
                    </div>
                    <input 
                        type="range" 
                        class="w-full h-2 bg-teal-500 rounded-lg appearance-none cursor-pointer" 
                        node_ref=slider_ref
                    />
                </div>
            }
        }
        Render::Blank => {
            view! { <div class="prose max-w-none"> </div> }
        }
    };
    view! {
        <div>
            <h1>{item.title().to_string()}</h1>
            <p><em>{item.description().to_string()}</em></p>
            <p><strong>{item.date().to_string()}</strong></p>
            {content}
        </div>
    }
    .into_view()
}

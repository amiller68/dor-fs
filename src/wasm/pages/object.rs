use std::path::PathBuf;
use std::str::FromStr;

use cid::Cid;
use leptos::*;
use leptos_use::use_event_listener;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::types::schema::Schemas;
use crate::types::{Manifest, Object};
use crate::wasm::utils::{gateway_url, markdown_to_html, object_url};

use super::{Page, PageContext};

#[derive(Clone, Serialize, Deserialize)]
pub struct ObjectPage(PageContext);

impl Page for ObjectPage {
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

impl IntoView for ObjectPage {
    fn into_view(self) -> View {
        let manifest = self.ctx().manifest();
        let query_str = match self.ctx().query() {
            Some(qs) => qs,
            None => {
                return view! {
                    <div>
                        <p>
                            "Oh no! It looks like you're missing a cid to query for!"
                        </p>
                    </div>
                }
                .into_view()
            }
        };

        let cid = match Cid::from_str(query_str) {
            Ok(cid) => cid,
            Err(_) => return view! {
                <div>
                    <p>
                        "Oh no! You're query specified in invalid Cid! Any valid string representation will do!"
                    </p>
                </div>
            }.into_view()
        };

        let manifest_for_path_object = manifest.clone();
        let manifest_for_object_render = manifest_for_path_object.clone();

        let path_object = create_rw_signal({
            move || {
                manifest_for_path_object
                    .object_by_cid(&cid)
                    .map(|(_path, object)| (_path.clone(), object.clone()))
            }
        });
        let page_view = create_rw_signal(None);
        let manifest_clone_for_async = manifest_for_object_render.clone();

        let _render_view = create_resource(
            || (),
            move |_| {
                let manifest = manifest_clone_for_async.clone();
                async move {
                    match (path_object.get())() {
                        None => {
                            let url = gateway_url(&cid);
                            web_sys::window()
                                .expect("window")
                                .location()
                                .set_href(&url)
                                .expect("href");
                            page_view.set(Some(view! {<div> </div>}.into_view()));
                        }
                        Some(item) => {
                            let (path, object) = item;
                            let ov = render_object_view(&manifest, &path, &object).await;
                            page_view.set(Some(ov.into_view()));
                        }
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

pub async fn render_object_view(
    manifest: &Manifest,
    path: &PathBuf,
    object: &Object,
) -> impl IntoView {
    let metadata = object.metadata();
    let date = object.updated_at().date_naive().to_string();
    let schema = Schemas::try_from(metadata.clone())
        .map_err(|_| {
            // If this is not an object we know how to render, just
            // forward them to the IPFS gateway
            let url = object_url(object);
            web_sys::window()
                .expect("window")
                .location()
                .set_href(&url)
                .expect("href");
            return view! { <div> </div> }.into_view();
        })
        .expect("valid schema");

    let content = match schema {
        Schemas::Writing(writing) => {
            let html = object_markdown_to_html(manifest, path).await;
            view! {
                <div>
                    <h1 class="text-3xl font-bold italic bg-gray-800 p-2">
                        {writing.title}
                        <div class="text-sm font-normal text-gray-200">
                            <p>Last updated: {date}</p>
                            <p>{writing.description}</p>
                        </div>
                    </h1>
                    <div class="prose max-w-none p-2 md" inner_html=html/>
                </div>
            }
        }
        Schemas::Visual(visual) => {
            let url = object_url(object);
            let html = format!(r#"<img src="{url}"/>"#, url = url);
            view! {
                <div>
                    <h1 class="text-3xl font-bold italic bg-gray-800 p-2">
                        {visual.title}
                        <div class="text-sm font-normal text-gray-200">
                            <p>Last updated: {date}</p>
                            <p>{visual.location}, {visual.medium}</p>
                        </div>
                    </h1>
                    <div class="prose max-w-none p-10" inner_html=html/>
                </div>
            }
        }
        Schemas::Audio(audio_obj) => {
            let url = object_url(object);
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
                <div>
                    <h1 class="text-3xl font-bold italic bg-gray-800 p-2">
                        {audio_obj.title}
                        <div class="text-sm font-normal text-gray-200">
                            <p>Last updated: {date}</p>
                            <p>{audio_obj.project.to_string()}</p>
                        </div>
                    </h1>
                    <div class="flex flex-col items-center justify-center space-y-4">
                        <audio
                            class="hidden"
                            node_ref=audio_ref
                            src={url}
                        />
                        <div class="flex items-center space-x-2">
                            <button
                                class="bg-gray-500 hover:bg-gray-600 font-bold py-2 px-4 rounded-full"
                                node_ref=button_ref
                            />
                        </div>
                        <input
                            type="range"
                            class="w-full h-2 bg-gray-500 rounded-lg appearance-none cursor-pointer"
                            node_ref=slider_ref
                        />
                    </div>
                </div>
            }
        }
    };
    view! {
        <div>
            {content}
        </div>
    }
    .into_view()
}

/// Replace all links to assets within the filesystem with links to the IPFS gateway
/// This is a hack to get around the fact that we don't have a way to resolve links
/// within our Manifest yet, since we decide we HATE unix-fs
async fn object_markdown_to_html(manifest: &Manifest, object_path: &PathBuf) -> String {
    let objects = manifest.objects();
    let object = objects.get(object_path).unwrap();
    let url = object_url(object);
    let object_content = reqwest::get(url)
        .await
        .expect("object_content")
        .text()
        .await
        .expect("object_content");
    let base_path = object_path.parent().expect("base_path");

    // First find all occurences of ./ and retrieve the object url from the manifest
    // Then replace the ./ with a link to the Ipfs gateway
    let re = Regex::new(r#"src="./([^"]+)"#).unwrap();
    let mut result = object_content.clone();

    for caps in re.captures_iter(&object_content) {
        if let Some(cap) = caps.get(1) {
            let path = PathBuf::from(cap.as_str());
            let path = base_path.join(path);
            let normalized_path = normalize_path(&path);
            if let Some(object) = objects.get(&normalized_path) {
                let url = object_url(object);

                let old = format!(r#"src="./{}""#, cap.as_str());
                let new = format!(r#"src="{}""#, url);
                result = result.replace(&old, &new);
            }
        }
    }

    let html = markdown_to_html(result);
    html
}

fn normalize_path(path: &PathBuf) -> PathBuf {
    let mut normalized_path = PathBuf::new();
    for component in path.components() {
        match component {
            std::path::Component::ParentDir => { normalized_path.pop(); },
            _ => { normalized_path.push(component); }
        }
    }
    normalized_path
}

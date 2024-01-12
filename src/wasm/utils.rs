use std::path::PathBuf;

use cid::Cid;
use pulldown_cmark::{html, Options, Parser};

use crate::types::Object;

use super::env::APP_IPFS_GATEWAY_URL;

pub fn origin_url() -> String {
    web_sys::window().expect("window").origin()
}

pub fn gateway_url(cid: &Cid) -> String {
    format!("{}/ipfs/{}", APP_IPFS_GATEWAY_URL, cid.to_string())
}

pub fn object_url(object: &Object) -> String {
    gateway_url(object.cid())
}

pub fn markdown_to_html(content: String) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(&content, options);
    let mut html = String::new();
    html::push_html(&mut html, parser);
    html
}

use pulldown_cmark::{html, Options, Parser};

pub fn origin_url() -> String {
    web_sys::window().expect("window").origin()
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

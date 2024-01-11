use leptos::component;
use leptos::view;
use leptos::IntoView;

use super::pages::InternalRouter;

use super::env::{APP_NAME, APP_VERSION};

pub struct App;

impl App {
    pub fn run() {
        console_error_panic_hook::set_once();
        leptos::mount_to_body(InternalRouter);
    }
}

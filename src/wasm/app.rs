use super::pages::InternalRouter;

pub struct App;

impl App {
    pub fn run() {
        console_error_panic_hook::set_once();
        leptos::mount_to_body(InternalRouter);
    }
}

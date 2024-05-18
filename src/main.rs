mod app;
mod pkg;

use app::*;
use leptos::*;

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    let new_jumble = pkg::Library::new_jumble();
    println!(" the jumbled word is {:#?}", new_jumble);
    logging::log!("csr mode - mounting to body");

    mount_to_body(|| {
        view! { <App/> }
    });
}

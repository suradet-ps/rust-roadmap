use leptos::prelude::*;
use rust_roadmap::app::App;

fn main() {
  console_error_panic_hook::set_once();
  mount_to_body(|| view! { <App/> })
}

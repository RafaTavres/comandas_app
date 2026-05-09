use leptos::mount::mount_to_body;
pub mod app;
pub mod components;
pub mod context;
pub mod pages;
pub mod shared;

fn main() {
    mount_to_body(app::App);
}

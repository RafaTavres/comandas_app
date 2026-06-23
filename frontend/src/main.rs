use leptos::mount::mount_to_body;
pub mod app;
pub mod components;
pub mod constants;
pub mod context;
pub mod services;
pub mod pages;
pub mod utils;
pub mod routes;
pub mod hooks;

fn main() {
    mount_to_body(app::App);
}

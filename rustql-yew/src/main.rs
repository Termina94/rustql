#![recursion_limit = "256"]
use app::App;
mod app;

fn main() {
    yew::start_app::<App>();
}

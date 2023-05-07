//? Runs the Yew App. Here, modules are stated to be brought into the module tree.

mod app;
use app::App;
mod components;

fn main() {
    yew::Renderer::<App>::new().render();
}

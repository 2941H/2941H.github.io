use sycamore::prelude::*;

#[component]
fn App() -> View {
    view! {
        div {
            h1(class="text-2xl font-medium") { "Hello, world!" }
            p { "This is my first Sycamore app" }
        }
    }
}

pub fn html() -> String {
    // basic templating
    let content = sycamore::render_to_string(App);
    include_str!("../index.html").replace("{content}", &content)
}

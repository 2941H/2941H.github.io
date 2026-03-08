use sycamore::prelude::*;

mod about;
mod r#box;
mod operation;
mod profile_card;
mod section;
mod title;
mod vex;

#[component]
fn App() -> View {
    view! {
        div(class="snap-y snap-mandatory overflow-y-scroll h-screen") {
            title::Title()
            vex::Vex()
            about::About()
            operation::Operation()
        }
    }
}

pub fn html() -> String {
    // basic templating
    let content = sycamore::render_to_string(App);
    include_str!("../index.html").replace("{content}", &content)
}

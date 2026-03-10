use sycamore::prelude::*;

mod about;
mod achievements;
mod r#box;
mod donate;
mod operation;
mod profile_card;
mod section;
mod title;
mod vex;
mod worlds;

#[component]
fn App() -> View {
    view! {
        div(class="snap-y snap-mandatory overflow-y-scroll h-screen") {
            title::Title()
            worlds::Worlds()
            vex::Vex()
            about::About()
            achievements::Achievements()
            donate::Donate()
            operation::Operation()
        }
    }
}

pub fn html() -> String {
    // basic templating
    let content = sycamore::render_to_string(App);
    include_str!("../index.html").replace("{content}", &content)
}

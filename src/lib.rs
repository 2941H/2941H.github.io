use sycamore::prelude::*;

mod r#box;

#[component]
fn App() -> View {
    view! {
        section(class="h-screen w-full") {
            img(src="./assets/robot_edited.webp", class="absolute top-0 left-0 w-full h-full object-cover")
            r#box::Box(class="absolute top-0 left-0 rounded-tl-none rounded-tr-none rounded-bl-none") {
                h1(class="text-9xl font-mono text-white") {
                    span {"2941H"}
                }
                h2(class="text-3xl text-gray-200 font-mono flex justify-between") {
                    span { "M" }
                    span { "e" }
                    span { "c" }
                    span { "h" }
                    span { "a" }
                    span { " " }
                    span { "S" }
                    span { "q" }
                    span { "u" }
                    span { "a" }
                    span { "d" }
                }
            }
            r#box::Box(class="absolute right-0 top-1/2 -translate-y-1/2 rounded-tr-none rounded-br-none w-1/5 text-justify") {
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aliquam eu felis venenatis, pretium enim id, placerat ipsum. Duis quam tortor, commodo quis blandit in, accumsan eget libero. Donec rhoncus lobortis mauris, vel pulvinar lacus. Etiam posuere odio non cursus ultricies. Nunc rutrum rutrum felis, eget cursus leo pretium id. Nulla ornare nulla odio, nec sodales tortor rhoncus ut. Vestibulum suscipit est mauris, ac bibendum diam facilisis id. Cras tempus enim at libero auctor hendrerit. Nulla ornare molestie pulvinar."
            }
        }
    }
}

pub fn html() -> String {
    // basic templating
    let content = sycamore::render_to_string(App);
    include_str!("../index.html").replace("{content}", &content)
}

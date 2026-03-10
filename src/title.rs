use crate::r#box;
use crate::section;
use sycamore::prelude::*;

#[component]
pub(crate) fn Title() -> View {
    view! {
        section::Section(breakpoint="lg") {
            r#box::Box(class="lg:z-1 text-center w-full lg:w-auto lg:absolute lg:top-0 lg:left-0 rounded-tl-none rounded-tr-none rounded-bl-none rounded-br-none lg:rounded-br-lg") {
                h1(class="text-8xl xl:text-9xl font-mono text-white") {
                    span {"2941H"}
                }
                h2(class="text-2xl xl:text-3xl text-gray-200 font-mono flex justify-between") {
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
            r#box::Box(class="lg:z-1 m-8 lg:m-0 lg:absolute lg:right-0 lg:top-1/2 lg:-translate-y-1/2  lg:rounded-tr-none lg:rounded-br-none lg:w-3/10 text-justify text-white text-xl") {
                // this is toml because of nix source filtering reasons
                (include_str!("team_overview.toml"))
            }
            // eager loading bc this is the title page
            img(loading="eager", src="./assets/robot_edited.webp", class="h-screen snap-start object-center  lg:translate-x-0 lg:snap-align-none lg:absolute inset-0 lg:h-full lg:w-full lg:object-right object-cover", alt="Team 2941H VEX robot on the field.")
        }
    }
}

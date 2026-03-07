use crate::r#box;
use crate::section;
use sycamore::prelude::*;

#[component]
pub(crate) fn Vex() -> View {
    view! {
        section::Section {
            img(src="https://placehold.co/600x400/darkgray/orange?text=Placeholder", class="w-auto absolute inset-0 h-full w-full object-center object-cover", alt="Placeholder image.")
            r#box::Box(class="lg:absolute w-full lg:w-auto lg:top-0 lg:left-0 lg:right-auto rounded-tl-none rounded-tr-none rounded-bl-none rounded-br-none lg:rounded-br-lg") {
                h1(class="text-[11vw] text-center lg:text-left lg:text-7xl font-mono text-white") {
                    "What is "
                    img(src="./assets/vex_robotics_logo.png", class="h-[1.2em] inline")
                    "?"
                }
            }
            r#box::Box(class="absolute m-8 bottom-0 lg:m-0 lg:left-8 lg:bottom-8 lg:w-100 text-white text-xl") {
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aliquam eu felis venenatis, pretium enim id, placerat ipsum. Duis quam tortor, commodo quis blandit in, accumsan eget libero. Donec rhoncus lobortis mauris, vel pulvinar lacus. Etiam posuere odio non cursus ultricies. Nunc rutrum rutrum felis, eget cursus leo pretium id. Nulla ornare nulla odio, nec sodales tortor rhoncus ut. Vestibulum suscipit est mauris, ac bibendum diam facilisis id."
            }
        }
    }
}

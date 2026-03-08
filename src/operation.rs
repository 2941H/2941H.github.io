use crate::r#box;
use crate::section;
use sycamore::prelude::*;

#[component]
pub(crate) fn Operation() -> View {
    view! {
        section::Section {
            r#box::Box(class="lg:absolute w-full lg:w-fit lg:right-auto rounded-tl-none rounded-tr-none rounded-bl-none rounded-br-none lg:rounded-br-lg") {
                h1(class="text-[11vw] text-center lg:text-left lg:text-7xl font-mono text-white") {
                    "Design Process"
                }
            }
        }
    }
}

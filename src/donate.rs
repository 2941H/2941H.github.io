use crate::r#box;
use crate::section;
use sycamore::prelude::*;

#[component]
pub(crate) fn Donate() -> View {
    view! {
        section::Section(breakpoint="lg") {
            r#box::Box(class="lg:absolute w-full lg:w-auto lg:top-0 lg:left-0 lg:right-auto rounded-tl-none rounded-tr-none rounded-bl-none rounded-br-none lg:rounded-br-lg") {
                h1(class="text-[11vw] text-center lg:text-left lg:text-7xl font-mono text-white") {
                    "Sponsor Us!"
                }
            }
            r#box::Box(class="mx-auto mt-8 lg:m-8 lg:absolute lg:right-0 lg:top-1/2 lg:-translate-y-1/2 w-fit h-fit") {
                iframe(loading="lazy", src="https://givealittle.co.nz/widget/light-skyscraper/cause/oats-to-vex-worlds", class="w-70 h-130")
                span(class="sr-only") {"give a little"}
                img(src="./assets/givealittle_logo.webp", class="w-40 mt-8 mx-auto", alt="", aria-hidden="true")
            }
            r#box::Box(class="lg:absolute m-8 lg:bottom-0 lg:m-0 lg:left-8 lg:bottom-8 lg:w-130 text-white text-xl") {
                p {"Getting a team of students from Tauranga to St. Louis is expensive. The cost is approximately $7000 NZD a student including a $1800 USD registration. Every dollar brings these students one step closer to the world stage."}
                p {"Donations will go directly to helping students fundraise and pay for:"}
                ul(class="list-disc pl-5") {
                    li {"Return flights from New Zealand to St. Louis"}
                    li {" Official tournament accommodation"}
                    li {" Registration and competition entry fees"}
                    li {" Robot equipment costs"}
                }
            }
        }
    }
}

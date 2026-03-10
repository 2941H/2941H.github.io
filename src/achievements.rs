use crate::r#box;
use crate::section;
use sycamore::prelude::*;

#[component]
pub(crate) fn Achievements() -> View {
    view! {
        section::Section {
            img(loading="lazy", src="./assets/awards.webp", class="absolute inset-0 h-full w-full object-top object-cover", alt="Placeholder image.")
            r#box::Box(class="bg-gray-900/40! lg:absolute w-full lg:w-auto lg:top-0 lg:left-0 lg:right-auto rounded-tl-none rounded-tr-none rounded-bl-none rounded-br-none lg:rounded-br-lg") {
                h1(class="text-[11vw] text-center lg:text-left lg:text-7xl font-mono text-white") {
                    "Achievements"
                }
            }
            r#box::Box(class="bg-gray-900/40! absolute m-8 bottom-0 text-justify text-white text-xl") {
                p {"This season we had our best season yet and achieved several major milestones. Some of our proudest ones are: "}
                ul(class="list-disc pl-5") {
                    li {"Qualifying for the World Championship"}
                    li {"Winning the tournament finalist and the 1st judges award"}
                    li {"Ending the New Zealand season with a 38 win 10 loss record for a 79.2% winrate"}
                    li {"Reaching the finals at all 4 of the tournaments we attended and winning 2 of them "}
                }
            }
        }
    }
}

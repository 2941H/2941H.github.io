use crate::r#box;
use crate::section;
use sycamore::prelude::*;

#[component]
pub(crate) fn Worlds() -> View {
    view! {
        section::Section(breakpoint="lg") {
            img(loading="lazy", src="./assets/st_louis.webp", class="w-auto absolute inset-0 h-full w-full object-center object-cover", alt="Placeholder image.")
            r#box::Box(class="bg-white/30 w-full  rounded-tl-none rounded-tr-none rounded-bl-none rounded-br-none") {
                h1(class="text-[11vw] text-center lg:text-left lg:text-7xl font-mono text-white") {
                    "We Qualified for the World Championships"
                }
            }
            r#box::Box(class="lg:absolute lg:bottom-0 m-8 text-justify text-white text-xl") {
                p {"The 2026 VEX Robotics World Championship represents the pinnacle of the competitive robotics calendar for teams, bringing together thousands of the world's top qualifying students in St. Louis, Missouri, USA  at the America's Center Convention Complex. Taking place the 21st-24th April, this multi-day event showcases the best student-designed robots from over 60 countries, making it one of the largest secondary school robotics competitions. Teams competing at Worlds have earned their place through strong performances at local, regional, and national championships throughout the season. "}
                p {"Qualifying for the World Championship is a prestigious achievement that recognizes months of dedication, innovation, and competitive excellence. For students, it represents the highlight of their robotics career, and a chance to test their skills against the very best in the world before potentially transitioning to university studies or careers. The event celebrates not just winning but also design excellence, sportsmanship, and strategy."}
            }
        }
    }
}

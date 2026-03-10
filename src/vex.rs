use crate::r#box;
use crate::section;
use sycamore::prelude::*;

#[component]
pub(crate) fn Vex() -> View {
    view! {
        section::Section(breakpoint="lg") {
            img(src="./assets/nationals.webp", class="w-auto absolute inset-0 h-full w-full object-center object-cover", alt="Placeholder image.")
            r#box::Box(class="bg-white/30 lg:absolute w-full lg:w-auto lg:top-0 lg:left-0 lg:right-auto rounded-tl-none rounded-tr-none rounded-bl-none rounded-br-none lg:rounded-br-lg") {
                h1(class="text-[11vw] text-center lg:text-left lg:text-7xl font-mono text-white") {
                    "What is "
                    span(class="sr-only") {"vex robotics"}
                    img(src="./assets/vex_robotics_logo.png", class="h-[1.2em] inline", alt="", aria-hidden="true")
                    "?"
                }
            }
            r#box::Box(class="lg:absolute m-8 lg:bottom-0 text-justify text-white text-xl") {
                p {"The VEX V5 Robotics Competition (VRC) is one of the world's largest and most prestigious competitive robotics programs for secondary schools. VRC engages millions of students across 60+ countries, making it a truly global STEM initiative. The competition challenges student teams to design, build, program, and drive robots to compete in annual game challenges."}
                p {"Each VEX Robotics season features a unique game challenge that is revealed globally at the start of the competition year. Matches are played on a 12-foot by 12-foot field, with two alliances (red and blue) competing against each other, each alliance consisting of two teams working collaboratively. Every match is divided into two distinct periods: a 15-second autonomous period where robots operate entirely on pre-programmed code without any driver input, followed by a 1-minute 45-second driver-controlled period where students operate their robots using handheld controllers. Points are scored through various game-specific tasks, which typically involve manipulating game objects such as balls, rings, cubes, or other elements into goals, zones, or stacking configurations. Additional points can be earned through autonomous bonuses and end-game objectives. Robots must adhere to strict size constraints (typically an 18-inch cube at the start of the match) and can only be built using official VEX V5 components."}
            }
        }
    }
}

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
            r#box::Box(class="ml-auto w-fit h-fit rounded-tl-none rounded-tr-none rounded-br-none") {
                h2(class="text-3xl") {"Our Engineering Notebook"}
                iframe(src="./assets/notebook.pdf", class="w-90 aspect-210/297", title="engineering notebook")
            }
            r#box::Box(class="lg:absolute lg:left-0 lg:bottom-0 m-8 lg:w-6/10 text-justify text-white text-xl") {
                p {"VEX robotics follows a very similar design process to engineering applications and any competitive event. Following the trend of wide iterations and creative design in the early season, where teams around the world work together to find mechanisms and strategies that are effective. Then flowing into mid-seasons, where general principles are defined but applications are widely varied. Then, finally, into the late season, where there are defined “metas” and robot designs, and the focus shifts towards optimisation and finding new mechanisms that give the slightest edge. VEX has a unique community of sharing knowledge and robots functionality to the whole world in order to further the competitions as a whole, if you were to google vex robotics explanation you'd be able to find building guides as well as breakdowns of peoples late season bots."}
                p {"Our team understands this and follows a multi design cycle philosophy, we will typically go through three to four full design cycles a year with each usually consisting of fully CADing the robot on fusion 360, and then a full rebuild. This process is documented in our engineering notebook which consists of all plans and ideas that we have throughout the season along with building progression and process."}
            }
        }
    }
}

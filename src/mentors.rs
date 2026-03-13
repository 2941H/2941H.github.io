use crate::about::mk_cards;
use crate::r#box;
use crate::profile_card;
use crate::section;
use sycamore::prelude::*;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct People {
    people: Vec<profile_card::Person>,
}

#[component]
pub(crate) fn Mentors() -> View {
    // read mentors.toml file
    let people: People =
        toml::from_str(include_str!("mentors.toml")).expect("failed to deserialize mentors.toml");

    // build cards from the mentors
    let cards = mk_cards(people.people.into_iter());

    view! {
        // I'm not sure if this is bad
        section::Section(breakpoint="no-breakpoint") {
            r#box::Box(class="z-1 w-full sticky top-0 lg:w-fit lg:right-auto rounded-tl-none rounded-tr-none rounded-bl-none rounded-br-none lg:rounded-br-lg") {
                h1(class="text-[11vw] text-center lg:text-left lg:text-7xl font-mono text-white") {
                    "About Oats Robotics"
                }
            }
            div(class="flex w-full") {
                div(class="center-content mx-auto") {
                    (cards)
                }
            }
        }
    }
}

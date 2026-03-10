use crate::r#box;
use crate::profile_card;
use crate::section;
use sycamore::prelude::*;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct People {
    people: Vec<profile_card::Person>,
}

#[component]
pub(crate) fn About() -> View {
    // read people.toml file
    let people: People =
        toml::from_str(include_str!("people.toml")).expect("failed to deserialize people.toml");

    // build cards from the people
    let cards = people
        .people
        .into_iter()
        .map(|person| {
            view! {
                div(class="w-fit p-8 snap-center") {
                    profile_card::Profile(person=person)
                }
            }
        })
        .fold(View::new(), |a, b| view! {(a) (b)});

    view! {
        // I'm not sure if this is bad
        section::Section(breakpoint="no-breakpoint") {
            r#box::Box(class="z-1 w-full sticky top-0 lg:w-fit lg:right-auto rounded-tl-none rounded-tr-none rounded-bl-none rounded-br-none lg:rounded-br-lg") {
                h1(class="text-[11vw] text-center lg:text-left lg:text-7xl font-mono text-white") {
                    "About Us"
                }
            }
            div(class="flex w-full") {
                div(class="snap-y snap-mandatory center-content mx-auto") {
                    (cards)
                }
            }
        }
    }
}

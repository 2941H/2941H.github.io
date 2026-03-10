use crate::r#box;
use sycamore::prelude::*;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct Person {
    photo: String,
    name: String,
    quote: Option<String>,
}

#[component(inline_props)]
pub(crate) fn Profile<'a>(person: Person) -> View {
    // quote elements
    let quote = person
        .quote
        .map(|q| {
            view! {
                div(class="text-xl text-justify text-white") {
                    (q)
                }
            }
        })
        .unwrap_or_default();

    view! {
        r#box::Box(class="block lg:flex") {
            figure(class="mb-4 lg:mb-0 lg:mr-4") {
                div(class="w-50 w-50 rounded-full overflow-hidden mx-auto") {
                    img(loading="lazy", src=person.photo, class="w-full h-full object-cover")
                }
                figcaption(class="text-2xl text-center text-white") {
                    (person.name)
                }
            }
            (quote)
        }
    }
}

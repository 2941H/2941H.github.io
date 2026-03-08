use crate::r#box;
use sycamore::prelude::*;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct Person {
    photo: String,
    name: String,
    quote: Option<String>,
}

#[component(inline_props)]
pub(crate) fn Profile<'a>(class: Option<&'a str>, person: Person) -> View {
    // quote elements
    let quote = person
        .quote
        .map(|q| {
            view! {
                div(class="text-xl text-white") {
                    (q)
                }
            }
        })
        .unwrap_or_default();

    view! {
        r#box::Box(class=&("flex".to_string() + class.unwrap_or_default())) {
            figure(class="w-fit h-fit mr-4") {
                img(src=person.photo, class="rounded-full w-30 h-30 object-cover")
                figcaption(class="text-xl text-center text-white") {
                    (person.name)
                }
            }
            (quote)
        }
    }
}

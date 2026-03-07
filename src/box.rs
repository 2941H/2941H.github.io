use sycamore::prelude::*;

/// small wrapper around a div for "box" styling
#[component(inline_props)]
pub(crate) fn Box<'a>(
    #[prop(attributes(html, div))] attributes: Attributes,
    class: Option<&'a str>,
    children: Children,
) -> View {
    let class = class.unwrap_or_default().to_string()
        + " bg-white/10 rounded-lg p-8 outline-2 outline-white/40 backdrop-blur-xl";
    view! {
        div(class=class, ..attributes) {
            (children)
        }
    }
}

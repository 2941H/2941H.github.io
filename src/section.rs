use sycamore::prelude::*;

#[component(inline_props)]
pub(crate) fn Section(children: Children, breakpoint: Option<&'static str>) -> View {
    let classes = match breakpoint {
        Some(bp) => format!("{bp}:h-screen w-full relative snap-start overflow-hidden"),
        None => "h-screen w-full relative snap-start overflow-hidden".to_string(),
    };

    view! {
        section(class=classes) {
            (children)
        }
    }
}

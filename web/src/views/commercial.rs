use dioxus::prelude::*;

const COMMERCIAL_FAVICON: &str = "https://image-resizer.simonpforster.com/oliviazuo-portfolio/elements/element-cyan.png";

#[component]
pub fn Commercial() -> Element {
    rsx! {
        document::Title { "OLIVIA ZUO - COMMERCIAL" }
        document::Link { rel: "icon", type: "image/x-icon", href: COMMERCIAL_FAVICON }
    }
}

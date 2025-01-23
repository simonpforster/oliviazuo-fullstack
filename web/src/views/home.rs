use dioxus::prelude::*;

const HOME_FAVICON: &str = "https://image-resizer.simonpforster.com/oliviazuo-portfolio/elements/element-pink-ico.png";

#[component]
pub fn Home() -> Element {
    rsx! {
        document::Title { "OLIVIA ZUO" }
        document::Link { rel: "icon", type: "image/x-icon", href: HOME_FAVICON }
    }
}

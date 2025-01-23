use dioxus::prelude::*;

const PERSONAL_FAVICON: &str = "https://image-resizer.simonpforster.com/oliviazuo-portfolio/elements/element-blue.png";


#[component]
pub fn Personal() -> Element {
    rsx! {
        document::Title { "OLIVIA ZUO - PERSONAL" }
        document::Link { rel: "icon", type: "image/x-icon", href: PERSONAL_FAVICON }
    }
}

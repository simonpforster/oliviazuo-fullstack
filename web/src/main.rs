use std::fmt::{Display, Formatter};
use dioxus::prelude::*;

use ui::Navbar;
use views::{Home, Commercial, Personal, Shop};

mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(WebNavbar)]
    #[route("/")]
    Home {},
    #[route("/personal")]
    Personal {},
    #[route("/commercial")]
    Commercial {},
    #[route("/shop")]
    Shop {},
}

const MAIN_CSS: Asset = asset!("/assets/main.css");
const FONT_CSS: Asset = asset!("/assets/font.woff2");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Router::<Route> {}
    }
}

/// A web-specific Router around the shared `Navbar` component
/// which allows us to use the web-specific `Route` enum.
#[component]
fn WebNavbar() -> Element {
    rsx! {
        Navbar {
            Link {
                id: "home",
                to: Route::Home {},
                "OLIVIA ZUO"
            }
            Link {
                id: "personal",
                to: Route::Personal {},
                "PERSONAL PROJECTS"
            }
            Link {
                id: "commercial",
                to: Route::Commercial {},
                "COMMERCIAL PROJECTS"
            }
            Link {
                id: "shop",
                to: Route::Shop {} ,
                "SHOP"
            }
        }

        Outlet::<Route> {}
    }
}

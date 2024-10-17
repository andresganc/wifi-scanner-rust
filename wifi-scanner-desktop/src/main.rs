#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

// Wifi Scanner
use std::cell::Cell;
use wifiscanner::Wifi;

extern crate wifiscanner;

// Route
#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/wifi_scanner")]
    WifiScanner {},
    #[route("/about_as")]
    AboutAs {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

#[component]
fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

// Main
fn main() {
    use wifiscanner;
    println!("Wifi Scanner: {:?}", wifiscanner::scan());

    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");

    dioxus::launch(App);
}

// WIfiScanner
enum Status {
    NoneFound,
    Scanning,
    Found(Vec<Wifi>),
}

struct StructAppProps {
    sender: Cell<Option<UnboundedSender<Status>>>,
    receiver: Cell<Option<UnboundedReceiver<Status>>>,
}

fn perform_scan() -> Status {
    if let Ok(devices) = wifiscanner::scan() {
        if devices.is_empty() {
            Status::NoneFound
        } else {
            Status::Found(devices)
        }
    } else {
        Status::NoneFound
    }
}

#[component]
fn WifiScanner() -> Element {
    let mut count = use_signal(|| 0);

    use wifiscanner;
    println!("{:?}", wifiscanner::scan());

    rsx! {

        Link { to: Route::Home{}, "Home" }
        Link { to: Route::WifiScanner{}, "Wifi Scanner" }
        Link { to: Route::AboutAs{}, "About As" }
        // Link { to: Route::Blog{id: count()}, "Blog"}

        div {

            display: "flex", justify_content: "center",
            h2 {
                color: "blue",
                "NC - WIFI SCANNER"
            }
        }


        div {
            display: "grid", justify_content: "center",

            table {

                thead {
                    tr {
                        th {"Señal"}
                        th {"Red
    "}
                        th {"Canal"}
                        th {"Seguridad"}
                    }
                }

                tbody {
                    {
                        use wifiscanner;
                        println!("{:?}", wifiscanner::scan());
                    }
                    // p {
                    //     {wifiscanner::scan()}
                    // }
                }
            }
        }

        div {

        }

    }
}

// Blog
#[component]
fn Blog(id: i32) -> Element {
    let mut count = use_signal(|| 0);

    rsx! {

        Link { to: Route::Home{}, "Home" }
        Link { to: Route::WifiScanner{}, "Wifi Scanner"}
        Link { to: Route::AboutAs{}, "About As" }
        // Link { to: Route::Blog{id: count()}, "Blog" }

        div {
            h2 {"Blog post {id}"}
        }
    }
}

// Home
#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {

        Link { to: Route::Home{}, "Home" }
        Link { to: Route::WifiScanner{}, "Wifi Scanner" }
        Link { to: Route::AboutAs{}, "About As" }
        // Link { to: Route::Blog {id: count()}, "Blog" }


        div {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "+" }
            button { onclick: move |_| count -= 1, "-" }
        }
    }
}

#[component]
fn AboutAs() -> Element {
    rsx! {

        Link { to: Route::Home{}, "Home" }
        Link { to: Route::WifiScanner{}, "Wifi Scanner" }
        Link { to: Route::AboutAs{}, "About As" }
        // Link { to: Route::Blog {id: count()}, "Blog" }

        div {
            h2 {
                "About As"
            }
        }
    }
}

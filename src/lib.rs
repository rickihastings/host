#[macro_use]
extern crate horrorshow;
#[macro_use]
extern crate strum_macros;

mod utils;

use horrorshow::{Error, Template};
use host_component::{start, Component, Model};
use wasm_bindgen::prelude::*;
use web_sys::Event;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

static ROOT: RootView = RootView { name: "Ricki" };

#[wasm_bindgen(start)]
pub fn main() {
    start("body", &ROOT)
}

#[repr(u8)]
#[derive(ToString, EnumString)]
enum Msg {
    Increment,
}

#[derive(Copy, Clone)]
struct RootView {
    name: &'static str,
}

impl Model for RootView {
    type Message = Msg;

    fn update(&self, event: &Event, message: Msg) {
        match message {
            Msg::Increment => {
                log!("{:#?}", event);

                // self.name = "Roy"
            }
        }
    }
}

impl Component<Error> for RootView {
    fn render(&self) -> Result<String, Error> {
        (html! {
            article {
                header(class="post-header", onclick=self.create_event(Msg::Increment)) {
                    p : self.name;
                }
                section(class="post-body") : "Body";
            }
        })
        .into_string()
    }
}

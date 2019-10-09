#![feature(proc_macro_hygiene)]

#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

mod utils;

use render::{html, Renderable};
use host_component::{start, Component};
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

#[wasm_bindgen(start)]
pub fn main() {
    start::<RootView>("body", RootViewProps { name: "ricki", color: "red" });
}

// todo
unsafe impl Send for RootView {}
unsafe impl Sync for RootView {}

#[derive(Copy, Clone, Debug)]
struct RootViewProps {
    name: &'static str,
    color: &'static str,
}

#[derive(Copy, Clone, Debug, Primitive)]
enum Message {
    Add = 1
}

#[derive(Copy, Clone, Debug)]
struct RootView {
    name: &'static str,
    color: &'static str,
}

impl Component for RootView {
    type Message = Message;
    type Props = RootViewProps;

    fn new(props: Self::Props) -> Self {
        log!("New comp");

        Self { name: props.name, color: props.color }
    }

    fn update(mut self, event: &Event, message: Message) -> Self {
        match message {
            Message::Add => {
                log!("Update! {:?}", event);
                self.name = "Rick";
                self.color = "blue";

                self
            }
        }
    }
}

impl Renderable for RootView {
    fn render(self) -> String {
        log!("Updated: {}", self.name);

        let style = format!("color: {}", self.color);

        html! {
            <article style={&*style} dataAttr={"test"}>
                <header class={"title"} onclick={self.create_event(&Message::Add)}>
                    <p>{self.name}</p>
                </header>
                <section class={"body"}>
                    {"Body"}
                </section>
            </article>
        }
    }
}

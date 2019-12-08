#![feature(proc_macro_hygiene)]

mod utils;

use rand::prelude::*;

use wasm_bindgen::prelude::*;
// use host_component_macro::*;
use host_component::{html, use_state, Application, Component, IterableNodes, VirtualNode};

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

#[wasm_bindgen]
struct Client {
    app: Application<NewHomeView>,
}

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let app = Application::new("body");

        Self { app }
    }

    #[wasm_bindgen]
    pub fn update(&mut self) {
        self.app.render();
    }
}

struct NewHomeView;

impl Component for NewHomeView {
    fn new() -> Self {
        Self {}
    }

    fn render(&self) -> VirtualNode {
        let (count, mut set_count) = use_state(self, "count", || 0);

        let mut rng = rand::thread_rng();
        let y: f64 = rng.gen();

        log!("first: {}", count);

        html! {
            <div>
                <strong>Hello World</strong><br/>
                <strong>Count: {format!{"{}", count}}</strong><br/>
                <strong>Random: {format!("{}", y)}</strong><br/>
                <button key={count} onclick=move |_event: web_sys::Event| {
                    set_count.set("count", count + 1);
                }>
                    // No need to wrap text in quotation marks (:
                    Click me and check your console
                </button>
            </div>
        }
    }
}

#![feature(proc_macro_hygiene)]

mod utils;

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use host_component::{html, Application, Component, VirtualNode, IterableNodes, topo, use_state, State, RcState};

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
    state: RcState,
    app: Application<HomeView>
}

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut state = Rc::new(RefCell::new(State::new()));
        let app = Application::new(String::from("body"), state.clone());

        Self {
            state,
            app
        }
    }

    #[wasm_bindgen]
    pub fn update(&mut self) {
        self.app.render();
    }
}

struct HomeView {
    state: RcState
}

impl Component<RcState> for HomeView {
    fn new(state: RcState) -> HomeView {
        Self {
            state
        }
    }

    fn render(&self) -> VirtualNode {
        topo::call!({
            // Declare a new state variable which we'll call "count"
            let (count, count_access) = use_state(self.state.clone(), || 0);

            html! {
                <div>
                    <strong>Hello World</strong><br/>
                    <strong>Count: {format!{"{}", count}}</strong><br/>
                    <button onclick=move |_event: web_sys::Event| {
                        log!("Button Clicked!");
                        count_access.set(count + 1);
                    }>
                        // No need to wrap text in quotation marks (:
                        Click me and check your console
                    </button>
                </div>
            }
        })
    }
}

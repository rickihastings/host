#![feature(proc_macro_hygiene)]

mod reducer;
mod utils;
mod component;

use crate::reducer::{Action, DefaultState, DefaultReducer};
use crate::component::NewHomeView;

use host_component::prelude::*;
use wasm_bindgen::prelude::*;

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
pub struct Client {
    #[allow(dead_code)]
    app: Application,
}

#[wasm_bindgen]
impl Client {
    #[allow(dead_code)]
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        utils::set_panic_hook();

        let store = DefaultReducer::new(DefaultState { count: 0 });

        let component = Box::new(NewHomeView::new());
        let mut app = Application::new("body", component);

        app.inject_store::<DefaultReducer, Action, DefaultState>(store);

        app.mount();

        Self { app }
    }

    #[allow(dead_code)]
    #[wasm_bindgen]
    pub fn update(&mut self) {
        self.app.render();
    }
}

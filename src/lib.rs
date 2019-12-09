#![feature(proc_macro_hygiene)]

mod utils;

use rand::prelude::*;

use host_component::*;
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
struct Client {
    app: Application,
}

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let component = Box::new(NewHomeView::new());
        let app = Application::new("body", component);

        Self { app }
    }

    #[wasm_bindgen]
    pub fn update(&mut self) {
        self.app.render();
    }
}

#[derive(Copy, Clone)]
struct NewHomeView {
    count: u8,
}

impl NewHomeView {
    fn new() -> Self {
        Self {
            count: 0
        }
    }

    fn update(&mut self) {
        self.count += 1;
    }
}

impl Component for NewHomeView {
    fn render(&self) -> VirtualNode {
        let mut rng = rand::thread_rng();
        let y: f64 = rng.gen();

        log!("first: {}", self.count);

        let children = if self.count == 5 {
            component! {
                ChildView {}
            }
        } else {
            html! {
                <div>huh</div>
            }
        };

        html! {
            <div>
                <strong>Hello World</strong><br/>
                <strong>Count: {format!{"{}", self.count}}</strong><br/>
                <strong>Random: {format!("{}", y)}</strong><br/>
                <button key={self.count} onclick=move |_event: web_sys::Event| {
                    get_component_mut::<NewHomeView, Fn(&mut NewHomeView) -> ()>(self.id(), |comp| {
                        comp.update();
                    });
                }>
                    // No need to wrap text in quotation marks (:
                    Click me and check your console
                </button>
                {children}
            </div>
        }
    }
}

#[derive(Copy, Clone)]
struct ChildView;

impl ChildView {
    fn new() -> Self {
        Self {}
    }
}

impl Component for ChildView {
    fn render(&self) -> VirtualNode {
        html! {
            <div>
                I am the child view
            </div>
        }
    }
}

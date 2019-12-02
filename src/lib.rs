#![feature(proc_macro_hygiene)]

mod utils;

use std::cell::RefCell;
use std::rc::Rc;
use rand::prelude::*;

use wasm_bindgen::prelude::*;
use host_component_macro::*;
use host_component::{html, use_state, Application, Component, VirtualNode, IterableNodes, Context, RawContext};

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
    context: Context,
    app: Application<NewHomeView>
}

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let context = Rc::new(RefCell::new(RawContext::new()));
        let app = Application::new(String::from("body"), context.clone());

        Self {
            context,
            app
        }
    }

    #[wasm_bindgen]
    pub fn update(&mut self) {
        self.app.render();
    }
}

use illicit;

pub struct LocalContext(Context);

impl LocalContext {
    pub fn current() -> Self {
        Wrapper::with_current_wrapper(|c| LocalContext(c.context.clone()))
    }
}

#[derive(Debug)]
struct Wrapper {
    context: Context
}

impl Wrapper {
    fn enter_child<R>(&self, context: Context, child: impl FnOnce() -> R) -> R {
        let child_wrapper = Self {
            context,
        };

        illicit::child_env!(Wrapper => child_wrapper).enter(child)
    }

	fn with_current_wrapper<Out>(op: impl FnOnce(&Wrapper) -> Out) -> Out {
        if let Some(current) = illicit::Env::get::<Self>() {
            op(&*current)
        } else {
            op(&Wrapper::default())
        }
    }
}

impl Default for Wrapper {
	fn default() -> Self {
		Self {
            context: Rc::new(RefCell::new(RawContext::new()))
        }
	}
}

struct NewHomeView;

component! {
    impl NewHomeView {
        fn new() -> Self {
            Self {}
        }

        fn render(&self) -> VirtualNode {
            // Declare a new state variable which we'll call "count"
            let (count, count_access) = use_state("count", || 0);
            
            let mut rng = rand::thread_rng();
            let y: f64 = rng.gen();

            html! {
                <div>
                    <strong>Hello World</strong><br/>
                    // <strong>Count: {format!{"{}", count}}</strong><br/>
                    <strong>Random: {format!("{}", y)}</strong><br/>
                    <button onclick=move |_event: web_sys::Event| {
                        log!("Button Clicked!");
                        // count_access.set(count + 1);
                    }>
                        // No need to wrap text in quotation marks (:
                        Click me and check your console
                    </button>
                </div>
            }
        }
    }
}

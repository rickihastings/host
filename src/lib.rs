#[macro_use]
extern crate horrorshow;

mod utils;

use horrorshow::{Error, Template};
use host_core::component::Renderable;
use host_component::render;
use host_vdom::dom;
use wasm_bindgen::prelude::*;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
// macro_rules! log {
//     ( $( $t:tt )* ) => {
//         web_sys::console::log_1(&format!( $( $t )* ).into());
//     }
// }

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn init() -> Result<(), JsValue> {
    let (document, body) = dom::prepare();
    let root = RootView::new("Ricki");

    render::render_into_dom(root, &document, &body);

    Ok(())
}

enum Msg {
    Increment
}

pub fn create_event<M>(message: M) -> &'static str {


    ""
}

struct RootView {
    name: &'static str,
}

impl RootView {
    fn new(name: &'static str) -> Self {
        Self { name }
    }

    fn update(&self, message: Msg) {
        web_sys::console::log_1(&"click".into());
    }
}

impl Renderable<Error> for RootView {
    fn render(&self) -> Result<String, Error> {
        (html! {
            article {
                header(class="post-header", onclick=create_event(Msg::Increment)) {
                    p : self.name;
                }
                section(class="post-body") : "Body";
            }
        })
        .into_string()
    }
}

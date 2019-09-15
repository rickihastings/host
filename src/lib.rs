#[macro_use]
extern crate horrorshow;

mod utils;

use horrorshow::{Template, Error};
use host_component::{
    render,
    component::{Renderable}
};
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

struct RootView {
    name: &'static str,
}

impl RootView {
    fn new(name: &'static str) -> Self {
        Self { name }
    }
}

impl Renderable<Error> for RootView {
    fn render(&self) -> Result<String, Error> {
        (html! {
            article {
                header(class="post-header") {
                    p : self.name;
                }
                section(class="post-body") : "Body";
            }
        })
        .into_string()
    }
}

// fn render(document: &Document, root: &Element, create: &Fn() -> Result<String, Error>) {
//     match create() {
//         Ok(val) => {
//             let tree = parse::create_tree(&val).unwrap();
//             match render::render(&document, &tree) {
//                 Some(val) => {
//                     root.append_child(&val);
//                     ()
//                 }
//                 None => (),
//             };
//         }
//         Err(e) => panic!(e),
//     }
// }

// fn create_html() -> Result<String, Error> {
//     (html! {
//         article {
//             header(class="post-header") {
//                 p : "Title";
//             }
//             section(class="post-body") : "Body";
//         }
//     })
//     .into_string()
// }

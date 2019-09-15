use wasm_bindgen_test::*;
use web_sys::Document;
use host_vdom::{render, parse};

const RAW_HTML: &str = "<article><header class=\"post-header\"><h1>Title</h1></header><section class=\"post-body\">Body</section></article>";

wasm_bindgen_test_configure!(run_in_browser);

fn get_document() -> Document {
    Document::new().unwrap()
}

#[wasm_bindgen_test]
fn render_test() {
    let root = parse::create_tree(RAW_HTML).unwrap();
    
    match render::render(&get_document(), &root) {
        Some(element) => assert_eq!(element.outer_html(), RAW_HTML, "render() output does not match VDOM"),
        None => assert!(false)
    }
}
use super::component::{Renderable};
use host_vdom::{parse, render};

use web_sys::{Document, Element, HtmlElement};

fn insert_node_into_dom(node: Option<Element>, root: &HtmlElement) {
    match node {
        Some(val) => {
            match root.append_child(&val) {
                Err(e) => log!("Could not render into root: {:#?}", e),
                _ => (),
            };
        }
        None => {
            log!("Could not render into root: {:#?}", root);
        }
    }
}

pub fn render_into_dom<T: Renderable<E>, E>(
    component: T,
    document: &Document,
    root: &HtmlElement
) {
    match component.render() {
        Ok(html) => {
            match parse::create_tree(&html) {
                Some(tree) => {
                    insert_node_into_dom(render::render(&document, &tree), &root);
                }
                None => {
                    log!("Could not create tree");
                }
            };
        }
        Err(_) => {
            log!("Could not create vDOM node");
        }
    };
}

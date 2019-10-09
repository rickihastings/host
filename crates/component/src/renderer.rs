use crate::{dom, parse, diff, Component};
use web_sys::{Document, Element};

fn insert_node_into_dom(node: Option<Element>, root: &Element) {
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

fn update_node_into_dom(node: Option<Element>, root: &Element) {
    match node {
        Some(val) => {
            let mut patch = diff::diff_nodes(root, &val);

            patch(root);
        }
        None => {
            log!("Could not render into root: {:#?}", root);
        }
    }
}

pub fn render_into_dom<T>(component: T, document: &Document, root: &Element, initial: bool)
where
    T: Component,
{
    let html = component.render();

    match parse::create_tree(&html) {
        Some(tree) => {
            let renderer = dom::TreeRenderer::new(&document, &tree, component);

            if initial {
                insert_node_into_dom(renderer.render(), &root);
            } else {
                update_node_into_dom(renderer.render(), &root);
            }
        }
        None => {
            log!("Could not create tree");
        }
    };
}


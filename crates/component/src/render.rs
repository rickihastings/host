use crate::{dom, parse, Component, Model};
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

pub fn render_into_dom<T, E>(
    component: &'static T,
    document: &Document,
    root: &Element,
)
where
    T: Component<E> + Model + 'static
{
    match component.render() {
        Ok(html) => {
            match parse::create_tree(&html) {
                Some(tree) => {
                    let renderer = dom::TreeRenderer::new(&document, &tree, component);

                    insert_node_into_dom(renderer.render(), &root);
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

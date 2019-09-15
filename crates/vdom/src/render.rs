use treexml::Element as VNode;
use web_sys::{Document, Element};

fn append_child(element: &Element, child: &Element) {
    match element.append_child(child) {
        Ok(_) => (),
        Err(e) => {
            log!("Could not append child to element: {:?}", e);
            ()
        }
    }
}

fn render_element(document: &Document, vnode: &VNode) -> Option<Element> {
    let element;

    match document.create_element(&vnode.name) {
        Ok(val) => element = val,
        Err(e) => {
            log!("Could not create element: {:?}", e);
            return None;
        }
    }

    // Set attributes first
    for (k, v) in vnode.attributes.iter() {
        let _ = element.set_attribute(k, v);
    }

    Some(element)
}

fn render_element_with_text(document: &Document, vnode: &VNode) -> Option<Element> {
    let element;

    match render_element(document, vnode) {
        Some(val) => element = val,
        _ => return None,
    }

    match &vnode.text {
        Some(val) => {
            element.set_inner_html(val);
            Some(element)
        }
        None => Some(element),
    }
}

fn render_element_with_children(document: &Document, vnode: &VNode) -> Option<Element> {
    let element;

    match render_element(document, vnode) {
        Some(val) => element = val,
        _ => return None,
    }

    // Set children
    for child in vnode.children.iter() {
        match render(&document, &child) {
            Some(val) => append_child(&element, &val),
            None => {}
        };
    }

    Some(element)
}

pub fn render(document: &Document, vnode: &VNode) -> Option<Element> {
    match &vnode.text {
        Some(_) => return render_element_with_text(document, vnode),
        None => (),
    }

    if vnode.children.len() > 0 {
        render_element_with_children(document, vnode)
    } else {
        None
    }
}

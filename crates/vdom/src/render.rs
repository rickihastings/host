use host_core::component::Renderable;
use treexml::Element as VNode;
use web_sys::{Document, Element};

//use crate::events;

fn append_child(element: &Element, child: &Element) {
    match element.append_child(child) {
        Ok(_) => (),
        Err(e) => {
            log!("Could not append child to element: {:?}", e);
            ()
        }
    }
}

fn render_element<T: Renderable<E>, E>(
    document: &Document,
    vnode: &VNode,
    component: &T,
) -> Option<Element> {
    let element;

    match document.create_element(&vnode.name) {
        Ok(val) => element = val,
        Err(e) => {
            log!("Could not create element: {:?}", e);
            return None;
        }
    }

    // Set attributes that aren't events first
    for (k, v) in vnode.attributes.iter() {
        if k.starts_with("on") {
            // let handler = Closure::wrap(Box::new(move || {
            //     web_sys::console::log_1(&"click".into());
            // }) as Box<dyn FnMut()>);

            // element
            //     .add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
            //     .unwrap();

            // handler.forget();
        } else {
            let _ = element.set_attribute(k, v);
        }
    }

    Some(element)
}

fn render_element_with_text<T: Renderable<E>, E>(
    document: &Document,
    vnode: &VNode,
    component: &T,
) -> Option<Element> {
    let element;

    match render_element(document, vnode, component) {
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

fn render_element_with_children<T: Renderable<E>, E>(
    document: &Document,
    vnode: &VNode,
    component: &T,
) -> Option<Element> {
    let element;

    match render_element(document, vnode, component) {
        Some(val) => element = val,
        _ => return None,
    }

    // Set children
    for child in vnode.children.iter() {
        match render(document, &child, component) {
            Some(val) => append_child(&element, &val),
            None => {}
        };
    }

    Some(element)
}

pub fn render<T: Renderable<E>, E>(
    document: &Document,
    vnode: &VNode,
    component: &T,
) -> Option<Element> {
    match &vnode.text {
        Some(_) => return render_element_with_text(document, vnode, component),
        None => (),
    }

    if vnode.children.len() > 0 {
        render_element_with_children(document, vnode, component)
    } else {
        None
    }
}

use crate::{events, Component};
use treexml::Element as VNode;
use web_sys::{Document, Element};

pub struct TreeRenderer<'a, T>
where
    T: Component,
{
    document: &'a Document,
    vnode: &'a VNode,
    component: T,
}

impl<'a, T> TreeRenderer<'a, T>
where
    T: Component,
{
    pub fn new(document: &'a Document, vnode: &'a VNode, component: T) -> Self {
        Self {
            document,
            vnode,
            component,
        }
    }

    fn render_element(&self) -> Option<Element> {
        let element;

        match self.document.create_element(&self.vnode.name) {
            Ok(val) => element = val,
            Err(e) => {
                log!("Could not create element: {:?}", e);
                return None;
            }
        }

        // Set attributes that aren't events first
        for (k, v) in self.vnode.attributes.iter() {
            if k.starts_with("on") {
                events::create_event_handler(k, &element, self.component);
            } else {
                let _ = element.set_attribute(k, v);
            }
        }

        Some(element)
    }

    fn render_element_with_text(&self) -> Option<Element> {
        let element;

        match self.render_element() {
            Some(val) => element = val,
            _ => return None,
        }

        match &self.vnode.text {
            Some(val) => {
                element.set_inner_html(&val);
                Some(element)
            }
            None => Some(element),
        }
    }

    fn render_element_with_children(&self) -> Option<Element> {
        let element;

        match self.render_element() {
            Some(val) => element = val,
            _ => return None,
        }

        // Set children
        for child in self.vnode.children.iter() {
            let renderer = TreeRenderer::new(self.document, &child, self.component);

            match renderer.render() {
                Some(val) => append_child(&element, &val),
                None => {}
            };
        }

        Some(element)
    }

    pub fn render(&self) -> Option<Element> {
        match self.vnode.text {
            Some(_) => return self.render_element_with_text(),
            None => (),
        }

        if self.vnode.children.len() > 0 {
            self.render_element_with_children()
        } else {
            None
        }
    }
}

pub fn get_document() -> Document {
    // Use `web_sys`'s global `window` function to get a handle on the global window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    document
}

pub fn prepare(selector: &str) -> (Document, Element) {
    let document = get_document();
    let element = document
        .query_selector(selector)
        .expect("cannot find element in document")
        .unwrap();

    (document, element)
}

fn append_child(element: &Element, child: &Element) {
    match element.append_child(child) {
        Ok(_) => (),
        Err(e) => {
            log!("Could not append child to element: {:?}", e);
            ()
        }
    }
}

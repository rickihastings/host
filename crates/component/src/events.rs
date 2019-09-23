use crate::Model;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, Event};

pub fn create_event_handler<T>(name: &str, element: &Element, component: T)
where
    T: Model,
{
    match map_js_event_to_event(name) {
        Some(real_event) => {
            let boxed_callback = Box::new(move |event| match get_message_from_event(&event) {
                Some(message) => match component.cast_to_message(&message) {
                    Some(casted_message) => {
                        component.update(&event, casted_message);
                    }
                    None => {}
                },
                None => {}
            });

            add_event_listener(real_event, element, boxed_callback);
        }
        None => {}
    }
}

fn add_event_listener<T>(event_name: &str, element: &Element, handler: T)
where
    T: 'static + FnMut(Event),
{
    let closure = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);

    element.add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref());
    closure.forget();
}

fn get_message_from_event<'a>(event: &'a Event) -> Option<String> {
    let target = event.current_target()?;

    if let Some(element) = target.dyn_ref::<web_sys::HtmlElement>() {
        Some(element.get_attribute(crate::EVENT_DATA_ATTRIBUTE)?)
    } else {
        None
    }
}

fn map_js_event_to_event(event: &str) -> Option<&'static str> {
    match event {
        "onclick" => Some("click"),
        _ => None,
    }
}

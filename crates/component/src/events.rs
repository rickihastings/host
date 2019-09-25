use crate::Model;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, Event};

pub fn create_event_handler<'a, T>(name: &str, element: &Element, component: T)
where
    T: Model,
{
    if let Some(real_event) = map_js_event_to_event(name) {
        let mut cloned = component.clone();
        let boxed_callback = Box::new(move |event| {
            if let Some(message) = get_message_from_event(&event) {
                if let Some(casted_message) = cloned.cast_to_message(&message) {
                    cloned.update(&event, casted_message);
                }
            }
        });

        add_event_listener(real_event, element, boxed_callback);
    }
}

fn add_event_listener<T>(event_name: &str, element: &Element, handler: T)
where
    T: 'static + FnMut(Event),
{
    let closure = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);

    if let Ok(_) =
        element.add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref())
    {
        closure.forget();
    };
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

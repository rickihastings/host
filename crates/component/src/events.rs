use crate::{state, Component};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, Event};

pub fn create_event_handler<'a, T>(name: &str, element: &Element, component: T)
where
    T: Component,
{
    if let Some(real_event) = map_js_event_to_event(name) {
        let global_state = state::get();

        if let Ok(event_map) = global_state.events.clone().lock() {
            if let Some(message) = event_map.get("1").map(|x| *x) {
                let mut cloned_component = component.clone();
                let boxed_callback = Box::new(move |event| {
                    log!("Test");
                    cloned_component.emit_event(&event, message)
                });

                add_event_listener(real_event, element, boxed_callback)
            }
        }
    }
}

fn add_event_listener<T>(event_name: &str, element: &Element, handler: T)
where
    T: FnMut(Event) + 'static,
{
    let closure = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);

    if let Ok(_) =
        element.add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref())
    {
        closure.forget();
    };
}

fn map_js_event_to_event(event: &str) -> Option<&'static str> {
    match event {
        "onclick" => Some("click"),
        _ => None,
    }
}

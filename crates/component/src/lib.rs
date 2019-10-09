extern crate web_sys;

use num_traits::{FromPrimitive, ToPrimitive};
use render::Renderable;
use web_sys::Event;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

/// An interface for a React-style Component
pub trait Component: Sized + 'static + Copy + Renderable {
    type Message: 'static + FromPrimitive + ToPrimitive;
    type Props;

    /// Constructor for the model with props
    fn new(props: Self::Props) -> Self;

    fn emit_update(mut self, event: &Event, message: i32) {
        if let Some(casted_message) = Self::Message::from_i32(message) {
            // todo - might be better to mutate
            let component = self.update(event, casted_message);
            let (document, root) = dom::prepare("body > article");

            renderer::render_into_dom(component, &document, &root, false);
        }
    }

    /// Update
    fn update(mut self, event: &Event, message: Self::Message) -> Self;

    /// Used to bind events to components
    fn create_event(self, message: &Self::Message) -> &'static str {
        let global_state = state::get();
        if let Ok(mut event_map) = global_state.events.lock() {
            if let Some(message_int) = message.to_i32() {
                // todo
                event_map.insert(String::from("1"), message_int);
            }
        }

        &""
    }
}

pub fn start<T>(root: &str, props: T::Props)
where
    T: Component,
{
    application::App::<T>::new(root, props).mount();
}

pub mod application;
pub mod diff;
pub mod dom;
pub mod events;
pub mod parse;
pub mod renderer;
pub mod state;

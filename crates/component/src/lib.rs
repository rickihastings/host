#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

extern crate web_sys;

use num_traits::{FromPrimitive, ToPrimitive};
use render::Renderable;
use web_sys::Event;

pub static EVENT_DATA_ATTRIBUTE: &str = "data-m";

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// pub trait Renderer {
//     fn on_update(&mut self);
// }

// impl<T> Renderer for T
// where
//     T: Component,
// {
//     fn on_update(&mut self) {
//         let (document, root) = dom::prepare("body");

//         render::render_into_dom(*self, &document, &root);
//     }
// }

/// An interface for the data layer behind a Component
// pub trait Model: Sized + Copy + Renderer + 'static {
//     /// The message type which should be used to update the view
//     type Message: 'static + ToString + FromStr;

//     /// Constructor for the model
//     fn new() -> Self;

//     fn trigger_update(&mut self, event: &Event, message: Self::Message) {
//         self.update(event, message);
//         self.on_update();
//     }

//     /// Called whenever an update is received from any source
//     fn update(&mut self, event: &Event, message: Self::Message);

//     /// Used to bind events to messages
//     fn create_event(&self, message: Self::Message) -> String {
//         message.to_string()
//     }

//     /// Used to cast a String to a Self::Message
//     fn cast_to_message(&self, message: &str) -> Option<Self::Message> {
//         match Self::Message::from_str(message) {
//             Ok(value) => Some(value),
//             Err(_) => None,
//         }
//     }
// }

/// An interface for a React-style Component
pub trait Component: Sized + Copy + Renderable {
    type Message: FromPrimitive + ToPrimitive;
    type Props;

    /// Constructor for the model with props
    fn new(props: Self::Props) -> Self;

    fn emit_event(self, event: &Event, message: i32) {
        if let Some(message_enum) = Self::Message::from_i32(message) {
            self.update(event, message_enum);
        }
    }

    /// Update
    fn update(self, event: &Event, message: Self::Message);

    /// Used to bind events to components
    fn create_event(self, message: &Self::Message) -> &'static str {
        let global_state = state::get();
        if let Ok(mut event_map) = global_state.events.lock() {
            if let Some(message_int) = message.to_i32() {
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
pub mod dom;
pub mod events;
pub mod parse;
pub mod renderer;
pub mod state;

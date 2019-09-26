extern crate web_sys;

use std::str::FromStr;
use std::string::ToString;
use web_sys::Event;

pub static EVENT_DATA_ATTRIBUTE: &str = "data-m";

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

impl<T> EventEmitter for T
where
    T: Model,
{
    fn on_update(&mut self) {
        log!("on update");
    }
}

pub trait EventEmitter {
    fn on_update(&mut self);
}

/// An interface for the data layer behind a Component
pub trait Model: Sized + Copy + EventEmitter + 'static {
    /// The message type which should be used to update the view
    type Message: 'static + ToString + FromStr;

    /// Constructor for the model
    fn new() -> Self;

    fn trigger_update(&mut self, event: &Event, message: Self::Message) {
        self.update(event, message);
        self.on_update();
    }

    /// Called whenever an update is received from any source
    fn update(&mut self, event: &Event, message: Self::Message);

    /// Used to bind events to messages
    fn create_event(&self, message: Self::Message) -> String {
        message.to_string()
    }

    /// Used to cast a String to a Self::Message
    fn cast_to_message(&self, message: &str) -> Option<Self::Message> {
        match Self::Message::from_str(message) {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    }
}

/// An interface for a React-style Component
pub trait Component<E>: Sized + Copy + Model {
    /// What returns the HTML, it's not essential to use horrorshow here, you can
    /// render with thatever you like, as long as it returns a Result<String, AnError>
    fn render(&self) -> Result<String, E>;
}

pub fn start<T, E>(root: &str)
where
    T: Component<E> + Model,
{
    application::App::<T, E>::new(root).mount();
}

pub mod application;
pub mod dom;
pub mod events;
pub mod parse;
pub mod render;

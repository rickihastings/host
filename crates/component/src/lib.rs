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

/// An interface for the data layer behind a Component
pub trait Model: Sized + Copy + 'static {
    /// The message type which should be used to update the view
    type Message: 'static + ToString + FromStr;

    /// Called whenever an update is received from any source
    fn update(&self, event: &Event, message: Self::Message);

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
pub trait Component<E>: Sized + Model + Copy {
    /// What returns the HTML, it's not essential to use horrorshow here, you can
    /// render with thatever you like, as long as it returns a Result<String, AnError>
    fn render(&self) -> Result<String, E>;

    fn re_render(&self) {
        // let (document, root) = dom::prepare("body");
        // render::render_into_dom(self, &document, &root);
    }
}

pub fn start<T, E>(element: &str, component: T)
where
    T: Component<E> + Model,
{
    application::App::new(component).mount(element);
}

pub mod application;
pub mod dom;
pub mod events;
pub mod parse;
pub mod render;

use std::sync::{Arc, Mutex, Once, ONCE_INIT};
use std::time::Duration;
use std::{mem, thread};
use std::collections::HashMap;
use web_sys::Event;

#[derive(Clone)]
pub struct GlobalState {
    // Since we will be used in many threads, we need to protect
    // concurrent access
    pub events: Arc<Mutex<HashMap<String, i32>>>,
}

pub fn get() -> GlobalState {
    // Initialize it to a null value
    static mut SINGLETON: *const GlobalState = 0 as *const GlobalState;
    static ONCE: Once = ONCE_INIT;

    unsafe {
        ONCE.call_once(|| {
            // Make it
            let singleton = GlobalState {
                events: Arc::new(Mutex::new(HashMap::new())),
            };

            // Put it in the heap so it can outlive this call
            SINGLETON = mem::transmute(Box::new(singleton));
        });

        // Now we give out a copy of the data that is safe to use concurrently.
        (*SINGLETON).clone()
    }
}
#![feature(proc_macro_hygiene)]

mod utils;

use host_component::prelude::*;
use wasm_bindgen::prelude::*;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Client {
    #[allow(dead_code)]
    app: Application,
}

#[wasm_bindgen]
impl Client {
    #[allow(dead_code)]
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        utils::set_panic_hook();

        let store = DefaultReducer::new(DefaultState { count: 0 });

        let component = Box::new(NewHomeView::new());
        let mut app = Application::new("body", component);

        app.inject_store::<DefaultReducer, Action, DefaultState>(store);

        app.mount();

        Self { app }
    }

    #[allow(dead_code)]
    #[wasm_bindgen]
    pub fn update(&mut self) {
        self.app.render();
    }
}

// Reducer

#[derive(Copy, Clone)]
pub enum Action {
    IncCount,
}

#[derive(Copy, Clone)]
struct DefaultState {
    count: u8,
}

#[derive(Copy, Clone)]
struct DefaultReducer {
    state: DefaultState,
}

impl Reducer<Action, DefaultState> for DefaultReducer {
    fn new(state: DefaultState) -> Self {
        Self { state }
    }

    fn get_state(&self) -> DefaultState {
        self.state
    }

    fn reducer(&mut self, action: Action) {
        match action {
            Action::IncCount => self.state.count += 1,
        };
    }
}

// Component

#[derive(Copy, Clone)]
struct NewHomeView;

impl NewHomeView {
    fn new() -> Self {
        Self
    }

    fn selector(&self, ctx: &ApplicationCtx) -> DefaultState {
        ctx.get_state::<DefaultReducer, Action, DefaultState>()
            .unwrap_or(DefaultState { count: 0 })
    }
}

impl Component for NewHomeView {
    fn render(&self, ctx: ApplicationCtx) -> VirtualNode {
        let state = self.selector(&ctx);

        html! {
            <div>
                <strong>Hello World</strong><br/>
                <strong>Count: {format!{"{}", state.count}}</strong><br/>
                <button
                    key={state.count}
                    onclick=move |_event: web_sys::Event| {
                        ctx.clone().dispatch::<DefaultReducer, Action, DefaultState>(Action::IncCount);
                    }
                >
                    // No need to wrap text in quotation marks
                    Click me
                </button>
                <br/>

                {if state.count == 10 {
                    {"Count is equal to 10"}
                }}

                // <br/>
                // {html_if_else! {(count > 10) {
                //     {"Count is greater than 10"}
                // } else {
                //     {"Count is less than 10"}
                // }}}

                // {{
                //     let mut __component = ChildView::new();
                //     // let __boxed_component = Box::new(__component);
                //     // let __component_context = $crate::ComponentContext::new(__boxed_component.id(), 0, __boxed_component);

                //     __component.render(ctx.clone())
                // }}
            </div>
        }
    }
}

// Child Component

#[derive(Copy, Clone)]
struct ChildView;

impl ChildView {
    fn new() -> Self {
        Self {}
    }
}

impl Component for ChildView {
    fn render(&self, ctx: ApplicationCtx) -> VirtualNode {
        html! {
            <div>
                I am the child view
            </div>
        }
    }
}

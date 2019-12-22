use crate::reducer::{Action, DefaultState, DefaultReducer};

use host_component::prelude::*;

pub struct NewHomeView;

impl NewHomeView {
    pub fn new() -> Self {
        Self
    }

    fn selector(&self, ctx: &ApplicationCtx) -> DefaultState {
        ctx.get_state::<DefaultReducer, Action, DefaultState>()
            .unwrap_or(DefaultState { count: 0 })
    }
}

impl Component for NewHomeView {
    #[illicit::from_env(ctx: &ApplicationCtx)]
    fn render(&self) -> VirtualNode {
        let cloned = ctx.clone();
        let state = self.selector(&ctx);

        html! {
            <div>
                <h1>Counter Example</h1>
                <button
                    key={state.count}
                    onclick=move |_event: web_sys::Event| {
                        cloned.dispatch::<DefaultReducer, Action, DefaultState>(Action::IncCount);
                    }
                >
                    // No need to wrap text in quotation marks
                    Click me
                </button>
                <hr/>
                <ChildView count={state.count}/>
            </div>
        }
    }
}

// Child Component
struct ChildView {
    count: u8,
}

impl Component for ChildView {
    fn render(&self) -> VirtualNode {
        html! {
            <div>
                Count is {format!("{}", self.count)}
            </div>
        }
    }
}

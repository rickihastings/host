use crate::{dom, render, Component, Model};

pub struct App<'a, T>
where
    T: Component + Model,
{
    component: T,
    root: &'a str,
}

impl<'a, T> App<'a, T>
where
    T: Component + Model,
{
    pub fn new(root: &'a str) -> Self {
        let component = T::new();

        App {
            component,
            root,
        }
    }

    pub fn mount(&self) {
        let (document, root) = dom::prepare(self.root);

        render::render_into_dom(self.component, &document, &root);
    }
}

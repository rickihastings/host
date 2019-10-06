use crate::{dom, renderer, Component};

pub struct App<'a, T>
where
    T: Component,
{
    component: T,
    root: &'a str,
}

impl<'a, T> App<'a, T>
where
    T: Component,
{
    pub fn new(root: &'a str, props: T::Props) -> Self {
        let component = T::new(props);

        App { component, root }
    }

    pub fn mount(&self) {
        let (document, root) = dom::prepare(self.root);

        renderer::render_into_dom(self.component, &document, &root);
    }
}

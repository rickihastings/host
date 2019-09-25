use crate::{dom, render, Component, Model};
use std::marker::PhantomData;

pub struct App<'a, T, E>
where
    T: Component<E> + Model,
{
    component: T,
    root: &'a str,
    __phantom: PhantomData<E>,
}

impl<'a, T, E> App<'a, T, E>
where
    T: Component<E> + Model,
{
    pub fn new(root: &'a str) -> Self {
        let component = T::new();

        App {
            component,
            root,
            __phantom: PhantomData,
        }
    }

    pub fn mount(&self) {
        let (document, root) = dom::prepare(self.root);

        render::render_into_dom(self.component, &document, &root);
    }
}

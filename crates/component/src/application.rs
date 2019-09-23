use crate::{dom, render, Component, Model};
use std::marker::PhantomData;

pub struct App<T, E>
where
    T: Component<E> + Model,
{
    component: T,
    __phantom: PhantomData<E>,
}

impl<T, E> App<T, E>
where
    T: Component<E> + Model,
{
    pub fn new(component: T) -> Self {
        App {
            component: component,
            __phantom: PhantomData,
        }
    }

    pub fn mount(&self, element: &str) {
        let (document, root) = dom::prepare(element);

        render::render_into_dom(self.component, &document, &root);
    }
}

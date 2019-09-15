pub trait Renderable<E> {
    fn render(&self) -> Result<String, E>;
}

// impl<T> std::fmt::Debug for Box<dyn Renderable<T>> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         // todo make this better
//         write!(f, "Renderable()")
//     }
// }

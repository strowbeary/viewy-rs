use crate::node::Node;
use crate::renderer::Renderable;

pub trait ChildContainer {
    fn get_children(&mut self) -> &mut Vec<Box<dyn Renderable>>;
}

pub trait Appendable<T = Self>: ChildContainer + Clone {
    fn append_child<C>(&mut self, child: C) -> Self
        where
            C: 'static + Renderable,
    {
        self.get_children().push(Box::new(child));
        self.clone()
    }
}
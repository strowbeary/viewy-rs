use crate::node::Node;

pub trait Renderable {
    fn render(self) -> Node;

    fn component_name(&self) -> &str {
        "View"
    }
}

impl<R> Renderable for &R
where
    R: Renderable + Clone,
{
    fn render(self) -> Node {
        self.clone().render()
    }
}

impl<R> Renderable for &mut R
where
    R: Renderable + Clone,
{
    fn render(self) -> Node {
        self.clone().render()
    }
}

impl<R> Renderable for Box<R>
where
    R: Renderable + Clone,
{
    fn render(self) -> Node {
        self.render()
    }
}

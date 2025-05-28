use crate::DefaultModifiers;
use crate::components::Appendable;
use crate::engine::Renderable;
use crate::node::Node;
use std::borrow::BorrowMut;

/// This is a simple configurable node
#[derive(Debug, Clone)]
pub struct View {
    pub node: Node,
}

impl View {
    pub fn new() -> Self {
        View {
            node: Default::default(),
        }
    }
}
impl std::ops::Deref for View {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

impl std::ops::DerefMut for View {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.node
    }
}
impl DefaultModifiers for View {}

impl Appendable for View {}

impl Renderable for View {
    fn render(mut self) -> Node {
        self.node
    }
}

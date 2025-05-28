use crate::DefaultModifiers;
use crate::Renderable;
use crate::node::Node;

/// Draw a horizontal line to separate content.
#[derive(Debug, Clone)]
pub struct Divider {
    node: Node,
}

impl DefaultModifiers for Divider {}

impl Divider {
    pub fn new() -> Self {
        Divider {
            node: Node::default(),
        }
    }
}
impl std::ops::Deref for Divider {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

impl std::ops::DerefMut for Divider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.node
    }
}

impl Renderable for Divider {
    fn render(mut self) -> Node {
        self.add_class("divider");
        self.node
    }
}

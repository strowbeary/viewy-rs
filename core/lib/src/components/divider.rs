use crate::renderer::{Renderable};
use crate::node::{Node, NodeContainer};
use std::borrow::BorrowMut;
use crate::DefaultModifiers;

/// Draw a horizontal line to separate content.
#[derive(Debug, Clone)]
pub struct Divider {
    node: Node,
}

impl NodeContainer for Divider {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers<Divider> for Divider {}



impl Divider {
    pub fn new() -> Self {
        Divider {
            node: Node::default(),
        }
    }
}

impl Renderable for Divider {
    fn render(&self) -> Node {

        let mut view = self.clone()
            .add_class("divider")
            .node;
        view
    }
}
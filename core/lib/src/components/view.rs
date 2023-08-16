use crate::core::node::{Node, NodeType};
use crate::core::widget::Widget;
use crate::core::modifiers::Appendable;

#[derive(Widget, Appendable)]
pub struct View {
    node: Node
}

impl View {
    pub fn new() -> Self {
        View {
            node: Node::default()
        }
    }

    pub fn render(&mut self) {

    }
}


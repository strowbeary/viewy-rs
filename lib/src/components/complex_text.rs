use crate::node::{Node, NodeContainer};
use crate::components::TextStyle;
use std::borrow::BorrowMut;
use crate::DefaultModifiers;
use crate::{Renderable};

#[derive(Debug, Clone)]
pub struct ComplexText {
    node: Node,
    pub content: String,
    pub style: TextStyle,
}

impl NodeContainer for ComplexText {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers<ComplexText> for ComplexText {}

impl ComplexText {
    pub fn new(content: &str, style: TextStyle) -> Self {
        let mut node = Node::default();
        node.text = Some(markdown::to_html(content.as_ref()));
        ComplexText {
            node,
            content: content.to_string(),
            style,
        }
    }
}

impl Renderable for ComplexText {
    fn render(&self) -> Node {
        let text = self.clone()
            .add_class("text")
            .add_class(format!("text--{:?}", self.style).to_lowercase().as_str());
        text.node
    }
}
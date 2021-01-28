use crate::{StyleRegistry, Renderable};
use std::sync::Mutex;
use crate::template_compilation_tools::ScriptRegistry;
use crate::node::{Node, DefaultModifiers, NodeContainer};
use std::borrow::BorrowMut;
use crate::components::TextStyle;


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
    fn render(&self, style_registery: &mut StyleRegistry, _script_registery: &mut ScriptRegistry) -> Node {
        style_registery.register_stylesheet(
            "text",
            include_str!("../themes/components/text.scss"),
        );
        let text = self.clone()
            .add_class("text")
            .add_class(format!("text--{:?}", self.style).to_lowercase().as_str());
        text.node
    }
}
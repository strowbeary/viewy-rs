use crate::DefaultModifiers;
use crate::Renderable;
use crate::components::TextStyle;
use crate::node::Node;
use std::borrow::BorrowMut;

#[derive(Debug, Clone)]
pub struct ComplexText {
    node: Node,
    pub content: String,
    pub style: TextStyle,
}

impl DefaultModifiers for ComplexText {}

impl ComplexText {
    pub fn new(content: &str, style: TextStyle) -> Self {
        ComplexText {
            node: Node::default(),
            content: content.to_string(),
            style,
        }
    }
}
impl std::ops::Deref for ComplexText {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

impl std::ops::DerefMut for ComplexText {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.node
    }
}

impl Renderable for ComplexText {
    fn render(mut self) -> Node {
        self.node.text = Some(markdown::to_html(self.content.as_ref()));
        let text_style = format!("text--{:?}", self.style).to_lowercase();
        self.add_class("text")
            .add_class("complex-text")
            .add_class(&text_style);
        self.node
    }
}

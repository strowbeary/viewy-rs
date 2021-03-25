use crate::node::{Node, NodeContainer};
use std::borrow::BorrowMut;
use crate::DefaultModifiers;
use crate::renderer::{ToHtml, Renderable};
use html_escape::encode_text;

#[derive(Debug, Clone)]
pub enum TextStyle {
    LargeTitle,
    H1,
    H2,
    H3,
    Headline,
    Subtitle1,
    Subtitle2,
    Subtitle3,
    Body,
    Button,
    Label,
    Overline,
    Caption,
}

#[derive(Debug, Clone)]
pub struct Text {
    node: Node,
    pub content: String,
    pub style: TextStyle,
}

impl NodeContainer for Text {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers<Text> for Text {}
impl ToHtml for Text {}

impl Text {
    pub fn new(content: &str, style: TextStyle) -> Self {
        let mut node = Node::default();
        node.text = Some(encode_text(content).to_string());
        Text {
            node,
            content: content.to_string(),
            style,
        }
    }
}

impl Renderable for Text {
    fn render(&self) -> Node {
        let text = self.clone()
            .add_class("text")
            .add_class(format!("text--{:?}", self.style).to_lowercase().as_str());
        text.node
    }
}
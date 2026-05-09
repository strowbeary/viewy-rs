use crate::Widget;
use crate::node::{Node, NodeType};
use crate::prelude::Attributable;
use crate::prelude::Classable;

#[derive(Debug)]
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
    Article,
    Label,
    Overline,
    Caption,
}

impl TextStyle {
    pub fn with_content(self, content: &str) -> Text {
        Text::new(content, self)
    }
}

#[derive(Widget, Classable, Attributable)]
#[widget(style = "./style.scss")]
pub struct Text {
    node: Node,
    style: TextStyle,
    content: String,
}

impl Text {
    pub fn new(content: &str, style: TextStyle) -> Self {
        Text {
            node: Default::default(),
            style,
            content: String::from(content),
        }
    }
    pub fn render(&mut self) {
        self.add_class("text");
        match &self.style {
            TextStyle::H1 => self.node.node_type = NodeType::Normal("h1"),
            TextStyle::H2 => self.node.node_type = NodeType::Normal("h2"),
            TextStyle::H3 => self.node.node_type = NodeType::Normal("h3"),
            _ => self.node.node_type = NodeType::Normal("p"),
        }

        self.add_class(format!("text--{:?}", self.style).to_lowercase().as_str());
        self.node.text = Some(self.content.clone());
    }
}
fn test() {
    TextStyle::Article.with_content("");
    Text::new("", TextStyle::Article);
}

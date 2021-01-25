use crate::{StyleRegistry, Renderable};
use std::sync::Mutex;
use crate::template_compilation_tools::ScriptRegistry;
use crate::node::{Node, DefaultModifiers, NodeContainer};
use std::borrow::BorrowMut;

#[derive(Debug, Clone)]
pub enum TextStyle {
    LargeTitle,
    H1,
    H2,
    H3,
    Headline,
    Subtitle1,
    Subtitle2,
    Body1,
    Body2,
    Button,
    Label,
    Overline,
    Caption,
}

#[derive(Debug, Clone)]
pub struct Text {
    pub view: Node,
    pub content: String,
    pub style: TextStyle
}
impl NodeContainer for Text {
    fn get_node(&mut self) -> &mut Node {
        self.view.borrow_mut()
    }
}

impl DefaultModifiers<Text> for Text {}

impl Text {
    pub fn new(content: &str, style: TextStyle) -> Self {
        let mut view = Node::default();
        view.text = Some(content.to_string());
        Text {
            view,
            content: content.to_string(),
            style: style
        }
    }

}

impl Renderable for Text {
    fn render(&self, style_registery: &mut StyleRegistry, script_registery: &mut ScriptRegistry) -> Node {
        style_registery.register_stylesheet(
            "text",
            include_str!("../themes/components/text.scss"),
        );
        script_registery.register_script(
            "button",
            include_str!("../js/button.js"),
        );
        let text = self.clone()
            .add_class("text")
            .add_class(format!("text--{:?}", self.style).to_lowercase().as_str());
        text.view
    }
}
use crate::node::{Node, NodeContainer};
use std::borrow::BorrowMut;
use crate::DefaultModifiers;
use crate::renderer::{ToHtml, Renderable, StyleRegistry, ScriptRegistry};

#[derive(Debug, Clone)]
pub struct Image {
    node: Node,
    pub src: String,
}

impl NodeContainer for Image {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers<Image> for Image {}

impl Image {
    pub fn new(src: &str) -> Self {
        Image {
            node: Default::default(),
            src: src.to_string(),
        }
    }
}

impl ToHtml for Image {}

impl Renderable for Image {
    fn render(&self, style_registery: &mut StyleRegistry, _script_registery: &mut ScriptRegistry) -> Node {
        style_registery.register_stylesheet(
            "image",
            include_str!("../themes/components/image.scss"),
        );
        self.clone()
            .add_class("image")
            .set_attr("src", self.src.as_str())
            .tag("img")
            .node
    }
}
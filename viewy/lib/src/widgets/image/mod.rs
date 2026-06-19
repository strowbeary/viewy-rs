use crate::core::node::{Node, NodeType};
use crate::core::widget::Widget;
use crate::modifiers::*;

#[derive(Debug, Clone)]
pub enum ObjectFit {
    Fill,
    Contain,
    Cover,
    None,
    ScaleDown,
}

#[derive(Widget, Classable, Attributable, BoxStylable)]
#[widget(style = "./style.scss")]
pub struct Image {
    node: Node,
    pub src: String,
}

impl Image {
    pub fn new(src: &str) -> Self {
        let mut node = Node::default();
        node.node_type = NodeType::Normal("img");
        Image {
            node,
            src: src.to_string(),
        }
    }
    pub fn object_fit(&mut self, fit: ObjectFit) -> &mut Self {
        self.node.node_style.insert(
            "object-fit".to_string(),
            {
                match fit {
                    ObjectFit::Fill => "fill",
                    ObjectFit::Contain => "contain",
                    ObjectFit::Cover => "cover",
                    ObjectFit::None => "none",
                    ObjectFit::ScaleDown => "scale-down",
                }
            }
            .to_string(),
        );

        self
    }
    pub fn aspect_ratio(&mut self, ratio: &str) -> &mut Self {
        self.node
            .node_style
            .insert("aspect-ratio".to_string(), ratio.to_string());

        self
    }

    pub fn render(&mut self) {
        let src = self.src.to_string();
        self.add_class("image").set_attr("src", &src);
    }
}

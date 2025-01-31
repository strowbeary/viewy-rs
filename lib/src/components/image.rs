use crate::node::{Node, NodeContainer};
use crate::DefaultModifiers;
use crate::Renderable;
use std::borrow::BorrowMut;

#[derive(Debug, Clone)]
pub enum ObjectFit {
    Fill,
    Contain,
    Cover,
    None,
    ScaleDown,
}

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
    pub fn object_fit(&mut self, fit: ObjectFit) -> Self {
        self.get_node().node_style.push((
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
        ));

        self.clone()
    }
    pub fn aspect_ratio(&mut self, ratio: &str) -> Self {
        self.get_node()
            .node_style
            .push(("aspect-ratio".to_string(), ratio.to_string()));

        self.clone()
    }
}

impl Renderable for Image {
    fn render(&self) -> Node {
        self.clone()
            .add_class("image")
            .set_attr("src", self.src.as_str())
            .tag("img")
            .node
    }
}

use crate::node::{Node, NodeContainer};
use std::collections::HashMap;
use crate::{DefaultModifiers, sp};
use crate::{Renderable};
use crate::components::icons::IconPack;

pub mod icons;

#[derive(Debug, Clone)]
pub struct Icon {
    node: Node,
    pub icon: Box<dyn IconPack>,
    pub stroke_width: String,
    pub size: i32,
}

impl Icon {
    pub fn new<T>(icon: T) -> Self
        where
            T: 'static + IconPack {
        Icon {
            node: Default::default(),
            icon: Box::new(icon),
            stroke_width: "3".to_string(),
            size: 24,
        }
    }

    pub fn stroke_width(&mut self, stroke_width: i32) -> Self {
        self.stroke_width = stroke_width.to_string();
        self.clone()
    }

    pub fn size(&mut self, size: i32) -> Self {
        self.size = size;
        self.clone()
    }
}

impl NodeContainer for Icon {
    fn get_node(&mut self) -> &mut Node {
        &mut self.node
    }
}

impl DefaultModifiers<Icon> for Icon {}

impl Renderable for Icon {
    fn render(&self) -> Node {
        let mut icon = self.clone()
            .add_class("icon")
            .set_attr("xmlns", "http://www.w3.org/2000/svg")
            .set_attr("width", sp(self.size).as_str())
            .set_attr("height", sp(self.size).as_str())
            .set_attr("viewBox", "0 0 24 24")
            .set_attr("fill", "none")
            .set_attr("stroke", "currentColor")
            .set_attr("stroke-width", self.stroke_width.as_str())
            .set_attr("stroke-linecap", "round")
            .set_attr("stroke-linejoin", "round")
            .tag("svg");
        icon.get_node().text = Some(self.icon.path().to_string());
        icon.get_node().clone()
    }
}
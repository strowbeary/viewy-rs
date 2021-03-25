use crate::node::{Node, NodeContainer};
use std::collections::HashMap;
use crate::{DefaultModifiers, sp};
use crate::renderer::{ToHtml, Renderable};

#[derive(Debug, Clone)]
pub struct Icon {
    node: Node,
    pub name: String,
    pub stroke_width: String,
    pub size: i32,
}

impl Icon {
    pub fn new(name: &str) -> Self {
        Icon {
            node: Default::default(),
            name: name.to_string(),
            stroke_width: "3".to_string(),
            size: 24,
        }
    }

    pub fn size(&mut self, size: i32) -> Self {
        self.size = size;
        self.clone()
    }

    fn get_path(&self, name: &str) -> String {
        let icons: HashMap<String, String> = serde_json::from_str(include_str!("../themes/icons.json")).unwrap();
        icons.get(name).expect(format!("Icon {} doesn't exist", name).as_str()).to_string()
    }
}

impl NodeContainer for Icon {
    fn get_node(&mut self) -> &mut Node {
        &mut self.node
    }
}

impl DefaultModifiers<Icon> for Icon {}

impl ToHtml for Icon {}

impl Renderable for Icon {
    fn render(&self) -> Node {
        let mut icon = self.clone()
            .add_class("icon")
            .set_attr("width", sp(self.size).as_str())
            .set_attr("height", sp(self.size).as_str())
            .set_attr("viewBox", "0 0 24 24")
            .set_attr("fill", "none")
            .set_attr("stroke", "currentColor")
            .set_attr("stroke-width", self.stroke_width.as_str())
            .set_attr("stroke-linecap", "round")
            .set_attr("stroke-linejoin", "round")
            .tag("svg");
        icon.get_node().text = Some(self.get_path(self.name.as_str()));
        icon.get_node().clone()
    }
}
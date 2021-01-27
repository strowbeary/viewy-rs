use crate::Renderable;
use crate::node::{Node, NodeContainer, DefaultModifiers};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use crate::template_compilation_tools::{StyleRegistry, ScriptRegistry};

#[derive(Debug, Clone)]
pub struct Icon {
    node: Node,
    name: String,
    stroke_width: String,
    size: String
}

impl Icon {
    pub fn new(name: &str) -> Self {
        Icon {
            node: Default::default(),
            name: name.to_string(),
            stroke_width: "3".to_string(),
            size: "24".to_string()
        }
    }

    pub fn size(&mut self, size: i32) -> Self {
        self.size = size.to_string();
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

impl Renderable for Icon {
    fn render(&self, style_registery: &mut StyleRegistry, script_registery: &mut ScriptRegistry) -> Node {
        let mut icon = self.clone()
            .add_class("icon")
            .set_attr("width", self.size.as_str())
            .set_attr("height", self.size.as_str())
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
use std::fmt::{Debug, Formatter};
use dyn_clone::DynClone;
use crate::node::{Node, NodeContainer};
use crate::{DefaultModifiers, Renderable};

#[derive(Debug, Clone)]
pub struct DynamicContent {
	node: Node,
	pub name: String,
}

impl DynamicContent {
	pub fn new(name: &str) -> Self {
		DynamicContent {
			node: Default::default(),
			name: name.to_string()
		}
	}
}

impl NodeContainer for DynamicContent {
	fn get_node(&mut self) -> &mut Node {
		&mut self.node
	}
}

impl DefaultModifiers<DynamicContent> for DynamicContent {}

impl Renderable for DynamicContent {
	fn render(&self) -> Node {
		let mut dynamic_content = self.clone()
			.add_class("dynamic-content")
			.set_attr("data-dynamic-content-name", &self.name);
		dynamic_content.node.clone()
	}
}
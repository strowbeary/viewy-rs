use std::fmt::{Debug, Formatter};
use dyn_clone::DynClone;
use crate::node::{Node, NodeContainer};
use crate::{DefaultModifiers, Renderable};
use crate::components::{Appendable, ChildContainer, HStack};

#[derive(Debug, Clone)]
pub struct DynamicContent {
	children: Vec<Box<dyn Renderable>>,
	node: Node,
	pub name: String,
}

impl ChildContainer for DynamicContent {
	fn get_children(&mut self) -> &mut Vec<Box<dyn Renderable>> {
		&mut self.children
	}
}

impl Appendable for DynamicContent {}

impl DynamicContent {
	pub fn new(name: &str) -> Self {
		DynamicContent {
			children: vec![],
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
			.set_attr("data-dynamic-content-name", &self.name)
			.node;
		self.children.iter()
			.for_each(|child|
				dynamic_content.children.push(child.render()));
		dynamic_content
	}
}
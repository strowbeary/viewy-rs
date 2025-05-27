use std::fmt::Debug;
use crate::node::{Node, NodeContainer};
use crate::{DefaultModifiers, Renderable};
use crate::components::{Appendable, ChildContainer};

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

impl DefaultModifiers for DynamicContent {}

impl Renderable for DynamicContent {
	fn render(mut self) -> Node {
		let name = self.name.to_string();
		self
			.add_class("dynamic-content")
			.set_attr("data-dynamic-content-name", &name);
		self.children.into_iter()
			.for_each(|child|
				self.node.children.push(child.render()));
		self.node
	}
}
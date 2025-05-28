use crate::components::Appendable;
use crate::node::Node;
use crate::{DefaultModifiers, Renderable};
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct DynamicContent {
    node: Node,
    pub name: String,
}

impl Appendable for DynamicContent {}

impl DynamicContent {
    pub fn new(name: &str) -> Self {
        DynamicContent {
            node: Default::default(),
            name: name.to_string(),
        }
    }
}

impl DefaultModifiers for DynamicContent {}

impl std::ops::Deref for DynamicContent {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

impl std::ops::DerefMut for DynamicContent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.node
    }
}

impl Renderable for DynamicContent {
    fn render(mut self) -> Node {
        let name = self.name.to_string();
        self.add_class("dynamic-content")
            .set_attr("data-dynamic-content-name", &name);

        self.node
    }
}

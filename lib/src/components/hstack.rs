use crate::Renderable;
use crate::components::Alignment;
use crate::components::Appendable;
use crate::node::Node;
use crate::{DefaultModifiers, sp};

#[derive(Debug, Clone)]
pub struct HStack {
    node: Node,
    pub alignment: Alignment,
}

impl Default for HStack {
    fn default() -> Self {
        HStack {
            node: Default::default(),
            alignment: Alignment::Stretch,
        }
    }
}

impl DefaultModifiers for HStack {}

impl HStack {
    pub fn new(alignment: Alignment) -> Self {
        HStack {
            node: Default::default(),
            alignment,
        }
    }
    pub fn justify_content(&mut self, justification: &str) -> &mut Self {
        self.node
            .node_style
            .push(("justify-content".to_string(), justification.to_string()));
        self
    }

    pub fn gap(&mut self, gaps: Vec<i32>) -> &mut Self {
        let params: Vec<String> = gaps.iter().map(|size| sp(size.clone())).collect();
        self.node
            .node_style
            .push(("grid-gap".to_string(), params.join(" ")));
        self
    }
    pub fn flex_wrap(&mut self) -> &mut Self {
        self.node
            .node_style
            .push(("flex-wrap".to_string(), "wrap".to_string()));
        self
    }
}
impl std::ops::Deref for HStack {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

impl std::ops::DerefMut for HStack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.node
    }
}

impl Appendable for HStack {}

impl Renderable for HStack {
    fn render(mut self) -> Node {
        let alignment = format!("stack--align-{:?}", self.alignment).to_lowercase();
        self.add_class("stack")
            .add_class("stack--horizontal")
            .add_class(&alignment);

        self.node
    }
}

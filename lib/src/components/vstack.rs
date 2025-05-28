use crate::Renderable;
use crate::components::Appendable;
use crate::node::Node;
use crate::{DefaultModifiers, sp};
use std::borrow::BorrowMut;

#[derive(Debug, Clone)]
pub enum Alignment {
    Center,
    Start,
    End,
    Stretch,
}

#[derive(Debug, Clone)]
pub struct VStack {
    node: Node,
    pub alignment: Alignment,
}

impl DefaultModifiers for VStack {}

impl VStack {
    pub fn new(alignment: Alignment) -> Self {
        VStack {
            node: Default::default(),
            alignment,
        }
    }
    pub fn gap(&mut self, gaps: Vec<i32>) -> &mut Self {
        let params: Vec<String> = gaps.iter().map(|size| sp(size.clone())).collect();
        self.node
            .node_style
            .push(("grid-gap".to_string(), params.join(" ")));
        self
    }
    pub fn justify_content(&mut self, value: &str) -> &mut Self {
        self.node
            .node_style
            .push(("justify-content".to_string(), value.to_string()));
        self
    }
    pub fn flex_wrap(&mut self) -> &mut Self {
        self.node
            .node_style
            .push(("flex-wrap".to_string(), "wrap".to_string()));
        self
    }
}

impl std::ops::Deref for VStack {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

impl std::ops::DerefMut for VStack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.node
    }
}
impl Appendable for VStack {}
impl Renderable for VStack {
    fn render(mut self) -> Node {
        let alignment = match self.alignment {
            Alignment::Center => "center",
            Alignment::Start => "start",
            Alignment::End => "end",
            Alignment::Stretch => "stretch",
        };
        self.add_class("stack")
            .add_class("stack--vertical")
            .add_class(&format!("stack--align-{alignment}",));
        self.node
    }
}

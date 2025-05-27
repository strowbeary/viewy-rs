use crate::{Renderable};
use crate::node::{Node, NodeContainer};
use crate::components::Alignment;
use std::borrow::BorrowMut;
use crate::{sp, DefaultModifiers};
use crate::components::{Appendable, ChildContainer};


#[derive(Debug, Clone)]
pub struct HStack {
    children: Vec<Box<dyn Renderable>>,
    node: Node,
    pub alignment: Alignment,
}

impl Default for HStack {
    fn default() -> Self {
        HStack {
            children: vec![],
            node: Default::default(),
            alignment: Alignment::Stretch,
        }
    }
}

impl NodeContainer for HStack {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers for HStack {}

impl HStack {
    pub fn new(alignment: Alignment) -> Self {
        HStack {
            children: vec![],
            node: Default::default(),
            alignment,
        }
    }
    pub fn justify_content(&mut self, justification: &str) -> &mut Self {
        self.node.node_style.push(("justify-content".to_string(), justification.to_string()));
        self
    }

    pub fn gap(&mut self, gaps: Vec<i32>) -> &mut Self {
        let params: Vec<String> = gaps.iter().map(|size| sp(size.clone())).collect();
        self.node.node_style.push(("grid-gap".to_string(), params.join(" ")));
        self
    }
    pub fn flex_wrap(&mut self) -> &mut Self {
        self.node.node_style.push(("flex-wrap".to_string(), "wrap".to_string()));
        self
    }
}
impl ChildContainer for HStack {
    fn get_children(&mut self) -> &mut Vec<Box<dyn Renderable>> {
        return self.children.borrow_mut();
    }
}
impl Appendable for HStack {}

impl Renderable for HStack {
    fn render(mut self) -> Node {
        let alignment =  format!("stack--align-{:?}", self.alignment)
            .to_lowercase();
        self
            .add_class("stack")
            .add_class("stack--horizontal")
            .add_class(
                &alignment
            );
        self.children.into_iter()
            .for_each(|child|
                self.node.children.push(child.render()));
        self.node
    }
}
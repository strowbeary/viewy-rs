use crate::core::node::Node;
use crate::modifiers::*;
use crate::{sp, Widget};

#[derive(Debug, Clone)]
pub enum Alignment {
    Center,
    Start,
    End,
    Stretch,
}

#[derive(Widget, Appendable, Colorable, Classable)]
pub struct VStack {
    node: Node,
}

impl VStack {
    pub fn new(alignment: Alignment) -> Self {
        let mut stack = VStack {
            node: Node::default()
        };
        stack.add_class("stack")
            .add_class("stack--horizontal")
            .add_class(format!("stack--align-{:?}", alignment).to_lowercase().as_str());
        stack
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
    pub fn render(&mut self) {}
}


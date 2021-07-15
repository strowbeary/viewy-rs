use crate::engine::{Renderable};
use crate::node::{Node, NodeContainer};
use std::borrow::BorrowMut;
use crate::DefaultModifiers;
use std::process::Child;
use crate::components::{Appendable, ChildContainer};

/// This is a simple configurable node
#[derive(Debug, Clone)]
pub struct View {
    children: Vec<Box<dyn Renderable>>,
    pub node: Node,
}

impl View {
    pub fn new() -> Self {
        View {
            children: vec![],
            node: Default::default(),
        }
    }

}

impl NodeContainer for View {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers<View> for View {}

impl ChildContainer for View {
    fn get_children(&mut self) -> &mut Vec<Box<dyn Renderable>> {
        return self.children.borrow_mut();
    }
}

impl Appendable for View {}

impl Renderable for View {
    fn render(&mut self) -> Node {
        let mut rendered_children: Vec<Node> = self.children.iter_mut()
            .map(|child| {
                child.render()
            })
            .collect();
        self.get_node().clone().children.append({
            &mut rendered_children
        });
        self.get_node().clone()
    }
}
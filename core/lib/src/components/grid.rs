use crate::node::{Node, NodeContainer};
use crate::components::Alignment;
use std::borrow::BorrowMut;
use crate::{DefaultModifiers, sp, Renderable};
use crate::components::{Appendable, ChildContainer};


#[derive(Debug, Clone)]
pub struct Grid {
    children: Vec<Box<dyn Renderable>>,
    node: Node,
    pub alignment: Alignment,
}

impl NodeContainer for Grid {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers<Grid> for Grid {}

impl Grid {
    pub fn new(alignment: Alignment) -> Self {
        Grid {
            children: vec![],
            node: Default::default(),
            alignment,
        }
    }
    pub fn gap(&mut self, gaps: Vec<i32>) -> Self {
        let params: Vec<String> = gaps.iter().map(|size| sp(size.clone())).collect();
        self.node.node_style.push(("grid-gap".to_string(), params.join(" ")));
        self.clone()
    }
    pub fn columns(&mut self, schema: &str) -> Self {
        self.node.node_style.push(("grid-template-columns".to_string(), schema.to_string()));
        self.clone()
    }

    pub fn areas(&mut self, schema: &str) -> Self {
        self.node.node_style.push(("grid-template-areas".to_string(), schema.to_string()));
        self.clone()
    }

    pub fn rows(&mut self, schema: &str) -> Self {
        self.node.node_style.push(("grid-template-rows".to_string(), schema.to_string()));
        self.clone()
    }
    pub fn auto_columns(&mut self, size: &str) -> Self {
        self.node.node_style.push(("grid-auto-columns".to_string(), size.to_string()));
        self.clone()
    }
    pub fn auto_rows(&mut self, size: &str) -> Self {
        self.node.node_style.push(("grid-auto-rows".to_string(), size.to_string()));
        self.clone()
    }
    pub fn auto_flow(&mut self, direction: &str) -> Self {
        self.node.node_style.push(("grid-auto-flow".to_string(), direction.to_string()));
        self.clone()
    }
}

impl ChildContainer for Grid {
    fn get_children(&mut self) -> &mut Vec<Box<dyn Renderable>> {
        return self.children.borrow_mut();
    }
}

impl Appendable for Grid {}

impl Renderable for Grid {
    fn render(&self) -> Node {
        let mut view = self
            .clone()
            .add_class("grid")
            .add_class(
                format!("grid--align-{:?}", self.alignment)
                    .to_lowercase()
                    .as_str()
            )
            .node;
        self.children.iter()
            .for_each(|child|
                view.children.push(child.render()));
        view
    }
}
use crate::{Renderable};
use crate::node::{Node, NodeContainer};
use std::borrow::BorrowMut;
use crate::{DefaultModifiers};
use crate::components::{Appendable, ChildContainer};



#[derive(Debug, Clone)]
pub struct Form {
    children: Vec<Box<dyn Renderable>>,
    node: Node,
    pub action: String,
    pub is_async: bool
}

impl NodeContainer for Form {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers<Form> for Form {}

impl Form {
    pub fn new(name: &str, action: &str) -> Self {
        Form {
            children: vec![],
            node: Node::default(),
            action: action.to_string(),
            is_async: false
        }
            .set_attr("id", name)
            .set_attr("method", "POST")
    }

    pub fn async_form(&mut self) -> Self {
        self.is_async = true;
        self.clone()
    }

    pub fn inject_into_dynamic_content(&mut self, dynamic_content_name: &str) -> Self {
        self.set_attr("data-dynamic-content-name", dynamic_content_name)
    }
}

impl ChildContainer for Form {
    fn get_children(&mut self) -> &mut Vec<Box<dyn Renderable>> {
        return self.children.borrow_mut();
    }
}
impl Appendable for Form {}

impl Renderable for Form {
    fn render(&self) -> Node {

        let mut form = self.clone()
            .add_class("form")
            .set_attr("action", &self.action)
            .tag("form");

        if self.is_async {
            form.set_attr("data-async", "data-async");
        }

        let mut node = form.node;

        self.children.iter()
            .for_each(|child|
                node.children.push(child.render()));
        node
    }
}
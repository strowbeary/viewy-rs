use crate::DefaultModifiers;
use crate::Renderable;
use crate::components::{Appendable, ChildContainer};
use crate::node::{Node, NodeContainer};
use std::borrow::BorrowMut;

#[derive(Debug, Clone)]
pub enum FormMethod {
    Post,
    Get,
}

#[derive(Debug, Clone)]
pub struct Form {
    children: Vec<Box<dyn Renderable>>,
    node: Node,
    pub name: String,
    pub action: String,
    pub is_async: bool,
    pub form_method: FormMethod,
}

impl NodeContainer for Form {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers for Form {}

impl Form {
    pub fn new(name: &str, action: &str) -> Self {
        Form {
            children: vec![],
            node: Node::default(),
            name: name.to_string(),
            action: action.to_string(),
            is_async: false,
            form_method: FormMethod::Post,
        }

    }
    pub fn method(&mut self, method: FormMethod) -> &mut Self {
        self.form_method = method;
        self
    }
    pub fn async_form(&mut self) -> &mut Self {
        self.is_async = true;
        self
    }

    pub fn inject_into_dynamic_content(&mut self, dynamic_content_name: &str) -> &mut Self {
        self.set_attr("data-dynamic-content-name", dynamic_content_name)
    }

    pub fn multipart(&mut self) -> &mut Self {
        self.set_attr("enctype", "multipart/form-data")
    }
}

impl ChildContainer for Form {
    fn get_children(&mut self) -> &mut Vec<Box<dyn Renderable>> {
        self.children.borrow_mut()
    }
}
impl Appendable for Form {}

impl Renderable for Form {
    fn render(mut self) -> Node {
        let method = match self.form_method {
            FormMethod::Post => "POST",
            FormMethod::Get => "GET",
        };
        let name = self.name.to_string();
        let action = self.action.to_string();
        self
            .set_attr("id", &name)
            .add_class("form")
            .set_attr("action", &action)
            .tag("form")
            .set_attr(
                "method",
                method
            );


        if self.is_async {
            self.set_attr("data-async", "data-async");
        }



        self.children
            .into_iter()
            .for_each(|child| self.node.children.push(child.render()));
        self.node
    }
}

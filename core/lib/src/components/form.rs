use crate::renderer::{Renderable, ToHtml, StyleRegistry, ScriptRegistry};
use crate::node::{Node, NodeContainer};
use std::borrow::BorrowMut;
use crate::{DefaultModifiers, scale};
use crate::components::{Icon, Text, TextStyle};


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
impl ToHtml for Form {}

impl Form {
    pub fn new(name: &str, action: &str) -> Self {
        Form {
            children: vec![],
            node: Node::default(),
            action: action.to_string(),
            is_async: false
        }
            .set_attr("id", name)
    }
    pub fn async_form(&mut self) -> Self {
        self.is_async = true;
        self.clone()
    }

    pub fn append_child<'a, T>(&'a mut self, child: T)
        where
            T: 'static + Renderable,
    {
        self.children.push(Box::new(child));
    }
}

impl Renderable for Form {
    fn render(&self, style_registery: &mut StyleRegistry, script_registery: &mut ScriptRegistry) -> Node {
        style_registery.register_stylesheet(
            "form",
            include_str!("../themes/components/form.scss"),
        );
        if self.is_async {
            script_registery.register_script(
                "form",
                include_str!("../js/async-form.js"),
            );
        }
        let mut form = self.clone()
            .add_class("form")
            .set_attr("action", &self.action)
            .set_attr("data-async", "data-async")
            .tag("form");

        let mut node = form.node;

        self.children.iter()
            .for_each(|child|
                node.children.push(child.render(style_registery, script_registery)));
        node
    }
}
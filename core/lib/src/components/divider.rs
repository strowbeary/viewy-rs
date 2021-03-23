use crate::renderer::{Renderable, ToHtml, StyleRegistry, ScriptRegistry};
use crate::node::{Node, NodeContainer};
use std::borrow::BorrowMut;
use crate::DefaultModifiers;
use crate::component::{Appendable, ChildContainer};

#[derive(Debug, Clone)]
pub struct Divider {
    node: Node,
}

impl NodeContainer for Divider {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers<Divider> for Divider {}
impl ToHtml for Divider {}



impl Divider {
    pub fn new() -> Self {
        Divider {
            node: Node::default(),
        }
    }
}

impl Renderable for Divider {
    fn render(&self, style_registery: &mut StyleRegistry, script_registery: &mut ScriptRegistry) -> Node {
        style_registery.register_stylesheet(
            "divider",
            include_str!("../themes/components/divider.scss"),
        );
        let mut view = self.clone()
            .add_class("divider")
            .node;
        view
    }
}
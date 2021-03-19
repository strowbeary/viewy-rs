use crate::renderer::{Renderable, StyleRegistry, ScriptRegistry, ToHtml};
use crate::node::{Node, NodeContainer};
use std::borrow::BorrowMut;
use crate::DefaultModifiers;
use crate::component::{Appendable, ChildContainer};
use std::process::Child;

#[derive(Debug, Clone)]
pub struct View {
    children: Vec<Box<dyn Renderable>>,
    pub node: Node,
}
impl NodeContainer for View {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers<View> for View {}


impl ToHtml for View {}

impl View {
    pub fn new() -> Self {
        View {
            children: vec![],
            node: Default::default(),
        }
    }

}

impl ChildContainer for View {
    fn get_children(&mut self) -> &mut Vec<Box<dyn Renderable>> {
        return self.children.borrow_mut();
    }
}

impl Appendable<View> for View {}

impl Renderable for View {
    fn render(&self, style_registery: &mut StyleRegistry, script_registery: &mut ScriptRegistry) -> Node {

        let mut node = self.clone().node;

        self.children.iter()
            .for_each(|child|
                node.children.push(child.render(style_registery, script_registery)));
        node
    }
}
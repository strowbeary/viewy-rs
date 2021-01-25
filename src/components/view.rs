use crate::node::{Node, DefaultModifiers, NodeContainer};
use crate::helper_fn::sp;
use crate::{Renderable, StyleRegistry};
use crate::template_compilation_tools::ScriptRegistry;
use std::borrow::BorrowMut;


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

impl View {
    pub fn new() -> Self {
        View {
            children: vec![],
            node: Default::default(),
        }
    }

    pub fn add_view_child<'a, T>(&'a mut self, child: T)
        where
            T: 'static + Renderable,
    {
        self.children.push(Box::new(child));
    }

}

impl Renderable for View {
    fn render(&self, style_registery: &mut StyleRegistry, script_registery: &mut ScriptRegistry) -> Node {
        style_registery.register_stylesheet(
            "stack",
            include_str!("../themes/components/stack.scss"),
        );
        let mut node = self.clone().node;

        self.children.iter()
            .for_each(|child|
                node.children.push(child.render(style_registery, script_registery)));
        node
    }
}
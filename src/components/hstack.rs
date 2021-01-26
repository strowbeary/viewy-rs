use crate::node::{Node, DefaultModifiers, NodeContainer};
use crate::helper_fn::sp;
use crate::{Renderable, StyleRegistry};
use crate::template_compilation_tools::ScriptRegistry;
use std::borrow::BorrowMut;
use crate::components::Alignment;

#[derive(Debug, Clone)]
pub struct HStack {
    children: Vec<Box<dyn Renderable>>,
    node: Node,
    alignment: Alignment,
}

impl Default for HStack {
    fn default() -> Self {
        HStack {
            children: vec![],
            node: Default::default(),
            alignment: Alignment::Stretch
        }
    }
}

impl NodeContainer for HStack {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers<HStack> for HStack {}

impl HStack {
    pub fn new(alignment: Alignment) -> Self {
        HStack {
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
    pub fn add_view_child<'a, T>(&'a mut self, child: T)
        where
            T: 'static + Renderable,
    {
        self.children.push(Box::new(child));
    }

}

impl Renderable for HStack {
    fn render(&self, style_registery: &mut StyleRegistry, script_registery: &mut ScriptRegistry) -> Node {
        style_registery.register_stylesheet(
            "stack",
            include_str!("../themes/components/stack.scss"),
        );
        let mut view = self
            .clone()
            .add_class("stack")
            .add_class("stack--horizontal")
            .add_class(
                format!("stack--align-{:?}", self.alignment)
                    .to_lowercase()
                    .as_str()
            )
            .node;
        self.children.iter()
            .for_each(|child|
                view.children.push(child.render(style_registery, script_registery)));
        view
    }
}
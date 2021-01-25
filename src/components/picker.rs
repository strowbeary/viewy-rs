use crate::node::{Node, DefaultModifiers, NodeContainer};
use crate::helper_fn::sp;
use crate::{Renderable, StyleRegistery};
use crate::template_compilation_tools::ScriptRegistry;
use std::borrow::BorrowMut;

#[derive(Debug, Clone)]
pub enum PickerStyle {
    Segmented,
    Dropdown,
    RadioGroup
}

#[derive(Debug, Clone)]
pub struct Picker {
    children: Vec<Box<dyn Renderable>>,
    pub node: Node,
    pub style: PickerStyle,
    pub label: String,

}
impl NodeContainer for Picker {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers<Picker> for Picker {}

impl Picker {
    pub fn new() -> Self {
        Picker {
            children: vec![],
            node: Default::default(),
        }
    }

    pub fn add_view_child<'a, T>(&'a mut self, child: Box<T>)
        where
            T: 'static + Renderable,
    {
        self.children.push(child);
    }



}

impl Renderable for Picker {
    fn render(&self, style_registery: &mut StyleRegistery, script_registery: &mut ScriptRegistry) -> Node {

        let mut node = self.clone().node;

        self.children.iter()
            .for_each(|child|
                node.children.push(child.render(style_registery, script_registery)));
        node
    }
}
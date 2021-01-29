use crate::renderer::{Renderable, StyleRegistry, ScriptRegistry, ToHtml};
use crate::node::{Node, NodeContainer, NodeType};
use crate::DefaultModifiers;
use std::borrow::BorrowMut;
use crate::components::View;

#[derive(Debug, Clone)]
pub struct Popover {
    children: Vec<Box<dyn Renderable>>,
    node: Node,
    pub el_to_attach_to: String,
    pub placement: String
}

impl NodeContainer for Popover {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers<Popover> for Popover {}


impl ToHtml for Popover {}

impl Popover {
    pub fn new() -> Self {
        Popover {
            children: vec![],
            node: Default::default(),
            el_to_attach_to: "".to_string(),
            placement: "auto".to_string()
        }
    }

    pub fn attach_to(&mut self, el: &str) -> Self {
        self.el_to_attach_to = el.to_string();
        self.clone()
    }

    pub fn placement(&mut self, placement: &str) -> Self {
        self.placement = placement.to_string();
        self.clone()
    }

    pub fn add_view_child<'a, T>(&'a mut self, child: T)
        where
            T: 'static + Renderable,
    {
        self.children.push(Box::new(child));
    }
}

impl Renderable for Popover {
    fn render(&self, style_registery: &mut StyleRegistry, script_registery: &mut ScriptRegistry) -> Node {

        style_registery.register_stylesheet(
            "popover",
            include_str!("../themes/components/popover.scss"),
        );
        script_registery.register_script(
            "popover",
            include_str!("../js/popover.js"),
        );
        let mut popover = self.clone()
            .add_class("popover")
            .set_attr("data-attach-to", self.el_to_attach_to.as_str());

        let arrow = View::new()
            .add_class("arrow")
            .set_attr("data-popper-arrow", "true");
        self.children.iter()
            .for_each(|child| {
                popover.node.children.push(child.render(style_registery, script_registery))
            });
        popover.node.children.push(arrow.render(style_registery, script_registery));

        popover.node
    }
}
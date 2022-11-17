use crate::{Renderable};
use crate::node::{Node, NodeContainer, NodeType};
use crate::DefaultModifiers;
use std::borrow::BorrowMut;
use crate::components::*;

#[derive(Debug, Clone)]
pub enum Placement {
    Auto,
    TopStart,
    Top,
    TopEnd,
    RightStart,
    Right,
    RightEnd,
    BottomStart,
    Bottom,
    BottomEnd,
    LeftStart,
    Left,
    LeftEnd
}

#[derive(Debug, Clone)]
pub struct Popover {
    children: Vec<Box<dyn Renderable>>,
    node: Node,
    hide_arrow: bool,
    pub el_to_attach_to: String,
    pub placement: Placement
}

impl NodeContainer for Popover {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers<Popover> for Popover {}

impl Popover {
    pub fn new() -> Self {
        Popover {
            children: vec![],
            node: Default::default(),
            hide_arrow: false,
            el_to_attach_to: "".to_string(),
            placement: Placement::Auto
        }
    }

    pub fn hide_arrow(&mut self) -> Self {
        self.hide_arrow = true;
        self.clone()
    }

    pub fn attach_to(&mut self, el: &str) -> Self {
        self.el_to_attach_to = el.to_string();
        self.clone()
    }

    pub fn placement(&mut self, placement: Placement) -> Self {
        self.placement = placement;
        self.clone()
    }
}

impl ChildContainer for Popover {
    fn get_children(&mut self) -> &mut Vec<Box<dyn Renderable>> {
        return self.children.borrow_mut();
    }
}
impl Appendable for Popover {}


impl Renderable for Popover {
    fn render(&self) -> Node {
        let mut popover = self.clone()
            .add_class("popover")
            .set_attr("data-attach-to", self.el_to_attach_to.as_str())
            .set_attr("data-placement", match self.placement {
                Placement::Auto => "auto",
                Placement::TopStart => "top-start",
                Placement::Top => "top",
                Placement::TopEnd => "top-end",
                Placement::RightStart => "right-start",
                Placement::Right => "right",
                Placement::RightEnd => "right-end",
                Placement::BottomStart => "bottom-start",
                Placement::Bottom => "bottom",
                Placement::BottomEnd => "bottom-end",
                Placement::LeftStart => "left-start",
                Placement::Left => "left",
                Placement::LeftEnd => "left-end",
            });

        self.children.iter()
            .for_each(|child| {
                popover.node.children.push(child.render())
            });
        if !self.hide_arrow {
            popover.node.children.push({
                View::new()
                    .add_class("arrow")
                    .set_attr("data-popper-arrow", "true")
                    .render()
            });
        }

        popover.node
    }
}
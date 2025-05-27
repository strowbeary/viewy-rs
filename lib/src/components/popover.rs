use crate::{Renderable};
use crate::node::{Node, NodeContainer};
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

impl DefaultModifiers for Popover {}

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

    pub fn hide_arrow(&mut self) -> &mut Self {
        self.hide_arrow = true;
        self
    }

    pub fn attach_to(&mut self, el: &str) -> &mut Self {
        self.el_to_attach_to = el.to_string();
        self
    }

    pub fn placement(&mut self, placement: Placement) -> &mut Self {
        self.placement = placement;
        self
    }
}

impl ChildContainer for Popover {
    fn get_children(&mut self) -> &mut Vec<Box<dyn Renderable>> {
        return self.children.borrow_mut();
    }
}
impl Appendable for Popover {}


impl Renderable for Popover {
    fn component_name(&self) -> &str {
        "Popover"
    }
    fn render(mut self) -> Node {
        let placement = match self.placement {
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
        };
        self
            .add_class("popover")
            .set_attr("data-attach-to", self.el_to_attach_to.as_str())
            .set_attr("data-placement", placement);

        self.children.into_iter()
            .for_each(|child| {
                self.node.children.push(child.render())
            });
        if !self.hide_arrow {
            self.node.children.push({
                let mut view = View::new();
                view.add_class("arrow")
                    .set_attr("data-popper-arrow", "true");
                view.render()
            });
        }

        self.node
    }
}
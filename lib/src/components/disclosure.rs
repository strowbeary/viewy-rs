use std::borrow::BorrowMut;
use crate::components::{Alignment, Appendable, ChildContainer, HStack, Icon, icons::Lucide};

use crate::{Renderable, scale};
use crate::DefaultModifiers;
use crate::node::{Node, NodeContainer};

/// Draw a horizontal line to separate content.
#[derive(Debug, Clone)]
pub struct Disclosure {
    node: Node,
    pub opener_item: Option<Box<dyn Renderable>>,
    children: Vec<Box<dyn Renderable>>,
    open: bool,
}

impl NodeContainer for Disclosure {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers<Disclosure> for Disclosure {}

impl ChildContainer for Disclosure {
    fn get_children(&mut self) -> &mut Vec<Box<dyn Renderable>> {
        return self.children.borrow_mut();
    }
}

impl Appendable for Disclosure {}

impl Disclosure {
    pub fn new() -> Self {
        Disclosure {
            node: Node::default(),
            opener_item: None,
            children: vec![],
            open: false,
        }
    }
    pub fn opener_item<T>(&mut self, item: T) -> Self
        where
            T: 'static + Renderable,
    {
        self.opener_item = Some(Box::new(item));
        self.clone()
    }


    pub fn open(&mut self, is_open: bool) -> Self {
        self.open = is_open;
        self.clone()
    }
}

impl Renderable for Disclosure {
    fn render(&self) -> Node {
        let mut view = self.clone()
            .tag("details")
            .add_class("disclosure");
        if self.open {
            view.set_attr("open", "open");
        }

        if let Some(opener_item) = self.opener_item.clone() {
            view.node.children.push(HStack::new(Alignment::Center)
                .gap(vec![scale(3)])
                .tag("summary")
                .add_class("clickable")
                .append_child(opener_item)
                .append_child({
                    Icon::new(Lucide::ChevronDown).stroke_width(2)
                })
                .render());
        }
        self.children.iter()
            .for_each(|child|
                view.node.children.push(child.render()));
        view.node
    }
}
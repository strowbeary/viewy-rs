use crate::{Renderable, DefaultModifiers};
use crate::node::{Node, NodeContainer};
use std::borrow::BorrowMut;
use crate::components::{ChildContainer, Appendable, View};

#[derive(Debug, Clone)]
pub struct Popup {
    children: Vec<Box<dyn Renderable>>,
    node: Node,
    pub el_to_attach_to: String,
}

impl NodeContainer for Popup {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers<Popup> for Popup {}

impl Popup {
    pub fn new() -> Self {
        Popup {
            children: vec![],
            node: Default::default(),
            el_to_attach_to: "".to_string(),
        }
    }

    pub fn attach_to(&mut self, el: &str) -> Self {
        self.el_to_attach_to = el.to_string();
        self.clone()
    }
    pub fn open(&mut self, is_open: bool) -> Self {
        {
            if is_open {
                self.add_class("visible")
            } else {
                self.remove_class("visible")
            }
        }.clone()
    }
}

impl ChildContainer for Popup {
    fn get_children(&mut self) -> &mut Vec<Box<dyn Renderable>> {
        return self.children.borrow_mut();
    }
}

impl Appendable for Popup {}


impl Renderable for Popup {
    fn render(&mut self) -> Node {
        let mut popup = self.clone()
            .add_class("popup")
            .set_attr("data-attach-to", self.el_to_attach_to.as_str());
        let mut window = View::new()
            .add_class("popup__window");
        self.children.iter()
            .for_each(|child| {
                window.node.children.push(child.render())
            });


        popup.node.children.push({
            window.render()
        });

        popup.node
    }
}
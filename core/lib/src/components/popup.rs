use std::borrow::BorrowMut;

use crate::{DefaultModifiers, Renderable};
use crate::components::{Alignment, Appendable, Button, ButtonStyle, ChildContainer, HStack, View};
use crate::node::{Node, NodeContainer};

#[derive(Debug, Clone)]
pub struct Popup {
    children: Vec<Box<dyn Renderable>>,
    node: Node,
    pub el_to_attach_to: String,
    pub window_controls: bool,
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
            window_controls: true,
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

    pub fn hide_window_controls(&mut self) -> Self {
        self.window_controls = false;
        self.clone()
    }
}

impl ChildContainer for Popup {
    fn get_children(&mut self) -> &mut Vec<Box<dyn Renderable>> {
        return self.children.borrow_mut();
    }
}

impl Appendable for Popup {}


impl Renderable for Popup {
    fn render(&self) -> Node {
        let mut popup = self.clone()
            .add_class("popup")
            .set_attr("data-attach-to", self.el_to_attach_to.as_str());
        let mut window = View::new()
            .add_class("popup__window");

        if self.window_controls {
            window.node.children.push({
                View::new()
                    .add_class("popup__window-bar")
                    .append_child({
                        Button::icon_only("x", ButtonStyle::Link)
                            .add_class("popup__window-controls")
                    })
            }.render());
        }

        window.node.children.push({
            let mut content = View::new()
                .add_class("popup__window-content");
            self.children.iter()
                .for_each(|child| {
                    content.append_child(child.clone());
                });
            content.render()
        });


        popup.node.children.push({
            window.render()
        });

        popup.node
    }
}
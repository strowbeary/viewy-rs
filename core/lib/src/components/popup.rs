use std::borrow::BorrowMut;

use crate::{DefaultModifiers, Renderable};
use crate::components::{Appendable, Button, ButtonStyle, ChildContainer, View};
use crate::node::{Node, NodeContainer};

#[derive(Debug, Clone)]
pub struct Popup {
    children: Vec<Box<dyn Renderable>>,
    node: Node,
    pub el_to_attach_to: String,
    form_to_submit: Option<String>,
    pub window_controls: bool,
    pub open: bool,
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
            form_to_submit: None,
            window_controls: true,
            open: false,
        }
    }

    pub fn attach_to(&mut self, el: &str) -> Self {
        self.el_to_attach_to = el.to_string();
        self.clone()
    }
    pub fn open(&mut self, is_open: bool) -> Self {
        self.open = is_open;
        self.clone()
    }

    pub fn hide_window_controls(&mut self) -> Self {
        self.window_controls = false;
        self.clone()
    }

    pub fn submit_form_on_open(&mut self, form_name: &str) -> Self {
        self.form_to_submit = Some(form_name.to_string());
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
        let mut popup = View::new()
            .add_class("popup")
            .set_attr("data-attach-to", self.el_to_attach_to.as_str());
        if let Some(form_name) = &self.form_to_submit {
            popup.set_attr("data-attached-form", form_name);
        }
        if self.open {
            popup.add_class("visible");
        } else {
            popup.remove_class("visible");
        }
        let mut window = self.clone()
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
            window.node
        });

        popup.node
    }
}
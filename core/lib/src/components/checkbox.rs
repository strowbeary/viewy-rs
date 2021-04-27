use crate::node::{Node, NodeContainer};
use std::borrow::BorrowMut;
use crate::{DefaultModifiers, scale};
use crate::renderer::Renderable;
use crate::components::*;

use std::ops::Deref;


#[derive(Debug, Clone)]
pub struct Checkbox {
    node: Node,
    label: Option<String>,
    name: String,
    value: String,
    checked: bool,
    children: Vec<Box<dyn Renderable>>,
}

impl NodeContainer for Checkbox {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers<Checkbox> for Checkbox {}

impl ChildContainer for Checkbox {
    fn get_children(&mut self) -> &mut Vec<Box<dyn Renderable>> {
        return self.children.borrow_mut();
    }
}

impl Appendable for Checkbox {}

impl Checkbox {
    pub fn new(name: &str, value: &str) -> Self {
        Self {
            node: Default::default(),
            label: None,
            name: name.to_string(),
            value: value.to_string(),
            checked: false,
            children: vec![],
        }
    }

    pub fn label(&mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self.clone()
    }

    pub fn is_checked(&mut self, checked: bool) -> Self {
        self.checked = checked;
        self.clone()
    }
}


impl Renderable for Checkbox {
    fn render(&self) -> Node {
        let radio_id = format!("checkbox-{}", self.name);

        let mut checkbox = View::new()
            .tag("input")
            .set_attr("type", "checkbox")
            .set_attr("name", self.name.as_str())
            .set_attr("id", radio_id.as_str());


        if self.checked {
            checkbox.set_attr("checked", "checked");
        }

        let mut container = self.clone()
            .add_class("checkbox")
            .append_child(checkbox);
        if let Some(label) = self.clone().label {
            container.append_child({
                Text::new(label.as_str(), TextStyle::Body)
                    .set_attr("for", radio_id.as_str())
                    .tag("label")
            });
        }
        container.clone().children.iter()
            .for_each(|child|
                container.node.children.push(child.render()));
        container.node
    }
}
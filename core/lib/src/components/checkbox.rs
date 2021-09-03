use crate::node::{Node, NodeContainer, NodeType};
use std::borrow::BorrowMut;
use crate::{DefaultModifiers, scale};
use crate::Renderable;
use crate::components::*;
use uuid::Uuid;

use std::ops::Deref;

/// Used to manipulate boolean values or to choose to include a value to a form submission.
#[derive(Debug, Clone)]
pub struct Checkbox {
    node: Node,
    label: Option<String>,
    name: String,
    value: String,
    checked: bool,
    children: Vec<Box<dyn Renderable>>,
}

impl Checkbox {
    /// Create a new checkbox
    pub fn new(name: &str, value: &str) -> Self {
        Self {
            node: Node::default(),
            label: None,
            name: name.to_string(),
            value: value.to_string(),
            checked: false,
            children: vec![],
        }
    }

    /// Define an optionnal label
    pub fn label(&mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self.clone()
    }

    /// Define the default state
    pub fn is_checked(&mut self, checked: bool) -> Self {
        self.checked = checked;
        self.clone()
    }
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

impl Renderable for Checkbox {
    fn render(&self) -> Node {
        let radio_id = Uuid::new_v4().to_hyphenated().to_string();

        let mut checkbox = View::new()
            .tag("input")
            .set_attr("type", "checkbox")
            .set_attr("name", &self.name)
            .set_attr("value", &self.value)
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
                    .margin_left(4)
            });
        }
        container.clone().children.iter()
            .for_each(|child|
                container.node.children.push(child.render()));
        container.node
    }
}
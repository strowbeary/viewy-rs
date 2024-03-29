use std::borrow::BorrowMut;
use std::ops::Deref;

use uuid::Uuid;

use crate::{DefaultModifiers, scale};
use crate::components::*;
use crate::node::{Node, NodeContainer, NodeType};
use crate::Renderable;

#[derive(Debug, Clone)]
pub enum CheckboxStyle {
    Checkbox,
    Switch,
}

/// Used to manipulate boolean values or to choose to include a value to a form submission.
#[derive(Debug, Clone)]
pub struct Checkbox {
    node: Node,
    style: CheckboxStyle,
    label: Option<String>,
    name: String,
    value: String,
    checked: bool,
    auto_submit: bool,
    children: Vec<Box<dyn Renderable>>,
}

impl Checkbox {
    /// Create a new checkbox
    pub fn new(name: &str, value: &str, style: CheckboxStyle) -> Self {
        Self {
            node: Node::default(),
            style,
            label: None,
            name: name.to_string(),
            value: value.to_string(),
            checked: false,
            auto_submit: false,
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

    pub fn auto_submit(&mut self, is_auto_submit: bool) -> Self {
        if is_auto_submit {
            self.auto_submit = is_auto_submit;
        }
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
        let radio_id = Uuid::new_v4().to_string();

        let mut checkbox = View::new()
            .tag("input")
            .set_attr("type", "checkbox")
            .set_attr("name", &self.name)
            .set_attr("value", &self.value)
            .set_attr("id", radio_id.as_str());


        if self.checked {
            checkbox.set_attr("checked", "checked");
        }
        if self.auto_submit {
            checkbox.set_attr("data-auto-submit", &self.auto_submit.to_string());
        }

        let mut container = self.clone()
            .add_class("checkbox")
            .add_class(match self.style {
                CheckboxStyle::Checkbox => "checkbox--checkbox",
                CheckboxStyle::Switch => "checkbox--switch"
            })
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
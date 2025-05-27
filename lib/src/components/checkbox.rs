use std::borrow::BorrowMut;

use uuid::Uuid;

use crate::components::*;
use crate::DefaultModifiers;
use crate::node::{Node, NodeContainer};
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
    id: Uuid,
    children: Vec<Box<dyn Renderable>>,
    form: Option<String>,
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
            id: Uuid::new_v4(),
            children: vec![],
            form: None,
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

    pub fn id(&mut self, id: Uuid) -> Self {
        self.id = id;
        self.clone()
    }

    pub fn attach_to_form(&mut self, form_name: &str) -> Self {
        self.form = Some(form_name.to_string());
        self.clone()
    }
}

impl NodeContainer for Checkbox {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers for Checkbox {}

impl ChildContainer for Checkbox {
    fn get_children(&mut self) -> &mut Vec<Box<dyn Renderable>> {
        return self.children.borrow_mut();
    }
}

impl Appendable for Checkbox {}

impl Renderable for Checkbox {
    fn render(mut self) -> Node {
        let mut checkbox = View::new();
        checkbox.tag("input")
            .set_attr("type", "checkbox")
            .set_attr("name", &self.name)
            .set_attr("value", &self.value)
            .set_attr("id", &self.id.to_string());

        if let Some(form) = &self.form {
            checkbox.set_attr("form", form);
        }

        if self.checked {
            checkbox.set_attr("checked", "checked");
        }
        if self.auto_submit {
            checkbox.set_attr("data-auto-submit", &self.auto_submit.to_string());
        }
        let checkbox_style = match &self.style {
            CheckboxStyle::Checkbox => "checkbox--checkbox",
            CheckboxStyle::Switch => "checkbox--switch"
        };
        self
            .add_class("checkbox")
            .add_class(checkbox_style)
            .append_child(checkbox);
        if let Some(label) = &self.label {
            self.append_child({
                let mut text = Text::new(label, TextStyle::Body);
                text.set_attr("for", &self.id.to_string())
                    .tag("label")
                    .margin_left(4);
                text
            });
        }
        let child_nodes = self.children.into_iter().map(|child| child.render()).collect();

        self.node.children = child_nodes;
        self.node
    }
}
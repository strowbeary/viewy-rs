use std::borrow::BorrowMut;

use uuid::Uuid;

use crate::components::{Button, Text, TextStyle, View};
use crate::DefaultModifiers;
use crate::node::{Node, NodeContainer};
use crate::Renderable;

#[derive(Debug, Clone)]
pub enum FieldType {
    Text,
    Password,
    Email,
    Number,
    Tel,
    Date,
    DateTimeLocal,
    Month,
    Search,
    Time,
    Url,
    Week,
    Hidden,
    TextArea
}


#[derive(Debug, Clone)]
pub struct TextField {
    node: Node,
    pub label: Option<String>,
    pub value: Option<String>,
    pub helper_text: Option<String>,
    pub placeholder: Option<String>,
    pub leading_icon: Option<String>,
    pub trailing_icon: Option<String>,
    pub field_type: FieldType,
    pub name: String,
    pub auto_sizing: bool,
    pub datalist: bool,
    pub required: bool,
}

impl NodeContainer for TextField {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers<TextField> for TextField {}

impl TextField {
    pub fn new(name: &str, field_type: FieldType) -> Self {
        TextField {
            node: Default::default(),
            label: None,
            value: None,
            helper_text: None,
            placeholder: None,
            leading_icon: None,
            trailing_icon: None,
            field_type,
            name: name.to_string(),
            auto_sizing: false,
            datalist: false,
            required: false,
        }
    }

    pub fn label(&mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self.clone()
    }

    pub fn value(&mut self, value: &str) -> Self {
        self.value = Some(value.to_string());
        self.clone()
    }

    pub fn helper_text(&mut self, helper_text: &str) -> Self {
        self.helper_text = Some(helper_text.to_string());
        self.clone()
    }

    pub fn error_message(&mut self, helper_text: &str) -> Self {
        self.add_class("textfield--error");
        self.helper_text = Some(helper_text.to_string());
        self.clone()
    }

    pub fn placeholder(&mut self, placeholder: &str) -> Self {
        self.placeholder = Some(placeholder.to_string());
        self.clone()
    }

    pub fn trailing_icon(&mut self, name: &str) -> Self {
        self.trailing_icon = Some(name.to_string());
        self.clone()
    }

    pub fn leading_icon(&mut self, name: &str) -> Self {
        self.leading_icon = Some(name.to_string());
        self.clone()
    }

    pub fn async_datalist(&mut self, url: &str) -> Self {
        self.datalist = true;
        self.set_attr("data-async-datalist", url)
    }

    pub fn required(&mut self, is_required: bool) -> Self {
        self.required = is_required;
        self.clone()
    }
}

impl Renderable for TextField {
    fn render(&self) -> Node {
        let mut field = self.clone()
            .add_class("textfield");

        let mut input = View::new()
            .tag(&match &self.field_type {
                FieldType::TextArea => "textarea",
                _ => "input",
            })
            .add_class("textfield__input")

            .set_attr("id", self.name.as_str())
            .set_attr("name", self.name.as_str());

        if self.required {
            input.set_attr("required", "required");
            field.node.children.push({
                Text::new("Requis", TextStyle::Caption)
                    .color("var(--color-text-secondary)")
                    .grid_area("required")
                    .render()
            });
        }

        match &self.field_type {
            FieldType::TextArea => {}
            _ => {
                input.set_attr("type", &match &field.field_type {
                    FieldType::DateTimeLocal => { "datetime-local".to_string() }
                    field_type => {
                        format!("{:?}", field_type).to_lowercase()
                    }
                });
            }
        }
        if self.datalist && !matches!(field.field_type, FieldType::TextArea) {
            let id = Uuid::new_v4().to_hyphenated().to_string();
            input.set_attr("list", id.as_str());
        }

        if let Some(value) = field.value {
            match &self.field_type {
                FieldType::TextArea => {
                    input.node.text = Some(value);
                }
                _ => {
                    input.set_attr("value", &value);
                }
            }

        }

        field.node.children.push(input.render());

        if !matches!(field.field_type, FieldType::Hidden) {
            if let Some(placeholder) = field.placeholder {
                input.set_attr("placeholder", placeholder.as_str());
            }

            if let Some(label) = field.label {
                let text = Text::new(label.as_str(), TextStyle::Label)
                    .add_class("textfield__label")
                    .set_attr("for", self.name.as_str())
                    .tag("label");
                field.node.children.push(text.render());
            }
        }

        if !matches!(field.field_type, FieldType::Hidden) {
            if let Some(helper_text) = field.helper_text {
                let text = Text::new(helper_text.as_str(), TextStyle::Caption)
                    .add_class("textfield__helper-text");
                field.node.children.push(text.render());
            }
        }
        field.node
    }
}
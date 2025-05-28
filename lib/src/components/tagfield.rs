use crate::DefaultModifiers;
use crate::Renderable;
use crate::components::{Text, TextStyle, View};
use crate::node::Node;
use std::borrow::BorrowMut;

#[derive(Debug, Clone)]
pub struct TagField {
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
}

impl DefaultModifiers for TagField {}

impl TagField {
    pub fn new(name: &str, field_type: FieldType) -> Self {
        TagField {
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
        self.add_class("tagfield--error");
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
}

impl Renderable for TagField {
    fn render(mut self) -> Node {
        let mut field = self.clone().add_class("tagfield");

        let mut input = View::new()
            .tag("input")
            .add_class("tagfield__input")
            .set_attr(
                "type",
                format!("{:?}", field.field_type).to_lowercase().as_str(),
            )
            .set_attr("id", self.name.as_str())
            .set_attr("name", self.name.as_str());

        if let Some(value) = field.value {
            input.set_attr("value", &value);
        }

        if let Some(placeholder) = field.placeholder {
            input.set_attr("placeholder", placeholder.as_str());
        }

        if let Some(label) = field.label {
            let mut text = Text::new(label.as_str(), TextStyle::Label);
            text.add_class("tagfield__label")
                .set_attr("for", self.name.as_str())
                .tag("label");
            field.node.children.push(text.render());
        }
        field.node.children.push(input.render());
        if let Some(helper_text) = field.helper_text {
            let text = Text::new(helper_text.as_str(), TextStyle::Caption)
                .add_class("tagfield__helper-text");
            field.node.children.push(text.render());
        }
        field.node
    }
}

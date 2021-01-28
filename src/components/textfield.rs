use crate::{StyleRegistry, Renderable};
use std::sync::Mutex;
use crate::template_compilation_tools::ScriptRegistry;
use crate::node::{Node, DefaultModifiers, NodeContainer};
use std::borrow::BorrowMut;
use crate::components::{Text, TextStyle, View};

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
}

#[derive(Debug, Clone)]
pub struct TextField {
    node: Node,
    pub label: Option<String>,
    pub helper_text: Option<String>,
    pub placeholder: Option<String>,
    pub leading_icon: Option<String>,
    pub trailing_icon: Option<String>,
    pub field_type: FieldType,
    pub name: String,
    pub auto_sizing: bool,
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
            helper_text: None,
            placeholder: None,
            leading_icon: None,
            trailing_icon: None,
            field_type,
            name: name.to_string(),
            auto_sizing: false
        }
    }

    pub fn label(&mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self.clone()
    }

    pub fn helper_text(&mut self, helper_text: &str) -> Self {
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

impl Renderable for TextField {
    fn render(&self, style_registery: &mut StyleRegistry, script_registery: &mut ScriptRegistry) -> Node {
        style_registery.register_stylesheet(
            "textfield",
            include_str!("../themes/components/textfield.scss"),
        );
        let mut field = self.clone()
            .add_class("textfield");

        let mut input = View::new()
            .tag("input")
            .add_class("textfield__input")
            .set_attr("type", format!("{:?}", field.field_type).to_lowercase().as_str())
            .set_attr("id", self.name.as_str());

        if let Some(placeholder) = field.placeholder {
            input.set_attr("placeholder", placeholder.as_str());
        }

        if let Some(label) = field.label {
            let text = Text::new(label.as_str(), TextStyle::Label)
                .set_attr("for", self.name.as_str())
                .tag("label");
            field.node.children.push(text.render(style_registery, script_registery));
        }
        field.node.children.push(input.render(style_registery, script_registery));
        if let Some(helper_text) = field.helper_text {
            let text = Text::new(helper_text.as_str(), TextStyle::Caption)
                .color("var(--color-text-secondary)");
            field.node.children.push(text.render(style_registery, script_registery));
        }
        field.node
    }
}
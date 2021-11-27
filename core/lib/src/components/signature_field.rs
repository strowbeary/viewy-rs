use crate::{Renderable};
use crate::node::{Node, NodeContainer};
use std::borrow::BorrowMut;
use uuid::Uuid;
use crate::{DefaultModifiers};
use crate::components::{Appendable, ChildContainer, View, Text, TextStyle, Card, CardStyle, Button, ButtonStyle, Popup};



#[derive(Debug, Clone)]
pub struct SignatureField {
    node: Node,
    pub label: Option<String>,
    pub name: String,
    pub id: String,
}

impl NodeContainer for SignatureField {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers<SignatureField> for SignatureField {}

impl SignatureField {
    pub fn new(name: &str) -> Self {
        SignatureField {
            node: Node::default(),
            label: None,
            name: name.to_string(),
            id: Uuid::new_v4().to_hyphenated().to_string()
        }
    }

    pub fn label(&mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self.clone()
    }
}


impl Renderable for SignatureField {
    fn render(&self) -> Node {

        let mut field = self.clone()
            .add_class("signature-field")
            .set_attr("data-signature-field-id", &self.id);

        if let Some(label) = field.label {
            let text = Text::new(label.as_str(), TextStyle::Label)
                .add_class("signature-field__label")
                .set_attr("for", self.name.as_str())
                .tag("label");
            field.node.children.push(text.render());
        }
        field.node.children.push({
           Button::new("Signer", ButtonStyle::Filled)
               .popup({
                   Popup::new()
                       .append_child({
                           View::new()
                               .add_class("signature-field__canvas")
                               .tag("canvas")
                               .set_attr("id", &format!("signature-field-{}__canvas", self.id))
                       })
               })
        }.render());

        field.node.children.push({
            View::new()
                .tag("input")
                .add_class("signature-field__input")
                .set_attr("type", "hidden")
                .set_attr("name", self.name.as_str())
                .set_attr("id", &format!("signature-field-{}__input", self.id))
        }.render());
        field.node
    }
}
use std::borrow::BorrowMut;

use uuid::Uuid;

use crate::{Renderable, scale};
use crate::DefaultModifiers;
use crate::components::{Alignment, Appendable, Button, ButtonStyle, Card, CardStyle, ChildContainer, HStack, Popup, Text, TextStyle, View, VStack};
use crate::node::{Node, NodeContainer};

#[derive(Debug, Clone)]
pub struct SignatureField {
    node: Node,
    pub label: Option<String>,
    pub name: String,
    pub id: String,
    pub form_name: Option<String>,
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
            id: Uuid::new_v4().to_string(),
            form_name: None,
        }
    }

    pub fn label(&mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self.clone()
    }
    pub fn attach_to_form(&mut self, form_name: &str) -> Self {
        self.form_name = Some(form_name.to_string());
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
                .add_class("signature-field__opener")
            .popup({
                    Popup::new()
                        .add_class("signature-field__popup")
                        .hide_window_controls()
                        .append_child({
                            VStack::new(Alignment::Stretch)
                                .width("100%")
                                .height("100%")
                                .gap(vec![scale(2)])
                                .padding(vec![scale(4)])
                                .append_child({
                                    View::new()
                                        .add_class("signature-field__canvas-container")
                                        .width("100%")
                                        .height("100%")
                                        .append_child({
                                            Card::new(CardStyle::Raised)
                                                .add_class("signature-field__canvas-container__canvas")
                                                .tag("canvas")
                                                .set_attr("id", &format!("signature-field-{}__canvas", self.id))
                                        })
                                })
                                .append_child({
                                    let mut sign_button = Button::new("Signer", ButtonStyle::Filled)
                                        .add_class("signature-field__submit")
                                        .align_self("flex-end")
                                        .close_popup();
                                    if let Some(form_name) = &self.form_name {
                                        sign_button = sign_button
                                            .set_attr("form", form_name)
                                            .set_attr("type", "submit");
                                    }
                                    sign_button
                                })
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
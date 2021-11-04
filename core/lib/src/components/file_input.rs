use std::borrow::BorrowMut;

use crate::{DefaultModifiers, Overflow, scale, sp};
use crate::components::{Alignment, Appendable, Button, ButtonStyle, Card, CardStyle, ChildContainer, HStack, Icon, Image, ObjectFit, Text, TextStyle, View, VStack};
use crate::node::{Node, NodeContainer};
use crate::Renderable;

#[derive(Debug, Clone)]
pub enum FileInputType {
    /// Ce champ n'est pas affiché et est destiné à être attaché à un MenuItem ou un Button
    Hidden,
    /// Simple bouton
    Simple,
    /// Ce champ affiche l'aperçu de l'image selectionnée
    Image,
}

#[derive(Debug, Clone)]
pub struct FileInput {
    children: Vec<Box<dyn Renderable>>,
    node: Node,
    auto_submit: bool,
    input_type: FileInputType,
    name: String,
}

impl NodeContainer for FileInput {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers<FileInput> for FileInput {}

impl FileInput {
    pub fn new(name: &str, file_input_type: FileInputType) -> Self {
        FileInput {
            children: vec![],
            node: Node::default(),
            auto_submit: false,
            input_type: file_input_type,
            name: name.to_string(),
        }
    }

    pub fn accept(&mut self, mime_types: &str) -> Self {
        self.set_attr("accept", mime_types)
    }

    pub fn multiple(&mut self) -> Self {
        self.set_attr("multiple", "multiple")
    }

    pub fn auto_submit(&mut self, is_auto_submit: bool) -> Self {
        if is_auto_submit {
            self.auto_submit = is_auto_submit;
        }
        self.clone()
    }
}


impl Renderable for FileInput {
    fn render(&self) -> Node {
        match self.input_type {
            FileInputType::Hidden => {
                let mut field = self.clone()
                    .add_class("file-input")
                    .add_class("file-input--hidden")
                    .set_attr("type", "file")
                    .tag("input")
                    .set_attr("id", &format!("file-input-{}", self.name))
                    .set_attr("name", &self.name);
                if self.auto_submit {
                    field.set_attr("data-auto-submit", &self.auto_submit.to_string());
                }
                field.node
            }
            FileInputType::Simple => {
                Card::new(CardStyle::Outlined)
                    .add_class("file-input")
                    .add_class("file-input--simple")
                    .append_child({
                        let mut input = View::new()
                            .set_attr("id", &format!("file-input-{}", self.name))
                            .set_attr("name", &self.name)
                            .set_attr("type", "file")
                            .tag("input");
                        if self.auto_submit {
                            input.set_attr("data-auto-submit", &self.auto_submit.to_string());
                        }
                        input
                    })
                    .append_child({
                        HStack::new(Alignment::Center)
                            .padding(vec![scale(3)])
                            .padding_left(16)
                            .gap(vec![scale(3)])
                            .append_child({
                                Text::new("Sélectionner un fichier", TextStyle::Body)
                                    .add_class("file-input__file-name")
                                    .flex_grow(1)
                            })
                            .append_child({
                                Button::icon_only("upload", ButtonStyle::Outlined)
                                    .add_class("file-input__button")
                            })
                    })
                    .render()
            }
            FileInputType::Image => {
                Card::new(CardStyle::Outlined)
                    .overflow(Overflow::Hidden)
                    .add_class("file-input")
                    .add_class("file-input--image")
                    .append_child({
                        let mut input = View::new()
                            .set_attr("id", &format!("file-input-{}", self.name))
                            .set_attr("name", &self.name)
                            .set_attr("type", "file")
                            .tag("input");
                        if self.auto_submit {
                            input.set_attr("data-auto-submit", &self.auto_submit.to_string());
                        }
                        input
                    })
                    .append_child({
                        VStack::new(Alignment::Stretch)
                            .append_child({
                                Image::new("")
                                    .set_attr("alt", " ")
                                    .background(&format!(
                                        "var(--surface) no-repeat center/2rem url({}) ",
                                        &format!("data:image/svg+xml;base64,{}", {
                                            base64::encode({
                                                Icon::new("file")
                                                    .set_attr("width", "200")
                                                    .set_attr("height", "200")
                                                    .set_attr("viewBox", "0 0 24 24")
                                                    .render().get_html().as_bytes()
                                            })
                                        })
                                    ))
                                    .add_class("file-input__image-preview")
                            })
                            .append_child({
                                HStack::new(Alignment::Center)
                                    .flex_grow(1)
                                    .padding(vec![scale(3)])
                                    .gap(vec![scale(3)])
                                    .append_child({
                                        Text::new("Sélectionner un fichier", TextStyle::Body)
                                            .add_class("file-input__file-name")
                                            .flex_grow(1)
                                    })
                                    .append_child({
                                        Button::icon_only("upload", ButtonStyle::Outlined)
                                            .add_class("file-input__button")
                                    })
                            })
                    })
                    .render()
            }
        }
    }
}
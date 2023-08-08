use std::borrow::BorrowMut;

use crate::{DefaultModifiers, Overflow, scale, sp};
use crate::components::{Alignment, Appendable, Button, ButtonStyle, Card, CardStyle, ChildContainer, HStack, Icon, Image, ObjectFit, Text, TextStyle, View, VStack};
use crate::components::icons::Lucide;
use crate::node::{Node, NodeContainer};
use crate::Renderable;

#[derive(Debug, Clone, PartialEq)]
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
    required: bool,
    input_type: FileInputType,
    name: String,
    label: Option<String>,
    error_text: Option<String>,
    image_preview: Option<String>,
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
            required: false,
            input_type: file_input_type,
            name: name.to_string(),
            label: None,
            error_text: None,
            image_preview: None
        }
    }

    pub fn accept(&mut self, mime_types: &str) -> Self {
        self.set_attr("accept", mime_types)
    }
    pub fn required(&mut self, is_required: bool) -> Self {
        self.required = is_required;
        self.clone()
    }

    pub fn label(&mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self.clone()
    }

    pub fn error_message(&mut self, helper_text: &str) -> Self {
        self.error_text = Some(helper_text.to_string());
        self.clone()
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
    pub fn image_preview(&mut self, src: &str) -> Self {
        self.image_preview = Some(src.to_string());
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
                    .set_attr("name", &self.name);
                if !field.node.attributes.contains_key("id") {
                    field.set_attr("id", &format!("file-input-{}", self.name));
                }
                if self.required {
                    field.set_attr("required", "required");
                }
                if self.auto_submit {
                    field.set_attr("data-auto-submit", &self.auto_submit.to_string());
                }
                field.node
            }
            FileInputType::Simple => {

                let mut simple_file_input = View::new()
                    .add_class("file-input")
                    .add_class("file-input--simple");
                if let Some(label) = &self.label {
                    simple_file_input.append_child({
                        Text::new(label.as_str(), TextStyle::Label)
                            .add_class("file-input__label")
                            .set_attr("for", &format!("file-input-{}", self.name))
                            .tag("label")
                    });
                }
                if self.required {
                    simple_file_input.append_child({
                        Text::new("Requis", TextStyle::Caption)
                            .add_class("file-input__required")
                    });
                }
                simple_file_input.append_child({
                        Card::new(CardStyle::Outlined)
                            .add_class("file-input__input")
                    .append_child({
                        let mut input = View::new()
                            .set_attr("id", &format!("file-input-{}", self.name))
                            .set_attr("name", &self.name)
                            .set_attr("type", "file")
                            .tag("input");
                        if self.required {
                            input.set_attr("required", "required");

                        }
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
                                Button::icon_only(Lucide::Upload, ButtonStyle::Outlined)
                                    .add_class("file-input__button")
                            })
                    })

                    });
                if let Some(error_message) = &self.error_text {
                    simple_file_input.add_class("file-input--error");
                    simple_file_input.append_child({
                        Text::new(error_message, TextStyle::Caption)
                            .add_class("file-input__helper-text")
                    });
                }
                    simple_file_input.render()
            }
            FileInputType::Image => {
                let mut image_file_input = View::new()
                    .add_class("file-input")
                    .add_class("file-input--image");
                if let Some(label) = &self.label {
                    image_file_input.append_child({
                        Text::new(label.as_str(), TextStyle::Label)
                            .add_class("file-input__label")
                            .set_attr("for", &format!("file-input-{}", self.name))
                            .tag("label")
                    });
                }
                if self.required {
                    image_file_input.append_child({
                        Text::new("Requis", TextStyle::Caption)
                            .add_class("file-input__required")
                    });
                }
                image_file_input.append_child({
                    Card::new(CardStyle::Outlined)
                        .add_class("file-input__input")
                        .overflow(Overflow::Hidden)
                        .append_child({
                            let mut input = View::new()
                                .set_attr("id", &format!("file-input-{}", self.name))
                                .set_attr("name", &self.name)
                                .set_attr("type", "file")
                                .tag("input");
                            if self.required {
                                input.set_attr("required", "required");
                            }
                            if self.auto_submit {
                                input.set_attr("data-auto-submit", &self.auto_submit.to_string());
                            }
                            input
                        })
                        .append_child({
                            VStack::new(Alignment::Stretch)
                                .append_child({
                                    Image::new(&self.clone().image_preview.unwrap_or_default())
                                        .set_attr("alt", " ")
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
                                            Button::icon_only(Lucide::Upload, ButtonStyle::Outlined)
                                                .add_class("file-input__button")
                                        })
                                })
                        })
                });

                if let Some(error_message) = &self.error_text {
                    image_file_input.add_class("file-input--error");
                    image_file_input.append_child({
                        Text::new(error_message, TextStyle::Caption)
                            .add_class("file-input__helper-text")
                    });
                }
                image_file_input.render()
            }
        }
    }
}
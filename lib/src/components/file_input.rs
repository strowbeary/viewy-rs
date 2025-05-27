use std::borrow::BorrowMut;

use crate::Renderable;
use crate::components::icons::Lucide;
use crate::components::{
    Alignment, Appendable, Button, ButtonStyle, Card, CardStyle, HStack, Image, Text, TextStyle,
    VStack, View,
};
use crate::node::{Node, NodeContainer};
use crate::{DefaultModifiers, Overflow, scale};

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
    accept: Option<String>,
    multiple: bool,
}

impl NodeContainer for FileInput {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers for FileInput {}

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
            image_preview: None,
            accept: None,
            multiple: false,
        }
    }

    pub fn accept(&mut self, mime_types: &str) -> &mut Self {
        self.accept = Some(mime_types.to_string());
        self
    }
    pub fn required(&mut self, is_required: bool) -> &mut Self {
        self.required = is_required;
        self
    }

    pub fn label(&mut self, label: &str) -> &mut Self {
        self.label = Some(label.to_string());
        self
    }

    pub fn error_message(&mut self, helper_text: &str) -> &mut Self {
        self.error_text = Some(helper_text.to_string());
        self
    }

    pub fn multiple(&mut self) -> &mut Self {
        self.multiple = true;
        self
    }

    pub fn auto_submit(&mut self, is_auto_submit: bool) -> &mut Self {
        self.auto_submit = is_auto_submit;

        self
    }
    pub fn image_preview(&mut self, src: &str) -> &mut Self {
        self.image_preview = Some(src.to_string());
        self
    }
}

impl Renderable for FileInput {
    fn render(mut self) -> Node {
        match self.input_type {
            FileInputType::Hidden => {
                self.add_class("file-input")
                    .add_class("file-input--hidden")
                    .set_attr("type", "file")
                    .tag("input")
                    .set_attr("name", &self.name);
                if !self.node.attributes.contains_key("id") {
                    self.set_attr("id", &format!("file-input-{}", self.name));
                }
                if self.required {
                    self.set_attr("required", "required");
                }
                if let Some(accept) = &self.accept {
                    self.set_attr("accept", accept);
                }
                if self.auto_submit {
                    self.set_attr("data-auto-submit", &self.auto_submit.to_string());
                }
                if self.multiple {
                    self.set_attr("multiple", "multiple");
                }
                self.node
            }
            FileInputType::Simple => {
                let mut simple_file_input = View::new();
                simple_file_input
                    .add_class("file-input")
                    .add_class("file-input--simple");
                if let Some(label) = &self.label {
                    simple_file_input.append_child({
                        let mut text = Text::new(label.as_str(), TextStyle::Label);
                        text.add_class("file-input__label")
                            .set_attr("for", &format!("file-input-{}", self.name))
                            .tag("label");
                        text
                    });
                }
                if self.required {
                    simple_file_input.append_child({
                        let mut text = Text::new("Requis", TextStyle::Caption);
                        text.add_class("file-input__required");
                        text
                    });
                }
                simple_file_input.append_child({
                    Card::new(CardStyle::Outlined)
                        .add_class("file-input__input")
                        .append_child({
                            let mut input = View::new();
                            input
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

                            if let Some(accept) = &self.accept {
                                input.set_attr("accept", accept);
                            }

                            if self.multiple {
                                input.set_attr("multiple", "multiple");
                            }
                            input
                        })
                        .append_child({
                            HStack::new(Alignment::Center)
                                .padding(vec![scale(3)])
                                .padding_left(16)
                                .gap(vec![scale(3)])
                                .append_child({
                                    let mut text =
                                        Text::new("Sélectionner un fichier", TextStyle::Body);
                                    text.add_class("file-input__file-name").flex_grow(1);
                                    text
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
                        let mut text = Text::new(error_message, TextStyle::Caption);
                        text.add_class("file-input__helper-text");
                        text
                    });
                }
                simple_file_input.render()
            }
            FileInputType::Image => {
                let mut image_file_input = View::new();
                image_file_input.add_class("file-input")
                    .add_class("file-input--image");
                if let Some(label) = &self.label {
                    image_file_input.append_child({
                        let mut text = Text::new(label.as_str(), TextStyle::Label);
                        text.add_class("file-input__label")
                            .set_attr("for", &format!("file-input-{}", self.name))
                            .tag("label");
                        text
                    });
                }
                if self.required {
                    image_file_input.append_child({
                        let mut text = Text::new("Requis", TextStyle::Caption);
                        text.add_class("file-input__required");
                        text
                    });
                }
                image_file_input.append_child({
                    Card::new(CardStyle::Outlined)
                        .add_class("file-input__input")
                        .overflow(Overflow::Hidden)
                        .append_child({
                            let mut input = View::new();
                            input
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
                            if let Some(accept) = &self.accept {
                                input.set_attr("accept", accept);
                            }
                            if self.multiple {
                                input.set_attr("multiple", "multiple");
                            }
                            input
                        })
                        .append_child({
                            VStack::new(Alignment::Stretch)
                                .append_child({
                                    let mut img =
                                        Image::new(&self.clone().image_preview.unwrap_or_default());
                                    img.set_attr("alt", "preview of the selected file")
                                        .add_class("file-input__image-preview");
                                    img
                                })
                                .append_child({
                                    HStack::new(Alignment::Center)
                                        .flex_grow(1)
                                        .padding(vec![scale(3)])
                                        .gap(vec![scale(3)])
                                        .append_child({
                                            let mut text = Text::new(
                                                "Sélectionner un fichier",
                                                TextStyle::Body,
                                            );
                                            text.add_class("file-input__file-name").flex_grow(1);
                                            text
                                        })
                                        .append_child({
                                            Button::icon_only(Lucide::Upload, ButtonStyle::Outlined)
                                                .add_class("file-input__button")
                                        })
                                })
                        })
                });

                if let Some(error_message) = &self.error_text {
                    image_file_input
                        .add_class("file-input--error")
                        .append_child({
                            let mut text = Text::new(error_message, TextStyle::Caption);
                            text.add_class("file-input__helper-text");
                            text
                        });
                }
                image_file_input.render()
            }
        }
    }
}

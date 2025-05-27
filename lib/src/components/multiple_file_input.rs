use std::borrow::BorrowMut;
use uuid::Uuid;

use crate::components::icons::Lucide;
use crate::components::{Alignment, Appendable, Button, ButtonStyle, FileInputType, HStack, Icon, Text, TextStyle, VStack, View, ProgressBar, progress_bar};
use crate::node::{Node, NodeContainer};
use crate::Renderable;
use crate::{scale, DefaultModifiers};

#[derive(Debug, Clone)]
pub struct MultipleFileInput {
    node: Node,
    auto_submit: bool,
    required: bool,
    input_type: FileInputType,
    name: String,
    label: Option<String>,
    error_text: Option<String>,
    accept: Option<String>,
    redirect_uri: Option<String>,
}

impl NodeContainer for MultipleFileInput {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers for MultipleFileInput {}

impl MultipleFileInput {
    pub fn new(name: &str, file_input_type: FileInputType) -> Self {
        MultipleFileInput {
            node: Node::default(),
            auto_submit: false,
            required: false,
            input_type: file_input_type,
            name: name.to_string(),
            label: None,
            error_text: None,
            accept: None,
            redirect_uri: None
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

    pub fn auto_submit(&mut self, is_auto_submit: bool) -> &mut Self {
        self.auto_submit = is_auto_submit;
        self
    }

    pub fn redirect_to_after_upload(&mut self, uri: &str) -> &mut Self {
        self.redirect_uri = Some(uri.to_string());
        self
    }
}

impl Renderable for MultipleFileInput {
    fn component_name(&self) -> &str {
        "MultipleFileInput"
    }
    fn render(mut self) -> Node {
        let instance_id = Uuid::new_v4();
        let mut file_list = VStack::new(Alignment::Stretch);
        file_list .add_class("multiple-file-input__file-list")
            .id(&instance_id.to_string())
            .append_child({
                let mut control_bar = HStack::new(Alignment::Center)
                    .add_class("multiple-file-input__file-list__control-bar")
                    .append_child({
                        Button::icon_only(Lucide::X, ButtonStyle::Link)
                            .add_class("multiple-file-input--hidden__file-list__control-bar__close-button")
                    });
                if self.input_type == FileInputType::Hidden {
                    control_bar
                        .add_class("multiple-file-input--hidden__file-list__control-bar");
                }
                control_bar
            })
            .append_child({
                VStack::new(Alignment::Stretch)
                    .add_class("multiple-file-input__file-list__content")
                    .append_child({
                        VStack::new(Alignment::Center)
                            .flex_grow(1)
                            .justify_content("center")
                            .gap(vec![scale(2)])
                            .append_child({
                                Icon::new(Lucide::CloudUpload)
                                    .size(64)
                                    .stroke_width(1)
                                    .color("var(--color-accent)")
                            })
                            .append_child({
                                Text::new("Téléverser vos fichiers", TextStyle::H1)
                            })
                    })
                    .append_child({
                        View::new()
                            .tag("template")
                            .append_child({
                                HStack::new(Alignment::Center)
                                    .gap(vec![scale(3)])
                                    .add_class("multiple-file-input__file-list__item")
                                    .append_child({
                                        Icon::new(Lucide::File)
                                            .stroke_width(1)
                                            .size(37)
                                    })
                                    .append_child({
                                        let mut stack = VStack::new(Alignment::Stretch);
                                        stack.flex_grow(1)
                                            .gap(vec![scale(2)])
                                            .append_child({
                                                let mut text = Text::new("Filename", TextStyle::Headline);
                                                text.add_class("t-file-name");
                                                text
                                            })
                                            .append_child({
                                                let mut progress_bar = ProgressBar::new();
                                                progress_bar.add_class("t-progress")
                                                    .flex_grow(1);
                                                progress_bar
                                            });
                                        stack
                                    })
                            })
                    })
            })
          ;

        match self.input_type {
            FileInputType::Hidden => {
                // Affiche une petite fenêtre pour suivre l'envoie des fichiers.
                self
                    .set_attr("data-file-list", &instance_id.to_string())
                    .add_class("multiple-file-input")
                    .add_class("multiple-file-input--hidden")
                    .add_class("multiple-file-input__input")
                    .set_attr("type", "file")
                    .set_attr("multiple", "multiple")
                    .tag("input")
                    .set_attr("name", &self.name);
                if let Some(redirect_uri) = &self.redirect_uri {
                    self.set_attr("data-redirect-uri", redirect_uri);
                }
                file_list.add_class("multiple-file-input--hidden__file-list");
                self.node.root_nodes.insert(Box::new(file_list));
                if !self.node.attributes.contains_key("id") {
                    self.set_attr("id", &format!("file-input-{}", self.name));
                }
                if self.required {
                    self.set_attr("required", "required");
                }
                if self.auto_submit {
                    self.set_attr("data-auto-submit", &self.auto_submit.to_string());
                }
                self.node
            }
            FileInputType::Simple | FileInputType::Image => {
                // Affiche un bloc complet avec le suivi de l'envoi des fichiers
                let mut container = self
                    .clone();
                container.set_attr("data-file-list", &instance_id.to_string())
                    .add_class("multiple-file-input")
                    .add_class("multiple-file-input--simple");

                container.node.children.push({
                    let mut field = self.clone();
                    field.add_class("multiple-file-input__input")
                        .set_attr("type", "file")
                        .set_attr("multiple", "multiple")
                        .tag("input")
                        .set_attr("name", &self.name);
                    if let Some(redirect_uri) = &self.redirect_uri {
                        field.set_attr("data-redirect-uri", redirect_uri);
                    }
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
                });
                container.node.children.push(file_list.add_class("multiple-file-input--simple__file-list").render());
                container.node.children.push({
                    let mut button = Button::new("Selectionner des fichiers", ButtonStyle::Outlined);
                    button.icon(Lucide::Files)
                        .add_class("multiple-file-input__button");
                    button.render()
                });

                container.node
            }
        }
    }
}

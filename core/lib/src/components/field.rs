use std::borrow::BorrowMut;

use chrono::{DateTime, Duration, NaiveDateTime, TimeZone, Utc};
use html_escape::encode_text;
use uuid::Uuid;

use crate::{DefaultModifiers, Overflow, scale, sp};
use crate::components::*;
use crate::components::FieldType::{DateTimeLocal, RichTextArea};
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
    /// Each duration will be bound to a Tag component. On click, the duration is added to the start date and injected in end_date field.
    Duration(Vec<Duration>),
    Month,
    Search,
    Time,
    Url,
    Week,
    Hidden,
    TextArea,
    RichTextArea,
}


#[derive(Debug, Clone)]
pub struct Field {
    node: Node,
    pub label: Option<String>,
    pub value: Option<String>,
    pub min: Option<String>,
    pub max: Option<String>,
    pub step: Option<String>,
    pub helper_text: Option<String>,
    pub placeholder: Option<String>,
    pub leading_icon: Option<String>,
    pub trailing_icon: Option<String>,
    pub field_type: FieldType,
    pub name: String,
    pub auto_sizing: bool,
    pub datalist: bool,
    pub required: bool,
    pub read_only: bool,
}

impl NodeContainer for Field {
    fn get_node(&mut self) -> &mut Node {
        self.node.borrow_mut()
    }
}

impl DefaultModifiers<Field> for Field {}

impl Field {
    pub fn new(name: &str, field_type: FieldType) -> Self {
        Field {
            node: Default::default(),
            label: None,
            value: None,
            min: None,
            max: None,
            step: None,
            helper_text: None,
            placeholder: None,
            leading_icon: None,
            trailing_icon: None,
            field_type,
            name: name.to_string(),
            auto_sizing: false,
            datalist: false,
            required: false,
            read_only: false,
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

    pub fn min(&mut self, value: &str) -> Self {
        self.min = Some(value.to_string());
        self.clone()
    }

    pub fn max(&mut self, value: &str) -> Self {
        self.max = Some(value.to_string());
        self.clone()
    }

    pub fn step(&mut self, value: &str) -> Self {
        self.step = Some(value.to_string());
        self.clone()
    }

    pub fn read_only(&mut self, is_read_only: bool) -> Self {
        self.read_only = is_read_only;
        self.clone()
    }

    pub fn helper_text(&mut self, helper_text: &str) -> Self {
        self.helper_text = Some(helper_text.to_string());
        self.clone()
    }

    pub fn error_message(&mut self, helper_text: &str) -> Self {
        self.add_class("field--error");
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

impl Renderable for Field {
    fn render(&self) -> Node {
        let mut field = self.clone()
            .add_class("field");
        match &self.field_type {
            FieldType::RichTextArea => {
                field.add_class("field__rich-text-area");
                let mut input = View::new()
                    .tag("input")
                    .add_class("field__input")
                    .set_attr("type", "text")
                    .display("none")
                    .set_attr("id", self.name.as_str())
                    .set_attr("name", self.name.as_str())
                    .set_attr("value", &encode_text(&field.value.unwrap_or_default()));

                if self.required {
                    input.set_attr("required", "required");
                    field.node.children.push({
                        Text::new("Requis", TextStyle::Caption)
                            .color("var(--color-text-secondary)")
                            .grid_area("required")
                            .render()
                    });
                }
                if self.read_only {
                    input.set_attr("readonly", "readonly");
                }


                let id = Uuid::new_v4().to_string();
                let editor_id = &format!("editor-{}", id);
                let toolbar_id = &format!("toolbar-{}", id);
                field.node.children.push({
                    Card::new(CardStyle::Outlined)
                        .grid_area("input")
                        .overflow(Overflow::Hidden)
                        .append_child({
                            HStack::new(Alignment::Center)
                                .padding(vec![scale(3)])
                                .set_attr("id", &toolbar_id)
                                .gap(vec![scale(4)])
                                .background("var(--surface)")
                                .add_class("field__toolbar")
                                .append_child({
                                    HStack::new(Alignment::Center)
                                        .gap(vec![scale(2)])
                                        .append_child({
                                            Button::new("Titre 1", ButtonStyle::Flat)
                                                .padding(vec![8])
                                                .height(&sp(32))
                                                .add_class("ql-header")
                                                .set_attr("value", "1")
                                        })
                                        .append_child({
                                            Button::new("Titre 2", ButtonStyle::Flat)
                                                .padding(vec![8])
                                                .height(&sp(32))
                                                .add_class("ql-header")
                                                .set_attr("value", "2")
                                        })
                                        .append_child({
                                            Button::new("Titre 3", ButtonStyle::Flat)
                                                .padding(vec![8])
                                                .height(&sp(32))
                                                .add_class("ql-header")
                                                .set_attr("value", "3")
                                        })
                                        .append_child({
                                            Button::new("Corps de texte", ButtonStyle::Flat)
                                                .padding(vec![8])
                                                .height(&sp(32))
                                                .add_class("ql-header")
                                                .set_attr("value", "")
                                        })
                                })
                                .append_child({
                                    HStack::new(Alignment::Center)
                                        .gap(vec![scale(2)])
                                        .append_child({
                                            Button::icon_only("bold", ButtonStyle::Flat)
                                                .add_class("ql-bold")
                                        })
                                        .append_child({
                                            Button::icon_only("italic", ButtonStyle::Flat)
                                                .add_class("ql-italic")
                                        })
                                        .append_child({
                                            Button::icon_only("underline", ButtonStyle::Flat)
                                                .add_class("ql-underline")
                                        })
                                })
                                .append_child({
                                    Button::icon_only("reset-format", ButtonStyle::Flat)
                                        .add_class("ql-clean")
                                })
                                .append_child({
                                    HStack::new(Alignment::Center)
                                        .gap(vec![scale(2)])
                                        .append_child({
                                            Button::icon_only("list", ButtonStyle::Flat)
                                                .add_class("ql-list")
                                                .set_attr("value", "bullet")
                                        })
                                        .append_child({
                                            Button::icon_only("link-2", ButtonStyle::Flat)
                                                .add_class("ql-link")
                                        })
                                })
                        })
                        .append_child(input)
                        .append_child({
                            let mut editor = View::new()
                                .add_class("field__editor")
                                .padding(vec![scale(3)])
                                .set_attr("id", editor_id);
                            editor.node.text = self.value.clone();
                            editor
                        })
                }.render());

                if let Some(label) = field.label {
                    let text = Text::new(label.as_str(), TextStyle::Label)
                        .add_class("field__label")
                        .set_attr("for", self.name.as_str())
                        .tag("label");
                    field.node.children.push(text.render());
                }

                if let Some(helper_text) = field.helper_text {
                    let text = ComplexText::new(helper_text.as_str(), TextStyle::Caption)
                        .add_class("field__helper-text");
                    field.node.children.push(text.render());
                }

                field.node.children.push({
                    let mut script = View::new()
                        .tag("script")
                        .render();
                    script.text = Some(format!("new Quill('#{}', {{ modules: {{ toolbar: '#{}' }} }})", editor_id, toolbar_id));
                    script
                });


                field.node
            }
            FieldType::Duration(values) => {
                let default_start_date = field.value.clone()
                    .and_then(|value| {
                        NaiveDateTime::parse_from_str(&value, "%Y-%m-%dT%R")
                            .map(|naive_datetime| Utc.from_utc_datetime(&naive_datetime))
                            .ok()
                    })
                    .unwrap_or(Utc::now());
                field.add_class("field--duration");

                field.node.children.push({
                    VStack::new(Alignment::Stretch)
                        .gap(vec![scale(3)])
                        .append_child({
                            Field::new(&format!("{}_start_datetime", self.name), FieldType::DateTimeLocal)
                                .required(self.required)
                                .label("Date de début")
                                .value(&default_start_date.format("%FT%R").to_string())
                                .set_attr("id", &format!("{}_start_datetime", self.name))
                                .set_attr("name", &format!("{}_start_datetime", self.name))
                        })
                        .append_child({
                            let mut tags = HStack::new(Alignment::Center).gap(vec![scale(2)]);
                            for duration in values {
                                tags.append_child({
                                    Tag::new(&{
                                        if duration.num_minutes() < 60 {
                                            format!("+{} min", duration.num_minutes())
                                        } else {
                                            if duration.num_minutes() % 60 != 0 {
                                                format!("+{}h{:02}", duration.num_minutes() / 60, duration.num_minutes() % 60)
                                            } else {
                                                format!("+{}h", duration.num_minutes() / 60)
                                            }
                                        }
                                    })
                                        .add_class("clickable")
                                        .set_attr("data-duration", &duration.num_minutes().to_string())
                                });
                            }
                            tags
                        })
                        .append_child({
                            Field::new(&format!("{}_end_datetime", self.name), FieldType::DateTimeLocal)
                                .required(self.required)
                                .label("Date de fin")
                                .set_attr("id", &format!("{}_end_datetime", self.name))
                                .set_attr("name", &format!("{}_end_datetime", self.name))
                        })
                }.render());

                field.node
            }
            _ => {
                let mut input = View::new()
                    .tag(&match &self.field_type {
                        FieldType::TextArea => "textarea",
                        _ => "input",
                    })
                    .add_class("field__input")

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
                if self.read_only {
                    input.set_attr("readonly", "readonly");
                }
                if let Some(placeholder) = &self.placeholder {
                    input.set_attr("placeholder", placeholder);
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
                    let id = Uuid::new_v4().to_string();
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
                if let Some(value) = field.min {
                    input.set_attr("min", &value);
                }
                if let Some(value) = field.max {
                    input.set_attr("max", &value);
                }
                if let Some(value) = field.step {
                    input.set_attr("step", &value);
                }

                if matches!(field.field_type, FieldType::Hidden) {
                    input.render()
                } else {
                    field.node.children.push(input.render());

                    if let Some(placeholder) = field.placeholder {
                        input.set_attr("placeholder", placeholder.as_str());
                    }

                    if let Some(label) = field.label {
                        let text = Text::new(label.as_str(), TextStyle::Label)
                            .add_class("field__label")
                            .set_attr("for", self.name.as_str())
                            .tag("label");
                        field.node.children.push(text.render());
                    }

                    if let Some(helper_text) = field.helper_text {
                        let text = ComplexText::new(helper_text.as_str(), TextStyle::Caption)
                            .add_class("field__helper-text");
                        field.node.children.push(text.render());
                    }

                    field.node
                }
            }
        }
    }
}
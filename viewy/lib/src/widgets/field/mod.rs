use crate::core::widget::Widget;
use crate::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq)]
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
    MainSearchBar,
    Time,
    Url,
    Week,
    Hidden,
    TextArea,
    RichTextArea,
}

/// A control that performs an action when triggered.
/// ```rust
/// use viewy::prelude::*;
/// Button::new("Label", ButtonStyle::Filled)
///     .action("/") // Here create a link to "/"
/// ```
#[derive(Widget)]
#[widget(style = "./style.scss")]
pub struct Field {
    node: Node,
    field_type: FieldType,
}

impl Field {
    pub fn render(&mut self) {}
}

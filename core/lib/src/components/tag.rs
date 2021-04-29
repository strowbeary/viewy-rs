use crate::components::{Text, TextStyle};
use crate::DefaultModifiers;

pub struct Tag;

impl Tag {
    pub fn new(label: &str) -> Text {
        Text::new(label, TextStyle::Overline)
            .add_class("tag")
    }
}
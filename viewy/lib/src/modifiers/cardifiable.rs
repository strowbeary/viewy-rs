use std::fmt::Display;
use crate::core::widget::Widget;
use crate::prelude::Classable;

pub enum CardStyle {
    Outlined,
    Filled,
    OutlinedRaised,
    FilledRaised,
}

impl Display for CardStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match self {
            CardStyle::Outlined => "outlined",
            CardStyle::Filled => "filled",
            CardStyle::OutlinedRaised => "outlined-raised",
            CardStyle::FilledRaised => "filled-raised"
        })
    }
}

pub trait Cardifiable: Widget + Classable {
    fn as_card(&mut self, style: CardStyle) -> &mut Self {
        self.add_class("card");
        self.add_class(&format!("card--{style}"));
        self
    }
}

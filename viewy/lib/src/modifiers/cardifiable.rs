use crate::core::widget::Widget;
use crate::prelude::Classable;
use std::fmt::Display;

pub enum CardStyle {
    /// Outlined card style.
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// let style = CardStyle::Outlined;
    /// ```
    Outlined,
    /// Filled card style.
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// let style = CardStyle::Filled;
    /// ```
    Filled,
    /// Outlined style with elevation.
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// let style = CardStyle::OutlinedRaised;
    /// ```
    OutlinedRaised,
    /// Filled style with elevation.
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// let style = CardStyle::FilledRaised;
    /// ```
    FilledRaised,
}

impl Display for CardStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CardStyle::Outlined => "outlined",
                CardStyle::Filled => "filled",
                CardStyle::OutlinedRaised => "outlined-raised",
                CardStyle::FilledRaised => "filled-raised",
            }
        )
    }
}

/// Adds card presentation classes to widgets.
///
/// This helper adds:
/// - `card`
/// - `card--<style>`
///
/// Intended for Viewy widget authors.
pub trait Cardifiable: Widget + Classable {
    /// Transform the widget into a card with the given style variant.
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// let mut panel = View::new();
    /// panel.as_card(CardStyle::OutlinedRaised);
    /// ```
    fn as_card(&mut self, style: CardStyle) -> &mut Self {
        self.add_class("card");
        self.add_class(&format!("card--{style}"));
        self
    }
}

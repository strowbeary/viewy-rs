mod view;
mod titlebar;
mod text;
mod grid;
mod card;
mod button;
mod vstack;
mod hstack;
mod textfield;
mod picker;
mod icon;
mod image;
mod complex_text;
mod popover;
mod form;
mod divider;
mod menu;
mod table;
mod checkbox;

pub use button::{Button, ButtonStyle};
pub use card::{Card, CardStyle};
pub use vstack::{VStack, Alignment};
pub use complex_text::ComplexText;
pub use titlebar::TitleBar;
pub use grid::Grid;
pub use view::View;
pub use textfield::{TextField, FieldType};
pub use picker::{Picker, PickerStyle, PickerOption};
pub use icon::Icon;
pub use image::Image;
pub use popover::Popover;
pub use hstack::HStack;
pub use text::{Text, TextStyle};
pub use form::Form;
pub use divider::Divider;
pub use menu::*;
pub use table::*;
pub use checkbox::*;

use crate::node::Node;
use crate::renderer::Renderable;

/// Trait that make the children property accessible for Appendable trait
pub trait ChildContainer {
    fn get_children(&mut self) -> &mut Vec<Box<dyn Renderable>>;
}

/// Trait that make a component capable of receiving children
pub trait Appendable: ChildContainer + Clone {

    /// Adds a node to the end of the list of children of a specified parent node.
    /// ```rust
    /// View::new()
    ///     .append_child({
    ///         View::new()
    ///     })
    /// ```
    fn append_child<C>(&mut self, child: C) -> Self
        where
            C: 'static + Renderable,
    {
        self.get_children().push(Box::new(child));
        self.clone()
    }

    /// Adds a node before the first child of the list of children of a specified parent node.
    /// ```rust
    /// View::new()
    ///     .prepend_child({
    ///         View::new()
    ///     })
    /// ```
    fn prepend_child<C>(&mut self, child: C) -> Self
        where
            C: 'static + Renderable,
    {
        self.get_children().insert(0, Box::new(child));
        self.clone()
    }
}
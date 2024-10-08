mod view;
mod titlebar;
mod text;
mod grid;
mod card;
mod button;
mod vstack;
mod hstack;
mod field;
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
mod avatar;
mod tag;
mod popup;
mod file_input;
mod color_picker;
mod snackbar;
mod tabs;
mod badge;
mod dynamic_content;
mod sortable_stack;
mod gauge;
mod stepper;
mod disclosure;
mod table_of_content;
mod multiple_file_input;
mod progress_bar;

pub use badge::{BadgeModifiers, Badge, BadgeType};
pub use button::{Button, ButtonStyle};
pub use disclosure::{Disclosure};
pub use card::{Card, CardStyle};
pub use vstack::{VStack, Alignment};
pub use complex_text::ComplexText;
pub use titlebar::TitleBar;
pub use grid::Grid;
pub use view::View;
pub use field::{Field, FieldType};
pub use picker::{Picker, PickerStyle, PickerOption};
pub use icon::{Icon, icons};
pub use image::{Image, ObjectFit};
pub use popover::{Popover, Placement};
pub use hstack::HStack;
pub use text::{Text, TextStyle};
pub use form::Form;
pub use divider::Divider;
pub use menu::*;
pub use table::*;
pub use checkbox::*;
pub use avatar::*;
pub use tag::*;
pub use popup::*;
pub use file_input::*;
pub use multiple_file_input::*;
pub use color_picker::*;
pub use gauge::{Gauge, GaugeStyle};
pub use progress_bar::ProgressBar;
pub use snackbar::{Snackbar, SnackbarType};
pub use dynamic_content::{DynamicContent};
pub use stepper::{};
pub use tabs::{
    TabView,
    TabViewItem
};
pub use sortable_stack::SortableStack;
pub use table_of_content::{TableOfContents, TableOfContentsItem, TableOfContentItemType};

use crate::node::Node;
use crate::Renderable;

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
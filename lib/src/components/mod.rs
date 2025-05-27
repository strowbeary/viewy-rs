mod avatar;
mod badge;
mod button;
mod card;
mod checkbox;
mod color_picker;
mod complex_text;
mod disclosure;
mod divider;
mod dynamic_content;
mod field;
mod file_input;
mod form;
mod gauge;
mod grid;
mod hstack;
mod icon;
mod image;
mod menu;
mod multiple_file_input;
mod picker;
mod popover;
mod popup;
mod progress_bar;
mod snackbar;
mod sortable_stack;
mod stepper;
mod table;
mod table_of_content;
mod tabs;
mod tag;
mod text;
mod titlebar;
mod view;
mod vstack;

pub use avatar::*;
pub use badge::{Badge, BadgeModifiers, BadgeType};
pub use button::{Button, ButtonStyle};
pub use card::{Card, CardStyle};
pub use checkbox::*;
pub use color_picker::*;
pub use complex_text::ComplexText;
pub use disclosure::Disclosure;
pub use divider::Divider;
pub use dynamic_content::DynamicContent;
pub use field::{Field, FieldType};
pub use file_input::*;
pub use form::{Form, FormMethod};
pub use gauge::{Gauge, GaugeStyle};
pub use grid::Grid;
pub use hstack::HStack;
pub use icon::{Icon, icons};
pub use image::{Image, ObjectFit};
pub use menu::*;
pub use multiple_file_input::*;
pub use picker::{Picker, PickerOption, PickerStyle};
pub use popover::{Placement, Popover};
pub use popup::*;
pub use progress_bar::ProgressBar;
pub use snackbar::{Snackbar, SnackbarType};
pub use sortable_stack::SortableStack;
pub use table::*;
pub use table_of_content::{TableOfContentItemType, TableOfContents, TableOfContentsItem};
pub use tabs::{TabView, TabViewItem};
pub use tag::*;
pub use text::{Text, TextStyle, SanitizationLevel};
pub use titlebar::TitleBar;
pub use view::View;
pub use vstack::{Alignment, VStack};

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
    fn append_child<C>(&mut self, child: C) -> &mut Self
    where
        C: 'static + Renderable,
    {
        self.get_children().push(Box::new(child));
        self
    }

    /// Adds a node before the first child of the list of children of a specified parent node.
    /// ```rust
    /// View::new()
    ///     .prepend_child({
    ///         View::new()
    ///     })
    /// ```
    fn prepend_child<C>(&mut self, child: C) -> &mut Self
    where
        C: 'static + Renderable,
    {
        self.get_children().insert(0, Box::new(child));
        self
    }
}

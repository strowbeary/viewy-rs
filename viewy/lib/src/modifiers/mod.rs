//! Shared widget modifiers.
//!
//! This module groups composable traits used to enrich widgets with:
//! - children management
//! - class and attribute manipulation
//! - common color helpers
//! - interaction and layout/style helpers (re-exported submodules)

use crate::core::node::Node;
use crate::core::widget::Widget;

mod actionnable;
mod box_stylable;
mod cardifiable;
mod sizable;

use crate::core::theme::Color;
#[doc(inline)]
pub use actionnable::*;
#[doc(inline)]
pub use box_stylable::*;
#[doc(inline)]
pub use cardifiable::*;

#[doc(inline)]
pub use sizable::*;

/// Adds child node manipulation helpers to widgets.
///
/// This is the main composition API used by container-like widgets.
/// Intended for Viewy widget contributors.
pub trait Appendable: Widget {
    /// Append one child to the widget.
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// View::new().append_child(View::new());
    /// ```
    ///
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// View::new().append_child(View::new().add_class("my-class"));
    /// ```
    ///
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// View::new().append_child({
    ///     let mut view = View::new();
    ///     view.add_class("my-view");
    ///     if true {
    ///         view.add_class("my-conditional-class");
    ///     }
    ///     view
    /// });
    /// ```
    fn append_child<C>(&mut self, child: C) -> &mut Self
    where
        C: Into<Node>,
    {
        let node: &mut Node = self.deref_mut();
        let child_node = child.into();

        node.children.push(child_node);

        self
    }

    /// Replace all existing children with a new list.
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// let mut root = View::new();
    /// root.set_children(vec![Text::new("Hello", TextStyle::Body).into()]);
    /// ```
    fn set_children(&mut self, children: Vec<Node>) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.children = children;
        self
    }
}

/// Adds CSS class list manipulation helpers.
///
/// Intended for Viewy widget authors.
pub trait Classable: Widget {
    /// Insert a class in the widget class list.
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// let mut button = Button::new("Save", ButtonStyle::Filled);
    /// button.add_class("is-loading");
    /// ```
    fn add_class(&mut self, class: &str) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.class_list.insert(class.to_string());
        self
    }
    /// Remove a class from the widget class list.
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// let mut button = Button::new("Save", ButtonStyle::Filled);
    /// button.add_class("is-loading").remove_class("is-loading");
    /// ```
    fn remove_class(&mut self, class: &str) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.class_list.remove(class);
        self
    }
}

/// Adds HTML attribute manipulation helpers.
///
/// Intended for Viewy widget authors.
pub trait Attributable: Widget {
    /// Set an HTML attribute on the widget root node.
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// let mut button = Button::new("Open", ButtonStyle::Filled);
    /// button.set_attr("aria-label", "Open dialog");
    /// ```
    fn set_attr(&mut self, name: &str, value: &str) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.attributes.insert(name.to_string(), value.to_string());
        self
    }

    /// Remove an HTML attribute from the widget root node.
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// let mut button = Button::new("Open", ButtonStyle::Filled);
    /// button.set_attr("disabled", "true").unset_attr("disabled");
    /// ```
    fn unset_attr(&mut self, name: &str) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.attributes.remove(name);
        self
    }
}

/// Adds common color-related style helpers.
///
/// Values are mapped to CSS variables produced by the Viewy theme system.
/// Intended for Viewy widget authors.
pub trait Colorable: Widget {
    /// Set the `color` CSS property using a theme token.
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// let mut text = Text::new("Status", TextStyle::Body);
    /// text.color(Color::Primary);
    /// ```
    fn color(&mut self, color: Color) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.node_style
            .insert("color".to_string(), format!("var({})", color.as_str()));
        self
    }

    /// Set the `background-color` CSS property using a theme token.
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// let mut view = View::new();
    /// view.background_color(Color::Surface);
    /// ```
    fn background_color(&mut self, color: Color) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.node_style.insert(
            "background-color".to_string(),
            format!("var({})", color.as_str()),
        );
        self
    }

    /// Set the `border-color` CSS property using a theme token.
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// let mut view = View::new();
    /// view.border_color(Color::Border);
    /// ```
    fn border_color(&mut self, color: Color) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.node_style.insert(
            "border-color".to_string(),
            format!("var({})", color.as_str()),
        );
        self
    }
}

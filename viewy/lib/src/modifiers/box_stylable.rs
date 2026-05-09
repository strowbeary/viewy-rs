use crate::core::widget::Widget;
use crate::helper_fn::sp;
use crate::node::Node;

pub trait Marginable: Widget {
    /// Set the CSS `margin` shorthand.
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// let mut view = View::new();
    /// view.margin(vec![8, 12]);
    /// ```
    fn margin(&mut self, margin: Vec<i32>) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        let params: Vec<String> = margin.iter().map(|size| sp(size.clone())).collect();
        node.node_style
            .insert("margin".to_string(), params.join(" "));
        self
    }
    /// Set the CSS `margin-top`.
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// View::new().margin_top(8);
    /// ```
    fn margin_top(&mut self, value: i32) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.node_style.insert("margin-top".to_string(), sp(value));
        self
    }
    /// Set the CSS `margin-bottom`.
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// View::new().margin_bottom(8);
    /// ```
    fn margin_bottom(&mut self, value: i32) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.node_style
            .insert("margin-bottom".to_string(), sp(value));
        self
    }
    /// Set the CSS `margin-left`.
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// View::new().margin_left(8);
    /// ```
    fn margin_left(&mut self, value: i32) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.node_style.insert("margin-left".to_string(), sp(value));
        self
    }
    /// Set the CSS `margin-right`.
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// View::new().margin_right(8);
    /// ```
    fn margin_right(&mut self, value: i32) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.node_style
            .insert("margin-right".to_string(), sp(value));
        self
    }
}

/// Adds border style helpers (`border`, `border-top`, ...).
pub trait Borderable: Widget {
    /// Set the CSS `border` shorthand.
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// View::new().border("1px solid var(--border)");
    /// ```
    fn border(&mut self, value: &str) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.node_style
            .insert("border".to_string(), value.to_string());
        self
    }
    /// Set the CSS `border-left`.
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// View::new().border_left("1px solid var(--border)");
    /// ```
    fn border_left(&mut self, value: &str) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.node_style
            .insert("border-left".to_string(), value.to_string());
        self
    }

    /// Set the CSS `border-right`.
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// View::new().border_right("1px solid var(--border)");
    /// ```
    fn border_right(&mut self, value: &str) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.node_style
            .insert("border-right".to_string(), value.to_string());
        self
    }

    /// Set the CSS `border-bottom`.
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// View::new().border_bottom("1px solid var(--border)");
    /// ```
    fn border_bottom(&mut self, value: &str) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.node_style
            .insert("border-bottom".to_string(), value.to_string());
        self
    }

    /// Set the CSS `border-top`.
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// View::new().border_top("1px solid var(--border)");
    /// ```
    fn border_top(&mut self, value: &str) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.node_style
            .insert("border-top".to_string(), value.to_string());
        self
    }
}

/// Adds padding style helpers (`padding`, `padding-top`, ...).
pub trait Paddingable: Widget {
    /// Set the CSS `padding` shorthand.
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// View::new().padding(vec![8, 12]);
    /// ```
    fn padding(&mut self, padding: Vec<i32>) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        let params: Vec<String> = padding.iter().map(|size| sp(size.clone())).collect();
        node.node_style
            .insert("padding".to_string(), params.join(" "));
        self
    }
    /// Set the CSS `padding-top`.
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// View::new().padding_top(8);
    /// ```
    fn padding_top(&mut self, value: i32) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.node_style.insert("padding-top".to_string(), sp(value));
        self
    }
    /// Set the CSS `padding-bottom`.
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// View::new().padding_bottom(8);
    /// ```
    fn padding_bottom(&mut self, value: i32) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.node_style
            .insert("padding-bottom".to_string(), sp(value));
        self
    }
    /// Set the CSS `padding-left`.
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// View::new().padding_left(8);
    /// ```
    fn padding_left(&mut self, value: i32) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.node_style
            .insert("padding-left".to_string(), sp(value));
        self
    }
    /// Set the CSS `padding-right`.
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// View::new().padding_right(8);
    /// ```
    fn padding_right(&mut self, value: i32) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.node_style
            .insert("padding-right".to_string(), sp(value));
        self
    }
}

/// A trait that provides methods to set dimension-related styles on widgets.
///
/// The `Dimensionable` trait builds upon widgets to allow for specifying their dimensions, such as
/// width, height, and their respective minimum and maximum constraints. Each method in this trait
/// updates the underlying widget's style with the respective dimension property.
/// Intended for Viewy widget authors.
///
/// # Requirements
///
/// Types implementing `Dimensionable` must also implement the `Widget` trait.
///
pub trait Dimensionable: Widget {
    /// Set the CSS `width`.
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// Button::new("Label", ButtonStyle::Filled).width("100px");
    /// ```
    fn width(&mut self, value: &str) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.node_style
            .insert("width".to_string(), value.to_string());
        self
    }
    /// Set the CSS `height`.
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// Button::new("Label", ButtonStyle::Filled).height("40px");
    /// ```
    fn height(&mut self, value: &str) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.node_style
            .insert("height".to_string(), value.to_string());
        self
    }
    /// Set the CSS `min-width`.
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// Button::new("Label", ButtonStyle::Filled).min_width("80px");
    /// ```
    fn min_width(&mut self, value: &str) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.node_style
            .insert("min-width".to_string(), value.to_string());
        self
    }
    /// Set the CSS `min-height`.
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// Button::new("Label", ButtonStyle::Filled).min_height("32px");
    /// ```
    fn min_height(&mut self, value: &str) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.node_style
            .insert("min-height".to_string(), value.to_string());
        self
    }
    /// Set the CSS `max-width`.
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// Button::new("Label", ButtonStyle::Filled).max_width("360px");
    /// ```
    fn max_width(&mut self, value: &str) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.node_style
            .insert("max-width".to_string(), value.to_string());
        self
    }
    /// Set the CSS `max-height`.
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// Button::new("Label", ButtonStyle::Filled).max_height("64px");
    /// ```
    fn max_height(&mut self, value: &str) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.node_style
            .insert("max-height".to_string(), value.to_string());
        self
    }
}

/// Supported values for the CSS `position` property.
pub enum Position {
    /// Default document flow positioning.
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// let position = Position::Static;
    /// ```
    Static,
    /// Position relative to its normal position.
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// let position = Position::Relative;
    /// ```
    Relative,
    /// Position relative to nearest positioned ancestor.
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// let position = Position::Absolute;
    /// ```
    Absolute,
    /// Position relative to viewport.
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// let position = Position::Fixed;
    /// ```
    Fixed,
}

/// Adds CSS positioning helpers.
pub trait Positionnable: Widget {
    /// Set the CSS `position` property.
    ///
    /// # Example
    /// ```rust,ignore
    /// use viewy::prelude::*;
    ///
    /// View::new().position(Position::Relative);
    /// ```
    fn position(&mut self, position: Position) -> &mut Self {
        let node: &mut Node = self.deref_mut();

        node.node_style.insert(
            "position".to_string(),
            match position {
                Position::Static => "static",
                Position::Relative => "relative",
                Position::Absolute => "absolute",
                Position::Fixed => "fixed",
            }
            .to_string(),
        );
        self
    }
}

/// Convenience trait bundling common box styling traits.
pub trait BoxStylable: Marginable + Borderable + Paddingable + Dimensionable {
    // ... potentially some additional common methods or overarching properties here ...
}

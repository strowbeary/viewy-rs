use crate::core::widget::Widget;
use crate::helper_fn::sp;
use crate::node::Node;

pub trait Marginable: Widget {
    fn margin(&mut self, margin: Vec<i32>) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        let params: Vec<String> = margin.iter().map(|size| sp(size.clone())).collect();
        node.node_style
            .insert("margin".to_string(), params.join(" "));
        self
    }
    fn margin_top(&mut self, value: i32) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.node_style.insert("margin-top".to_string(), sp(value));
        self
    }
    fn margin_bottom(&mut self, value: i32) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.node_style
            .insert("margin-bottom".to_string(), sp(value));
        self
    }
    fn margin_left(&mut self, value: i32) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.node_style.insert("margin-left".to_string(), sp(value));
        self
    }
    fn margin_right(&mut self, value: i32) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.node_style
            .insert("margin-right".to_string(), sp(value));
        self
    }
}

pub trait Borderable: Widget {
    fn border(&mut self, value: &str) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.node_style
            .insert("border".to_string(), value.to_string());
        self
    }
    fn border_left(&mut self, value: &str) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.node_style
            .insert("border-left".to_string(), value.to_string());
        self
    }

    fn border_right(&mut self, value: &str) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.node_style
            .insert("border-right".to_string(), value.to_string());
        self
    }

    fn border_bottom(&mut self, value: &str) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.node_style
            .insert("border-bottom".to_string(), value.to_string());
        self
    }

    fn border_top(&mut self, value: &str) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.node_style
            .insert("border-top".to_string(), value.to_string());
        self
    }
}

pub trait Paddingable: Widget {
    fn padding(&mut self, padding: Vec<i32>) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        let params: Vec<String> = padding.iter().map(|size| sp(size.clone())).collect();
        node.node_style
            .insert("padding".to_string(), params.join(" "));
        self
    }
    fn padding_top(&mut self, value: i32) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.node_style.insert("padding-top".to_string(), sp(value));
        self
    }
    fn padding_bottom(&mut self, value: i32) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.node_style
            .insert("padding-bottom".to_string(), sp(value));
        self
    }
    fn padding_left(&mut self, value: i32) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.node_style
            .insert("padding-left".to_string(), sp(value));
        self
    }
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
///
/// # Requirements
///
/// Types implementing `Dimensionable` must also implement the `Widget` trait.
///
/// # Examples
///
/// ```rust
/// // Assuming a `Button` struct implements both the `Widget` and `Dimensionable` traits.
/// let btn = Button::new("Label", ButtonStyle::Filled)
///    .width("100px")
///    .height("50px")
///    .min_width("50px");
/// ```
pub trait Dimensionable: Widget {
    fn width(&mut self, value: &str) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.node_style
            .insert("width".to_string(), value.to_string());
        self
    }
    fn height(&mut self, value: &str) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.node_style
            .insert("height".to_string(), value.to_string());
        self
    }
    fn min_width(&mut self, value: &str) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.node_style
            .insert("min-width".to_string(), value.to_string());
        self
    }
    fn min_height(&mut self, value: &str) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.node_style
            .insert("min-height".to_string(), value.to_string());
        self
    }
    fn max_width(&mut self, value: &str) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.node_style
            .insert("max-width".to_string(), value.to_string());
        self
    }
    fn max_height(&mut self, value: &str) -> &mut Self {
        let node: &mut Node = self.deref_mut();
        node.node_style
            .insert("max-height".to_string(), value.to_string());
        self
    }
}

pub enum Position {
    Static,
    Relative,
    Absolute,
    Fixed,
}

pub trait Positionnable: Widget {
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

pub trait BoxStylable: Marginable + Borderable + Paddingable + Dimensionable {
    // ... potentially some additional common methods or overarching properties here ...
}

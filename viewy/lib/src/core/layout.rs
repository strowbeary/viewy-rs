use crate::node::Node;

/// `Layout` is a type alias for a function that takes a `Node` and returns a `Node`.
/// This function will be used to transform the content of a Page.
///
/// # Example
/// A layout function might be used to wrap page content within a common site layout.
///
/// ```rust
/// use viewy::prelude::*;
/// fn layout(content: Node) -> Node {
///    // Components that compose the layout
/// }
///
/// let page = Page::with_title("Page Title")
///     .with_content(View::new())
///     .with_layout(&layout);
/// ```
pub type Layout<'a> = &'a dyn Fn(Node) -> Node;

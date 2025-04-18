use crate::core::layout::Layout;
use crate::core::node::Node;

/// Define a struct as a `Component` to use it in `append_child` method of widgets
/// ```rust
///
/// fn sub_component(data: &Homepage) -> View {
///     View::new()
/// }
///
/// use viewy::prelude::*;
/// #[derive(Component)]
/// struct Homepage {
///     pub user_name: String,
/// }
///
/// impl Component for Homepage {
///     fn render(self) -> Node {
///         sub_component(&self).into()
///     }
/// }
pub trait Component: Into<Node> {
    /// Will include the string returned as a script in Javascript in the page
    fn script() -> &'static str {
        ""
    }
    

    /// You can write the template of your component with viewy basic widgets or other components
    fn render(self) -> Node;
}

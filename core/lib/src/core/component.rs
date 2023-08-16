use crate::core::node::Node;

pub trait Component: Into<Node> {

    fn script() -> &'static str {
        ""
    }

    fn render(self) -> Node;
}

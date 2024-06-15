use crate::node::Node;

pub trait Layout {
    fn apply(self, node: Node) -> Node;
}

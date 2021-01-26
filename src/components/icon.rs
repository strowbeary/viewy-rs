use crate::Renderable;
use crate::node::Node;

#[derive(Debug, Clone)]
pub struct Icon {
    children: Vec<Box<dyn Renderable>>,
    pub node: Node,
    pub name: String
}
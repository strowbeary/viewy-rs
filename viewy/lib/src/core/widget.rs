use std::ops::{Deref, DerefMut};

use super::node::Node;

pub trait Widget: Deref<Target = Node> + DerefMut<Target = Node> + Into<Node> {
    const STYLE: &'static str;
    fn widget_name() -> &'static str;
}

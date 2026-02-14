use std::ops::{Deref, DerefMut};

use super::node::Node;

pub trait Widget: Deref<Target = Node> + DerefMut<Target = Node> + Into<Node> {
    const STYLE: &'static str;
    fn widget_name() -> &'static str;
}

pub struct WidgetStyleRegistration {
    pub style: &'static str,
}

impl WidgetStyleRegistration {
    pub const fn new(style: &'static str) -> Self {
        Self { style }
    }
}

::inventory::collect!(WidgetStyleRegistration);

pub fn iter_widget_styles() -> impl Iterator<Item = &'static WidgetStyleRegistration> {
    ::inventory::iter::<WidgetStyleRegistration>.into_iter()
}

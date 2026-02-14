use crate::core::widget::Widget;
use crate::node::Node;
use crate::prelude::{Attributable, Button, Classable, IconPack};
use crate::widgets::navigation_bar::NavigationBar;
use crate::widgets::toolbar::Toolbar;

#[derive(Widget, Classable, Attributable)]
#[widget(style = "./style.scss")]
pub struct Header {
    node: Node,
    left_button: Option<Button>,
    icon: Option<Box<dyn IconPack>>,
    title: String,
    main_toolbar: Option<Toolbar>,
    navigation_bar: Option<NavigationBar>,
}

impl Header {
    fn render(&mut self) {}
}

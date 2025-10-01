use crate::Widget;
use crate::prelude::Attributable;
use crate::prelude::Classable;
use crate::node::Node;

pub enum TextStyle {
    LargeTitle,
    H1,
    H2,
    H3,
    Headline,
    Subtitle1,
    Subtitle2,
    Subtitle3,
    Body,
    Article,
    Button,
    Label,
    Overline,
    Caption,
}

#[derive(Widget, Classable, Attributable)]
#[widget(style = "./style.scss")]
pub struct Text {
    node: Node,
    style: TextStyle,
}

impl Text {
    pub fn render(&mut self) {

    }
}
fn test() {}

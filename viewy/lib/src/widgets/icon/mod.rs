use crate::core::node::{Node, NodeType};
use crate::core::widget::Widget;
use crate::helper_fn::sp;
use crate::modifiers::{Attributable, Classable, Dimensionable};

pub mod icons;
pub use icons::*;

#[derive(Widget, Classable, Attributable, Dimensionable)]
#[widget(style = "./style.scss")]
pub struct Icon {
    node: Node,
    pub icon: Box<dyn icons::IconPack>,
    pub stroke_width: u8,
    pub size: i32,
}

impl Icon {
    pub fn new<T>(icon: T) -> Self
    where
        T: 'static + icons::IconPack,
    {
        let mut node = Node::default();
        node.node_type = NodeType::Normal("svg");

        Self {
            node,
            icon: Box::new(icon),
            stroke_width: 3,
            size: 24,
        }
    }

    pub fn stroke_width(&mut self, stroke_width: u8) -> &mut Self {
        self.stroke_width = stroke_width;
        self
    }

    pub fn size(&mut self, size: i32) -> &mut Self {
        self.size = size;
        self
    }

    pub fn render(&mut self) {
        let icon_pack = dyn_clone::clone_box(&*self.icon);
        icon_pack.configure(self);

        let size = sp(self.size);
        let stroke_width = self.stroke_width.to_string();
        let symbol_id = icon_pack.symbol_id();
        self.add_class("icon")
            .set_attr("xmlns", "http://www.w3.org/2000/svg")
            .set_attr("width", &size)
            .set_attr("height", &size)
            .set_attr("viewBox", "0 0 24 24")
            .set_attr("data-v-icon-id", symbol_id)
            .set_attr("aria-hidden", "true")
            .set_attr("stroke-width", &stroke_width)
            .set_attr("stroke-linecap", "round")
            .set_attr("stroke-linejoin", "round");

        self.min_height(&size);
        self.min_width(&size);
        self.node.text = Some(format!("<use href=\"#{}\"></use>", symbol_id));
    }
}

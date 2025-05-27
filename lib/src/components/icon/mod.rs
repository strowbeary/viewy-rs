use crate::Renderable;
use crate::components::icons::IconPack;
use crate::node::{Node, NodeContainer};
use crate::{DefaultModifiers, sp};

pub mod icons;

#[derive(Debug, Clone)]
pub struct Icon {
    node: Node,
    pub icon: Box<dyn IconPack>,
    pub stroke_width: String,
    pub size: i32,
}

impl Icon {
    pub fn new<T>(icon: T) -> Self
    where
        T: 'static + IconPack,
    {
        Icon {
            node: Default::default(),
            icon: Box::new(icon),
            stroke_width: "3".to_string(),
            size: 24,
        }
    }

    pub fn stroke_width(&mut self, stroke_width: i32) -> Self {
        self.stroke_width = stroke_width.to_string();
        self.clone()
    }

    pub fn size(&mut self, size: i32) -> Self {
        self.size = size;
        self.clone()
    }
}

impl NodeContainer for Icon {
    fn get_node(&mut self) -> &mut Node {
        &mut self.node
    }
}

impl DefaultModifiers for Icon {}

impl Renderable for Icon {
    fn render(mut self) -> Node {
        let mut icon = self.icon.configure(self);
        icon.add_class("icon")
            .set_attr("xmlns", "http://www.w3.org/2000/svg")
            .set_attr("width", sp(icon.size).as_str())
            .set_attr("height", sp(icon.size).as_str())
            .set_attr("viewBox", "0 0 24 24")
            .set_attr("stroke-width", icon.stroke_width.as_str())
            .set_attr("stroke-linecap", "round")
            .set_attr("stroke-linejoin", "round")
            .tag("svg");
        icon.min_height(&sp(icon.size));
        icon.min_width(&sp(icon.size));
        icon.get_node().text = Some(icon.icon.path().to_string());
        icon.node
    }
}

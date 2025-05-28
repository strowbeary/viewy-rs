use crate::Renderable;
use crate::components::icons::IconPack;
use crate::node::Node;
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
impl std::ops::Deref for Icon {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

impl std::ops::DerefMut for Icon {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.node
    }
}
impl DefaultModifiers for Icon {}

impl Renderable for Icon {
    fn render(self) -> Node {
        let self_values = self.clone();
        let mut icon = self_values.icon.configure(self);
        icon.add_class("icon")
            .set_attr("xmlns", "http://www.w3.org/2000/svg")
            .set_attr("width", sp(self_values.size).as_str())
            .set_attr("height", sp(self_values.size).as_str())
            .set_attr("viewBox", "0 0 24 24")
            .set_attr("stroke-width", self_values.stroke_width.as_str())
            .set_attr("stroke-linecap", "round")
            .set_attr("stroke-linejoin", "round")
            .tag("svg");
        icon.min_height(&sp(self_values.size));
        icon.min_width(&sp(self_values.size));
        icon.node.text = Some(self_values.icon.path().to_string());
        icon.node
    }
}

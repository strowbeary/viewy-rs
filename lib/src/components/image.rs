use crate::DefaultModifiers;
use crate::Renderable;
use crate::node::Node;

#[derive(Debug, Clone)]
pub enum ObjectFit {
    Fill,
    Contain,
    Cover,
    None,
    ScaleDown,
}

#[derive(Debug, Clone)]
pub struct Image {
    node: Node,
    pub src: String,
}

impl DefaultModifiers for Image {}

impl Image {
    pub fn new(src: &str) -> Self {
        Image {
            node: Default::default(),
            src: src.to_string(),
        }
    }
    pub fn object_fit(&mut self, fit: ObjectFit) -> &mut Self {
        self.node.node_style.push((
            "object-fit".to_string(),
            {
                match fit {
                    ObjectFit::Fill => "fill",
                    ObjectFit::Contain => "contain",
                    ObjectFit::Cover => "cover",
                    ObjectFit::None => "none",
                    ObjectFit::ScaleDown => "scale-down",
                }
            }
            .to_string(),
        ));

        self
    }
    pub fn aspect_ratio(&mut self, ratio: &str) -> &mut Self {
        self.node
            .node_style
            .push(("aspect-ratio".to_string(), ratio.to_string()));

        self
    }
}

impl std::ops::Deref for Image {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

impl std::ops::DerefMut for Image {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.node
    }
}

impl Renderable for Image {
    fn render(mut self) -> Node {
        let src = self.src.to_string();
        self.add_class("image").set_attr("src", &src).tag("img");
        self.node
    }
}

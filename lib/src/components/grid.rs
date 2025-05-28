use crate::components::Alignment;
use crate::components::Appendable;
use crate::node::Node;
use crate::{DefaultModifiers, Renderable, sp};

#[derive(Debug, Clone)]
pub struct Grid {
    node: Node,
    pub alignment: Alignment,
}

impl DefaultModifiers for Grid {}

impl Grid {
    pub fn new(alignment: Alignment) -> Self {
        Grid {
            node: Default::default(),
            alignment,
        }
    }
    pub fn gap(&mut self, gaps: Vec<i32>) -> &mut Self {
        let params: Vec<String> = gaps.iter().map(|size| sp(size.clone())).collect();
        self.node
            .node_style
            .push(("grid-gap".to_string(), params.join(" ")));
        self
    }
    pub fn columns(&mut self, schema: &str) -> &mut Self {
        self.node
            .node_style
            .push(("grid-template-columns".to_string(), schema.to_string()));
        self
    }

    //pub fn responsive_columns(&mut self, schemas: Vec<(&str, &str)>) -> -> &mut Self{}

    pub fn areas(&mut self, schema: &str) -> &mut Self {
        self.node
            .node_style
            .push(("grid-template-areas".to_string(), schema.to_string()));
        self
    }

    pub fn rows(&mut self, schema: &str) -> &mut Self {
        self.node
            .node_style
            .push(("grid-template-rows".to_string(), schema.to_string()));
        self
    }
    pub fn auto_columns(&mut self, size: &str) -> &mut Self {
        self.node
            .node_style
            .push(("grid-auto-columns".to_string(), size.to_string()));
        self
    }
    pub fn auto_rows(&mut self, size: &str) -> &mut Self {
        self.node
            .node_style
            .push(("grid-auto-rows".to_string(), size.to_string()));
        self
    }
    pub fn auto_flow(&mut self, direction: &str) -> &mut Self {
        self.node
            .node_style
            .push(("grid-auto-flow".to_string(), direction.to_string()));
        self
    }
    pub fn align_items(&mut self, value: &str) -> &mut Self {
        self.node
            .node_style
            .push(("align-items".to_string(), value.to_string()));
        self
    }
    pub fn justify_content(&mut self, value: &str) -> &mut Self {
        self.node
            .node_style
            .push(("justify-content".to_string(), value.to_string()));
        self
    }
    pub fn justify_items(&mut self, value: &str) -> &mut Self {
        self.node
            .node_style
            .push(("justify-items".to_string(), value.to_string()));
        self
    }
}

impl std::ops::Deref for Grid {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

impl std::ops::DerefMut for Grid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.node
    }
}

impl Appendable for Grid {}

impl Renderable for Grid {
    fn render(mut self) -> Node {
        let alignment = format!("grid--align-{:?}", self.alignment).to_lowercase();
        self.add_class("grid").add_class(&alignment);
        self.node
    }
}

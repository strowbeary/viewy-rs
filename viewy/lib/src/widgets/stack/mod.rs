use crate::{Widget, sp};

pub mod horizontal;
pub mod vertical;

pub use horizontal::*;
pub use vertical::*;
use crate::modifiers::Cardifiable;

pub trait Stack: Widget + Cardifiable {
    fn new(alignment: Alignment) -> Self;

    fn justify_content(&mut self, justification: &str) -> &mut Self {
        self.node_style
            .push(("justify-content".to_string(), justification.to_string()));
        self
    }

    fn gap(&mut self, gaps: Vec<i32>) -> &mut Self {
        let params: Vec<String> = gaps.iter().map(|size| sp(size.clone())).collect();
        self.node_style.push(("gap".to_string(), params.join(" ")));
        self
    }
    fn flex_wrap(&mut self) -> &mut Self {
        self.node_style
            .push(("flex-wrap".to_string(), "wrap".to_string()));
        self
    }
    fn render(&mut self) {}
}
#[derive(Debug, Clone)]
pub enum Alignment {
    Center,
    Start,
    End,
    Stretch,
}

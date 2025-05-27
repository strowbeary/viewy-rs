use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use crate::node::Node;
use std::ops::Deref;
use dyn_clone::{DynClone, clone_trait_object, clone};

pub trait Renderable: DynClone + Send + Sync + Debug {
    fn render(self) -> Node;

    fn component_name(&self) -> &str {
        "View"
    }
}

clone_trait_object!(Renderable);

impl Renderable for Box<dyn Renderable> {
    fn render(self) -> Node {
        self.render()
    }
}

impl<T: Renderable + Clone> Renderable for &T {
    fn render(self) -> Node {
        self.clone().render()
    }
}
impl<T: Renderable + Clone> Renderable for &mut T {
    fn render(self) -> Node {
        self.clone().render()
    }
}

impl PartialEq for Box<dyn Renderable> {
    fn eq(&self, other: &Box<dyn Renderable>) -> bool {
        self.deref().component_name().eq(other.deref().component_name())
    }
}
impl Eq for Box<dyn Renderable> {

}


impl Hash for Box<dyn Renderable> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.deref().component_name().hash(state);
    }
}
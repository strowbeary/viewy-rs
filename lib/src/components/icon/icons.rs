use std::fmt::Debug;
use dyn_clone::{clone_trait_object, DynClone};
use crate::components::Icon;

pub trait IconPack: DynClone + Send + Sync + Debug {
    fn path(&self) -> &'static str;
    fn configure(&self, icon: Icon) -> Icon;
}

clone_trait_object!(IconPack);

impl IconPack for Box<dyn IconPack> {
    fn path(&self) -> &'static str {
        (*self.clone()).path()
    }

    fn configure(&self, icon: Icon) -> Icon {
        (*self.clone()).configure(icon)
    }
}

include!(concat!(env!("OUT_DIR"), "/icons.rs"));
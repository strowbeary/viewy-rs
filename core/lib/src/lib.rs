#![feature(in_band_lifetimes)]
extern crate minifier;
extern crate grass;
extern crate serde;
extern crate serde_json;
extern crate html_escape;
extern crate uuid;
extern crate dyn_clone;

mod helper_fn;
mod node;
mod modifiers;
mod renderer;

mod page;
pub use page::{Page, RenderMode};

/// Module containing all ui components of viewy-rs
pub mod components;

pub mod engine;

pub use modifiers::{DefaultModifiers, Overflow};
pub use renderer::{Renderable};
pub use helper_fn::*;



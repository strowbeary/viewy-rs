extern crate minifier;
extern crate grass;
extern crate serde;
extern crate serde_json;
extern crate html_escape;
extern crate uuid;
extern crate dyn_clone;
extern crate toml;
extern crate base64;
extern crate lazy_static;

pub use engine::Renderable;
pub use helper_fn::*;
pub use modifiers::{DefaultModifiers, Overflow, Position};
pub use page::{Page, RenderMode};
use lazy_static::lazy_static;
use theme::{Config, ConfigLoader};

mod helper_fn;
mod node;
mod modifiers;

mod page;
/// Module containing all ui components of viewy-rs
pub mod components;
pub mod layouts;

pub mod engine;
mod theme;


lazy_static!{
    pub static ref CONFIG: Config = ConfigLoader::load_from_config_folder();
}

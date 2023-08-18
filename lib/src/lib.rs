extern crate minifier;
extern crate grass;
extern crate serde;
extern crate serde_json;
extern crate html_escape;
extern crate uuid;
extern crate dyn_clone;
extern crate toml;
extern crate base64;

pub use engine::Renderable;
pub use helper_fn::*;
pub use modifiers::{DefaultModifiers, Overflow, Position};
pub use page::{Page, RenderMode};
pub use config::Config;

mod helper_fn;
mod node;
mod modifiers;

mod page;
/// Module containing all ui components of viewy-rs
pub mod components;
pub mod layouts;

pub mod engine;
mod config;

#[cfg(test)]
mod tests {
    use std::time::Instant;
    use crate::components::{Appendable, View};
    use crate::Renderable;

    #[test]
    fn it_works() {

        let start = Instant::now();
        let mut view = View::new();
        for i in 0..10000 {
            view.append_child(View::new());
        }

        println!("{}", view.to_html());
        let duration = start.elapsed();

        println!("{:?}", duration);
    }
}
#[cfg(test)]
mod tests {
    use std::time::Instant;
    use crate::components::{Appendable, View};
    use crate::Renderable;

    #[test]
    fn it_works() {

        let start = Instant::now();
        let mut view = View::new();
        for i in 0..10000 {
            view.append_child(View::new());
        }

        println!("{}", view.to_html());
        let duration = start.elapsed();

        println!("{:?}", duration);
    }
}
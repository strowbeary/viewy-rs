extern crate minifier;
extern crate grass;
extern crate serde;
extern crate serde_json;
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
    use crate::components::{Appendable, Button, ButtonStyle, View};
    use crate::{Page, Renderable, RenderMode};

    #[test]
    fn benchmark() {

        let start = Instant::now();
        let mut view = View::new();
        for i in 0..50000 {
            view.append_child(
                Button::new("Label", ButtonStyle::Filled));
        }
        let render_start = Instant::now();
        let _ = Page::new("Render test", &|content| {
            Box::new(View::new().append_child(content))
        } ,view).compile(RenderMode::Complete);


        let render_duration = render_start.elapsed();
        let duration = start.elapsed();
        println!("Render duration = {:?}", render_duration);
        println!("Global duration = {:?}", duration);
    }
}
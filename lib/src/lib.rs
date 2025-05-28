extern crate base64;
extern crate dyn_clone;
extern crate grass;
extern crate minifier;
extern crate serde;
extern crate serde_json;
extern crate toml;
extern crate uuid;

pub use config::Config;
pub use engine::Renderable;
pub use helper_fn::*;
pub use modifiers::{DefaultModifiers, Overflow, Position};
pub use page::{Page, RenderMode};

mod helper_fn;
mod modifiers;
pub mod node;

/// Module containing all ui components of viewy-rs
pub mod components;
pub mod layouts;
mod page;

mod config;
pub mod engine;

#[cfg(test)]
mod tests {
    use crate::components::{Appendable, Button, ButtonStyle, View};
    use crate::{Page, RenderMode, Renderable};
    use std::time::Instant;

    #[test]
    fn benchmark() {
        let start = Instant::now();
        let mut view = View::new();
        for i in 0..50000 {
            view.append_child(Button::new("Label", ButtonStyle::Filled));
        }
        let render_start = Instant::now();
        let _ = Page::new(
            "Render test",
            &|content| {
                let mut view = View::new();

                view.node.children.push(content);
                view.render()
            },
            view,
        )
        .compile(RenderMode::Complete);

        let render_duration = render_start.elapsed();
        let duration = start.elapsed();
        println!("Render duration = {:?}", render_duration);
        println!("Global duration = {:?}", duration);
    }
}

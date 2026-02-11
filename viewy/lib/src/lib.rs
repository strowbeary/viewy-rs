#![doc(html_favicon_url = "https://viewy.rs/logos/favicon.svg")]
#![doc(html_logo_url = "https://viewy.rs/logos/logo-light.svg")]

//! # Viewy Rust Documentation
//!
//! Welcome to the Viewy documentation! Viewy is an innovative web UI toolkit designed with the modern developer in mind. If you've ever wished for a seamless blend of the power and flexibility of a design system, with the ease and simplicity of an UI library, then Viewy might just be what you've been looking for.
//!
//! #### Why Choose Viewy?
//!
//! - **Simplicity**: Viewy has an intuitive and expressive API that makes it easy to generate UI widgets.
//!
//! - **Customizability**: A unique feature of Viewy is its configurability through the `Viewy.toml` file. This provides developers the flexibility to fine-tune the appearance and behavior of widgets, without having to dig deep into the codebase.
//!
//! - **Rich Component Library**: From buttons and cards to navigation bars, Viewy offers a wide variety of widgets that cater to most web development needs.
//!
//! #### Quick Start
//!
//! Getting started with Viewy is easy. Here's a small snippet to showcase how you can generate a webpage with a button:
//!
//! ```rust
//! use viewy::prelude::*;
//!
//! #[derive(Component)]
//! struct MyPage {
//!     pub btn_label: String
//! }
//!
//! impl Component for MyPage {
//!     fn render(self) -> Node {
//!         View::new().append_child({
//!              Button::new(&self.btn_label, ButtonStyle::Filled)
//!          })
//!             .into()
//!     }
//! }
//! fn main() {
//! Page::with_title("Test")
//!     .with_content({
//!         MyPage {
//!             btn_label: "Hello".to_string()
//!         }
//!             .into()
//!     })
//!     .compile(RenderMode::Complete);
//! }
//! ```
//!
//! In this example, a page with the title "Test" is generated, and a filled-style button with the label "Hello" is appended to it.
//!
//! #### Customizing with Viewy.toml
//!
//! Most of Viewy's widgets can be customized using the `Viewy.toml` configuration file. By specifying your design tokens in this file, you can achieve a consistent look and feel across your web application, without the repetitive code.
//!
//! For detailed guidelines on using the `Viewy.toml` file, refer to the documentation of the [`Config`](./prelude/struct.Config.html) struct.
//!
//! ---
//!
//! As you navigate through this documentation, you'll discover more advanced features, tips, and best practices to make the most of Viewy. Dive in, and happy coding!

#[macro_use]
extern crate viewy_codegen;
extern crate figment;
extern crate serde;
#[doc(inline)]
pub use crate::core::component::*;
pub use crate::core::node;
use crate::core::widget::Widget;
use lazy_static::lazy_static;
#[doc(inline)]
pub use viewy_codegen::*;

pub use strum;

mod core;

pub mod bindings;
pub mod widgets;

mod helper_fn;
pub mod modifiers;

#[doc(inline)]
pub use crate::helper_fn::*;

use crate::core::config::Config;
lazy_static! {
    static ref CONFIG: Config = Config::load();
}

pub mod prelude {
    pub use crate::core::component::Component;
    pub use crate::core::config::Config;
    pub use crate::core::layout::*;
    pub use crate::core::node::*;
    pub use crate::core::page::*;
    pub use crate::core::theme::*;
    pub use crate::helper_fn::*;
    pub use crate::modifiers::*;
    pub use crate::widgets::button::*;
    pub use crate::widgets::sheet::*;
    pub use crate::widgets::stack::*;
    pub use crate::widgets::text::*;
    pub use crate::widgets::view::*;
}

#[cfg(test)]
mod tests {
    use grass::{InputSyntax, Options, OutputStyle};
    use std::time::Instant;

    use crate::modifiers::Appendable;
    use crate::prelude::*;
    use crate::prelude::{Page, RenderMode};

    #[test]
    fn benchmark() {
        let start = Instant::now();
        let mut view = View::new();

        for _ in 0..50000 {
            view.append_child({ Button::new("Label", ButtonStyle::Filled) });
        }

        let render_start = Instant::now();
        let _ = Page::with_title("Test")
            .with_content(view)
            .compile(RenderMode::Complete);
        let render_duration = render_start.elapsed();
        let duration = start.elapsed();

        println!("Render duration = {:?}", render_duration);
        println!("Global duration = {:?}", duration);
        // fs::write("./output.html", html).expect("Can't write output");
    }

    #[test]
    fn can_append_child() {
        let html = Page::with_title("Test")
            .with_content({
                let mut view = View::new();
                view.append_child({ Button::new("Hello", ButtonStyle::Filled) });
                view
            })
            .compile(RenderMode::Complete);

        println!("{}", html);
    }

    #[test]
    fn compile_styles() {
        let stylesheets = crate::widgets::get_all_stylesheet().join("");
        println!("SCSS stylesheets size = {} bytes", stylesheets.len());
        if let Ok(compiled_stylesheet) = grass::from_string(
            stylesheets,
            &Options::default()
                .style(OutputStyle::Compressed)
                .quiet(true)
                .input_syntax(InputSyntax::Scss),
        ) {
            println!("Stylesheet size = {} bytes", compiled_stylesheet.len());
        } else {
            panic!("Error during scss compilation")
        }
    }
}

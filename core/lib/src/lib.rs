#![doc(html_favicon_url = "https://viewy.rs/logos/favicon.svg")]
#![doc(html_logo_url = "https://viewy.rs/logos/logo-light.svg")]

#![warn(missing_docs)]

//! # Viewy Rust Documentation
//!
//! Welcome to the Viewy documentation! Viewy is an innovative web UI toolkit designed with the modern developer in mind. If you've ever wished for a seamless blend of the power and flexibility of a design system, with the ease and simplicity of an UI library, then Viewy might just be what you've been looking for.
//!
//! #### Why Choose Viewy?
//!
//! - **Simplicity**: Viewy has an intuitive and expressive API that makes it easy to generate UI components.
//!
//! - **Customizability**: A unique feature of Viewy is its configurability through the `Viewy.toml` file. This provides developers the flexibility to fine-tune the appearance and behavior of components, without having to dig deep into the codebase.
//!
//! - **Rich Component Library**: From buttons and cards to navigation bars, Viewy offers a wide variety of components that cater to most web development needs.
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
//!          });
//!     }
//! }
//! fn main() {
//! Page::with_title("Test")
//!     .with_content({
//!         MyPage {
//!             btn_label: "Hello".to_string()
//!         }
//!     })
//!     .compile(RenderMode::Complete);
//! }
//! ```
//!
//! In this example, a page with the title "Test" is generated, and a filled-style button with the label "Hello" is appended to it.
//!
//! #### Customizing with Viewy.toml
//!
//! Most of Viewy's components can be customized using the `Viewy.toml` configuration file. By specifying your design tokens in this file, you can achieve a consistent look and feel across your web application, without the repetitive code.
//!
//! For detailed guidelines on using the `Viewy.toml` file, refer to the documentation of the [`Config`](./prelude/struct.Config.html) struct.
//!
//! ---
//!
//! As you navigate through this documentation, you'll discover more advanced features, tips, and best practices to make the most of Viewy. Dive in, and happy coding!

#[macro_use]
extern crate viewy_codegen;
extern crate serde;
extern crate figment;

#[doc(inline)]
pub use crate::core::component::*;
pub use crate::core::modifiers;
pub use crate::core::node;
use crate::core::widget::Widget;

mod core;

/// This module contains all http framework specific implementations on viewy objects.
///
/// Supported frameworks :
///
/// | Supported frameworks | Feature tag |
/// | :--------------------|:-----------:|
/// | Rocket-rs            | `rocket`    |
pub mod bindings;
pub mod widgets;


mod helper_fn;

#[doc(inline)]
pub use crate::helper_fn::*;

pub use crate::core::page;

pub mod prelude {
    pub use crate::widgets::button::*;
    pub use crate::widgets::popup::*;
    pub use crate::widgets::view::*;
    pub use crate::core::component::Component;
    pub use crate::core::modifiers::*;
    pub use crate::core::node::*;
    pub use crate::helper_fn::*;
    pub use crate::core::page::*;
    pub use crate::core::config::Config;
    pub use crate::core::theme::*;
}


#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::core::modifiers::Appendable;
    use crate::core::node::Node;
    use crate::core::page::{Page, RenderMode};
    use crate::core::widget::Widget;
    use crate::prelude::*;

    #[derive(Component)]
    pub struct UserProfileTag {
        pub user_name: String,
        pub user_profile_img: String,
    }

    impl Component for UserProfileTag {
        fn render(self) -> Node {
            Button::new("label", ButtonStyle::Filled).into()
        }
    }

    #[test]
    fn benchmark() {
        let start = Instant::now();
        let mut view = View::new();

        for _ in 0..10000 {
            view
                .append_child({
                    Button::new("Label", ButtonStyle::Filled)
                });
        }

        let _ = Page::with_title("Test")
            .with_content(view)
            .compile(RenderMode::Complete);
        let duration = start.elapsed();

        println!("{:?}", duration);
        // fs::write("./output.html", html).expect("Can't write output");
    }

    #[test]
    fn can_append_child() {


        let html = Page::with_title("Test")
            .with_content({
                View::new().append_child({
                    Button::new("Hello", ButtonStyle::Filled)
                });
            })
            .compile(RenderMode::Complete);

        println!("{}", html);
    }

    #[test]
    fn compile_styles() {
        println!("{}", crate::widgets::get_all_stylesheet().join(""));
    }
}
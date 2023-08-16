#[macro_use]
extern crate viewy_codegen;

use std::ops::{Deref, DerefMut};

use crate::core::widget::Widget;


mod core;
mod components;

pub mod prelude {
    pub use crate::components::button::*;
    pub use crate::components::view::*;
    pub use crate::components::popup::*;
    pub use crate::core::component::Component;
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::Instant;

    use crate::core::modifiers::Appendable;
    use crate::core::node::Node;
    use crate::core::widget::Widget;
    use crate::prelude::*;

    pub struct UserProfileTag {
        pub user_name: String,
        pub user_profile_img: String,
    }

    impl UserProfileTag {
        pub fn new(name: &str, profile_img: &str) -> Self {
            UserProfileTag {
                user_name: name.to_string(),
                user_profile_img: profile_img.to_string(),
            }
        }
    }

    impl Into<Node> for UserProfileTag {
        fn into(self) -> Node {
            self.render()
        }
    }

    impl Component for UserProfileTag {
        fn render(self) -> Node {
            Button::new("label", ButtonStyle::Filled).into()
        }
    }

    #[test]
    fn it_works() {
        let start = Instant::now();
        let mut view = View::new();

        for i in 0..10000 {
            view
                .append_child({
                    Button::new("Label", ButtonStyle::Filled)
                        .reversed(true)
                });
        }

        let html = view.to_html();
        let duration = start.elapsed();

        println!("{:?}", duration);
        fs::write("./output.html", html).expect("Can't write output");
    }
}
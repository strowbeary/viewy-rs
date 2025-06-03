mod default;
mod login;

pub use default::default_layout;
pub use login::login_layout;
use viewy::{Layout, node::Node};

pub fn no_layout() -> Layout {
    Box::new(|content| content)
}

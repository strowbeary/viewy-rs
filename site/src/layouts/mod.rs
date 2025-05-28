mod default;
mod login;

pub use default::default_layout;
pub use login::login_layout;
use viewy::node::Node;

pub fn no_layout(content: Node) -> Node {
    content
}

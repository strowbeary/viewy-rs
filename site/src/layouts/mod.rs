mod default;
mod login;

pub use default::default_layout;
pub use login::login_layout;
use viewy::Renderable;

pub fn no_layout(content: Box<dyn Renderable>) -> Box<dyn Renderable> {
    content
}
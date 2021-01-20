mod stack;
mod button;
mod card;
mod page;

#[macro_export]
macro_rules! component_style {
    ($l:literal) => { include_str!(concat!("../themes/components/", $l, ".scss")) }
}

#[macro_export]
macro_rules! component_script {
    ($l:literal) => { include_str!(concat!("../js/", $l, ".js")) }
}

pub use stack::*;
pub use button::*;
pub use card::*;


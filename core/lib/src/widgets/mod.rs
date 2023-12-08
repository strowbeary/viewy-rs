use crate::core::widget::Widget;

pub mod view;
pub mod button;
pub mod popup;
pub mod stack;

#[cfg(feature = "rich-text-area")]
pub mod rich_text_area;

#[cfg(feature = "sortable-stack")]
pub mod sortable_stack;

pub fn get_all_scripts()-> Vec<&'static str> {
    vec![
        view::View::SCRIPT,
        button::Button::SCRIPT,
        popup::Popup::SCRIPT,
        stack::VStack::SCRIPT
    ]
}


pub const fn get_all_stylesheet() -> &'static [&'static str; 4] {
    &[
        view::View::STYLE,
        button::Button::STYLE,
        popup::Popup::STYLE,
        stack::VStack::STYLE
    ]
}


use crate::core::widget::Widget;

pub mod button;
pub mod form;
pub mod popup;
#[cfg(feature = "rich-text-area")]
pub mod rich_text_area;
pub mod stack;
pub mod view;

#[cfg(feature = "sortable-stack")]
pub mod sortable_stack;

pub fn get_all_scripts() -> Vec<&'static str> {
    vec![
        view::View::SCRIPT,
        button::Button::SCRIPT,
        popup::Popup::SCRIPT,
        stack::VStack::SCRIPT,
        stack::HStack::SCRIPT,
    ]
}

pub const fn get_all_stylesheet() -> &'static [&'static str; 5] {
    &[
        view::View::STYLE,
        button::Button::STYLE,
        popup::Popup::STYLE,
        stack::VStack::STYLE,
        stack::HStack::STYLE,
    ]
}

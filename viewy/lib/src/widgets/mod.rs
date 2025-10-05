use crate::core::widget::Widget;

pub mod button;
pub mod form;
pub mod popup;
#[cfg(feature = "rich-text-area")]
pub mod rich_text_area;
pub mod stack;
pub mod view;

pub mod text;

#[cfg(feature = "sortable-stack")]
pub mod sortable_stack;

pub const fn get_all_stylesheet() -> &'static [&'static str; 5] {
    &[
        view::View::STYLE,
        button::Button::STYLE,
        popup::Popup::STYLE,
        stack::VStack::STYLE,
        stack::HStack::STYLE,
    ]
}

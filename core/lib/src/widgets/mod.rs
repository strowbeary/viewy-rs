use crate::core::widget::Widget;

pub mod view;
pub mod button;
pub mod popup;
pub mod stack;

#[cfg(feature = "rich-text-area")]
pub mod rich_text_area;

pub const fn get_all_stylesheet() -> &'static [&'static str; 3] {
    &[
        view::View::STYLE,
        button::Button::STYLE,
        popup::Popup::STYLE
    ]
}


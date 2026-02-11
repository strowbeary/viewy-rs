use crate::CONFIG;
use crate::core::widget::Widget;

pub mod button;
pub mod form;
#[cfg(feature = "rich-text-area")]
pub mod rich_text_area;
pub mod stack;
pub mod tabs;
pub mod view;

pub mod text;

pub mod breadcrumb;
pub mod header;

pub mod sheet;
pub mod tag;

/*
*TODO
* ComboBox : a combination of select and input search
* Toast : A toast provider that managed a stack of toasts.
* Toggle (checkbox, switch and button)
* Picker (Select, Segment, RadioGroup)
* Toolbar : A group of buttons, pickers, toggles, links, labels or combobox presented as a coherent container.
**/

#[cfg(feature = "sortable-stack")]
pub mod sortable_stack;

pub const fn get_all_stylesheet() -> &'static [&'static str; 6] {
    &[
        view::View::STYLE,
        button::Button::STYLE,
        stack::VStack::STYLE,
        stack::HStack::STYLE,
        text::Text::STYLE,
        tabs::TabContainer::STYLE,
    ]
}

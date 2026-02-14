use std::sync::OnceLock;

pub mod button;
pub mod form;
pub mod icon;
pub mod picker;
#[cfg(feature = "rich-text-area")]
pub mod rich_text_area;
pub mod select;
pub mod stack;
pub mod tabs;
pub mod view;

pub mod text;

pub mod breadcrumb;
pub mod header;

pub mod navigation_bar;
pub mod sheet;
pub mod tag;
pub mod toolbar;

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

static ALL_STYLESHEETS: OnceLock<Vec<&'static str>> = OnceLock::new();

pub fn get_all_stylesheet() -> &'static [&'static str] {
    ALL_STYLESHEETS
        .get_or_init(|| {
            let styles = crate::core::widget::iter_widget_styles().collect::<Vec<_>>();
            styles.into_iter().map(|entry| entry.style).collect()
        })
        .as_slice()
}

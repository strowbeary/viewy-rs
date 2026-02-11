use std::iter::Scan;

use viewy::bindings::uri::Uri;
use viewy::prelude::{
    Action, Appendable, Button, ButtonStyle, OnClickActionnable, Paddingable, Page, SheetEdge,
    Stack, VStack,
};
use viewy::scale;

use crate::create_button_group;
use crate::rocket_uri_macro_benchmark;

#[get("/sheet")]
pub fn sheet() -> Page<'static> {
    Page::with_title("Test").with_content({
        let mut button = Button::new("Open sheet", ButtonStyle::Filled);
        button.on_click(Action::OpenSheet {
            edge: SheetEdge::Right,
            sheet_content_url: Uri::from(uri!(sheet_content())),
            with_backdrop: true,
        });
        button
    })
}

#[get("/sheet-content")]
pub fn sheet_content() -> Page<'static> {
    Page::with_title("Sheet Content").with_content(
        VStack::new(viewy::prelude::Alignment::Stretch)
            .padding(vec![scale(5)])
            .append_child(Button::new("Open popup", ButtonStyle::Filled).on_click(
                Action::OpenPopup {
                    popup_content_url: Uri::from(uri!(sheet_content)),
                    display_window_controls: true,
                },
            ))
            .append_child(create_button_group(ButtonStyle::Filled)),
    )
}

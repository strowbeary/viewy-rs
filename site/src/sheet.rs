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
            edge: SheetEdge::Left,
            sheet_content_url: Uri::from(uri!(benchmark())),
        });
        button
    })
}

#[get("/sheet-content")]
pub fn sheet_content() -> Page<'static> {
    Page::with_title("Sheet Content").with_content(
        VStack::new(viewy::prelude::Alignment::Stretch)
            .padding(vec![scale(5)])
            .append_child(create_button_group(ButtonStyle::Filled)),
    )
}

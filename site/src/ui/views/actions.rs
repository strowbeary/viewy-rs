use rocket::http::hyper::header::ACCEPT;
use viewy::{prelude::*, widgets::form::Form};

pub fn actions() -> VStack {
    let mut stack = VStack::new(Alignment::Stretch);
    stack
        .gap(vec![scale(5)])
        .append_child(Text::new("Actions", TextStyle::LargeTitle))
        .append_child(
            Button::new("Popup", ButtonStyle::Filled).on_click(Action::OpenPopup {
                popup_content_url: uri!(crate::popover_content()),
                display_window_controls: true,
            }),
        )
        .append_child(
            Button::new("Popover", ButtonStyle::Filled).on_click(Action::OpenPopover {
                popover_content_url: uri!(crate::popover_content()),
            }),
        )
        .append_child(
            Button::new("Sheet", ButtonStyle::Filled).on_click(Action::OpenSheet {
                sheet_content_url: uri!(crate::popover_content()),
                edge: SheetEdge::Bottom,
                with_backdrop: false,
            }),
        )
        .append_child(
            VStack::new(Alignment::Stretch)
                .gap(vec![scale(5)])
                .append_child(Form::new(
                    viewy::widgets::form::FormMethod::Get,
                    uri!(crate::popover_content()),
                ))
                .append_child(Button::new("Submit Form", ButtonStyle::Filled)),
        );
    stack
}

use viewy::{DefaultModifiers, scale};
use viewy::components::*;

pub fn table() -> VStack {
    VStack::new(Alignment::Stretch)
        .padding(vec![scale(4)])
        .gap(vec![scale(5)])
        .append_child({
            VStack::new(Alignment::Stretch)
                .append_child({
                    Text::new("Table", TextStyle::H1)
                })
                .append_child({
                    Divider::new()
                })
                .append_child({
                    Form::new("liste", "")
                        .append_child({
                            let mut table = Table::new("test_tableau", vec![
                                Column::new(Some("First col"))
                                    .width("50%"),
                                Column::new(Some("Second col"))
                                    .width("50%"),
                            ])
                                .selectable(true)
                                .width("100%");

                            for i in 1..20 {
                                table.append_child({
                                    Row::new(i.to_string().as_str())
                                        .append_child({
                                            Text::new(i.to_string().as_str(), TextStyle::Body)
                                        })
                                        .append_child({
                                            Text::new("Row content", TextStyle::Body)
                                        })
                                });
                            }
                            table
                        })
                        .append_child({
                            Button::new("Envoyer", ButtonStyle::Filled)
                                .attach_to_form("liste")
                        })
                })
        })
        .append_child({
            VStack::new(Alignment::Stretch)
                .append_child({
                    Text::new("Sortable stack", TextStyle::H1)
                })
                .append_child({
                    Divider::new()
                })
                .append_child({
                    SortableStack::new()
                        .append_child({
                            Text::new("Item 1", TextStyle::LargeTitle)
                        })
                        .append_child({
                            Text::new("Item 2", TextStyle::LargeTitle)
                        })
                        .append_child({
                            Text::new("Item 3", TextStyle::LargeTitle)
                        })
                })
        })
}
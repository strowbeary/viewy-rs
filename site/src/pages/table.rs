use viewy::components::*;
use viewy::{DefaultModifiers, scale};

pub fn table() -> VStack {
    let mut stack = VStack::new(Alignment::Stretch);
    stack
        .padding(vec![scale(4)])
        .gap(vec![scale(5)])
        .append_child(
            VStack::new(Alignment::Stretch)
                .append_child(Text::new("Table", TextStyle::H1))
                .append_child(Divider::new())
                .append_child(
                    Form::new("liste", "")
                        .append_child({
                            let mut table = Table::new(
                                "test_tableau",
                                vec![
                                    {
                                        let mut col1 = Column::new(Some("First col"));
                                        col1.width("50%");
                                        col1
                                    },
                                    {
                                        let mut col2 = Column::new(Some("Second col"));
                                        col2.width("50%");
                                        col2
                                    },
                                ],
                            );
                            table.selectable(true).width("100%");

                            for i in 1..20 {
                                let mut row = Row::new(i.to_string().as_str());
                                row.append_child(Text::new(
                                    i.to_string().as_str(),
                                    TextStyle::Body,
                                ))
                                .append_child(Text::new("Row content", TextStyle::Body));
                                table.append_child(row);
                            }
                            table
                        })
                        .append_child(
                            Button::new("Envoyer", ButtonStyle::Filled).attach_to_form("liste"),
                        ),
                ),
        )
        .append_child(
            VStack::new(Alignment::Stretch)
                .append_child(Text::new("Sortable stack", TextStyle::H1))
                .append_child(Divider::new())
                .append_child(
                    SortableStack::new()
                        .append_child(Text::new("Item 1", TextStyle::LargeTitle))
                        .append_child(Text::new("Item 2", TextStyle::LargeTitle))
                        .append_child(Text::new("Item 3", TextStyle::LargeTitle)),
                ),
        );
    stack
}

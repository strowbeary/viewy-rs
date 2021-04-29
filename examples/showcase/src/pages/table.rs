use viewy::components::*;
use viewy::DefaultModifiers;

pub fn table() -> VStack {
    VStack::new(Alignment::Stretch)
        .append_child({
            Text::new("Table", TextStyle::H1)
        })
        .append_child({
            Divider::new()
        })
        .append_child({
            let mut table = Table::new("test_tableau", vec![
                Column::new(Some("First col"))
                    .width("50%"),
                Column::new(Some("Second col"))
                    .width("50%"),
            ])
                .selectable(true)
                .width("100%");

            for i in 1..100 {
                table.append_row({
                    Row::new(i.to_string().as_str())
                        .append_child({
                            Text::new(i.to_string().as_str(), TextStyle::Body)
                        })
                        .append_child({
                            Text::new("ho 2", TextStyle::Body)
                        })
                });
            }
            table
        })
}
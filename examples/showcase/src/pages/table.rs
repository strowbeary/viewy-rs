use viewy::components::*;
use viewy::DefaultModifiers;

pub fn table() -> Table {
    Table::new("test_tableau", vec![
        Column::new(Some("First col"))
            .width("50%"),
        Column::new(Some("Second col"))
            .width("50%"),
    ])
        .selectable(true)
        .width("100%")
        .append_row({
            Row::new("1")
                .append_child({
                    Text::new("hey", TextStyle::Body)
                })
                .append_child({
                    Text::new("ho", TextStyle::Body)
                })
        })
        .append_row({
            Row::new("2")
                .append_child({
                    Text::new("hey 2", TextStyle::Body)
                })
                .append_child({
                    Text::new("ho 2", TextStyle::Body)
                })
        })
        .append_row({
            Row::new("3")
                .append_child({
                    Text::new("hey 2", TextStyle::Body)
                })
                .append_child({
                    Text::new("ho 2", TextStyle::Body)
                })
        })
}
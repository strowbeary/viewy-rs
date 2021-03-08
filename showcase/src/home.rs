use viewy::components::*;
use viewy::{DefaultModifiers, sp};

pub fn home() -> VStack {
    let mut o = VStack::new(Alignment::Stretch);
    o.add_view_child({
        TitleBar::new("Viewy showcase")
            .left_item({
                Button::new("Back", ButtonStyle::Link)
                    .icon("arrow-left")
                    .action("/macro")
                    .grid_area("left_item")
            })
            .bottom_item({
                TextField::new("Search", FieldType::Search)
                    .placeholder("Search for everything")
                    .grid_area("bottom_item")
            })
            .right_item({
                Button::new("Donate", ButtonStyle::Filled)
                    .icon("dollar-sign")
                    .grid_area("right_item")
            })
    });

    o.add_view_child({
        let mut o =
            VStack::new(Alignment::Stretch)
                .gap(vec![20])
                .padding(vec![30]);

        o.add_view_child({
            let mut o = Card::new(CardStyle::Raised)
                .padding(vec![30]);
            o.add_view_child({
                Text::new("Texts", TextStyle::H1)
                    .margin_bottom(25)
            });
            o.add_view_child({
                let mut o =
                    VStack::new(Alignment::Start)
                        .gap(vec![16])
                        .width("50%");
                o.add_view_child(Text::new("Large title", TextStyle::LargeTitle));
                o.add_view_child(Text::new("Title 1", TextStyle::H1));
                o.add_view_child(Text::new("Subtitle 1", TextStyle::Subtitle1));
                o.add_view_child(Text::new("Title 2", TextStyle::H2));
                o.add_view_child(Text::new("Subtitle 2", TextStyle::Subtitle2));
                o.add_view_child(Text::new("Title 3", TextStyle::H3));
                o.add_view_child(Text::new("Subtitle 3", TextStyle::Subtitle3));
                o.add_view_child(Text::new("Headline", TextStyle::Headline));
                o.add_view_child(Text::new("Body", TextStyle::Body));
                o.add_view_child(Text::new("Button", TextStyle::Button));
                o.add_view_child(Text::new("Label", TextStyle::Label));
                o.add_view_child(Text::new("Overline", TextStyle::Overline));
                o.add_view_child(Text::new("Caption", TextStyle::Caption));
                o.add_view_child(ComplexText::new(r#"
The `ComplexText` component allows you to use **markdown** annotations.

It is a *long established* fact that a reader will be **distracted** by the readable content of a page when looking at its layout. The point of using Lorem Ipsum is that it has a more-or-less normal distribution of letters, as opposed to using 'Content here, content here', making it look like readable English.
"#, TextStyle::Body));
                o
            });
            o
        });
        o.add_view_child({
            let mut o = Card::new(CardStyle::Raised)
                .padding(vec![30]);
            o.add_view_child({
                Text::new("Form", TextStyle::H1)
                    .margin_bottom(25)
            });
            o.add_view_child({
                let mut o =
                    VStack::new(Alignment::Start)
                        .gap(vec![16])
                        .width("50%");
                o.add_view_child({
                    let mut o = Form::new("/").async_form();
                    o.add_view_child(TextField::new("input2", FieldType::Text)
                        .label("Label"));
                    o.add_view_child(
                        Button::new("submit", ButtonStyle::Filled)
                        .set_attr("type", "submit")
                    );
                    o
                });
                o
            });
            o
        });
        o.add_view_child({
            let mut o = Card::new(CardStyle::Raised)
                .padding(vec![30]);
            o.add_view_child({
                Text::new("Buttons", TextStyle::H1)
                    .margin_bottom(25)
            });
            o.add_view_child({
                let mut o =
                    HStack::new(Alignment::Center)
                        .gap(vec![16]);
                o.add_view_child({
                    Button::new("Hello", ButtonStyle::Link)
                });
                o.add_view_child({
                    Button::new("Hello", ButtonStyle::Flat)
                });
                o.add_view_child({
                    Button::new("Hello", ButtonStyle::Outlined)
                });
                o.add_view_child({
                    Button::new("Hello", ButtonStyle::Filled)
                });
                o
            });
            o.add_view_child({
                let mut o =
                    HStack::new(Alignment::Center)
                        .gap(vec![16])
                        .margin_top(20);
                o.add_view_child({
                    Button::new("Valider", ButtonStyle::Link)
                        .icon("check")
                });
                o.add_view_child({
                    Button::new("Valider", ButtonStyle::Flat)
                        .icon("check")
                });
                o.add_view_child({
                    Button::new("Valider", ButtonStyle::Outlined)
                        .icon("check")
                });
                o.add_view_child({
                    Button::new("Valider", ButtonStyle::Filled)
                        .icon("check")
                });
                o
            });
            o
        });
        o.add_view_child({
            let mut o = Card::new(CardStyle::Raised)
                .padding(vec![30]);
            o.add_view_child({
                Text::new("Text Field", TextStyle::H1)
                    .margin_bottom(25)
            });
            o.add_view_child({
                let mut o =
                    VStack::new(Alignment::Stretch)
                        .gap(vec![16]);
                o.add_view_child({
                    TextField::new("input1", FieldType::Text)
                        .placeholder("Optional placeholder")
                });
                o.add_view_child({
                    TextField::new("input2", FieldType::Text)
                        .label("Label")
                });
                o.add_view_child({
                    TextField::new("input3", FieldType::Text)
                        .label("Label")
                        .helper_text("Message d'aide")
                });
                o
            });
            o
        });
        o.add_view_child({
            let mut o = Card::new(CardStyle::Raised)
                .padding(vec![30]);
            o.add_view_child({
                Text::new("Pickers", TextStyle::H1)
                    .margin_bottom(25)
            });
            o.add_view_child({
                let mut o =
                    VStack::new(Alignment::Stretch)
                        .gap(vec![16]);
                o.add_view_child({
                    let mut o = Picker::new("Hello", "2", PickerStyle::Segmented)
                        .label("Segmented picker");
                    o.add_view_child({
                        PickerOption::new("Test 1", "1")
                            .icon("user")
                    });
                    o.add_view_child(PickerOption::new("Test 2", "2"));
                    o.add_view_child(PickerOption::new("Test 3", "3"));
                    o
                });
                o.add_view_child({
                    let mut o = Picker::new("Hello", "2", PickerStyle::RadioGroup)
                        .label("Radio group picker");
                    o.add_view_child({
                        PickerOption::new("Test 1 - ignored icon", "1")
                            .icon("user")
                    });
                    o.add_view_child(PickerOption::new("Test 2", "2"));
                    o.add_view_child(PickerOption::new("Test 3", "3"));
                    o
                });
                o
            });
            o
        });
        o.add_view_child({
            let mut o = Card::new(CardStyle::Raised)
                .padding(vec![30]);
            o.add_view_child({
                Text::new("Image", TextStyle::H1)
                    .margin_bottom(25)
            });
            o.add_view_child({
                let mut o =
                    VStack::new(Alignment::Stretch)
                        .gap(vec![16]);
                o.add_view_child({
                    Image::new("https://image.shutterstock.com/z/stock-photo-small-white-flowers-on-a-toned-on-gentle-soft-blue-and-pink-background-outdoors-close-up-macro-549094105.jpg")
                        .width(sp(250).as_str())
                        .height(sp(250).as_str())
                });
                o
            });
            o
        });
        o.add_view_child({
            let mut o = Card::new(CardStyle::Raised)
                .padding(vec![30]);
            o.add_view_child({
                Text::new("Popover", TextStyle::H1)
                    .margin_bottom(25)
            });
            o.add_view_child({
                let mut o =
                    VStack::new(Alignment::Stretch)
                        .gap(vec![16]);
                o.add_view_child({
                    Button::new("Open popover", ButtonStyle::Filled)
                        .popover({
                            let mut o = Popover::new();
                            o.add_view_child(Text::new("Popover content", TextStyle::H1));
                            o
                        })
                });
                o.add_view_child({
                    Button::new("Open popover 2", ButtonStyle::Filled)
                        .popover({
                            let mut o = Popover::new()
                                .placement("bottom-end");
                            o.add_view_child(Text::new("Popover content 2", TextStyle::H1));
                            o
                        })
                });
                o
            });
            o
        });
        o
    });
    o
}
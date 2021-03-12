use viewy::components::*;
use viewy::{DefaultModifiers, sp};

pub fn home() -> VStack {
    let mut o = VStack::new(Alignment::Stretch);
    o.append_child({
        TitleBar::new("Viewy showcase")
            .left_item({
                let mut o = HStack::new(Alignment::Stretch);
                o.append_child(Button::new("Back", ButtonStyle::Link)
                    .icon("arrow-left")
                    .action("/macro"));
                o.append_child(
                    Button::new("Login", ButtonStyle::Link)
                        .action("/login")
                );
                o
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

    o.append_child({
        let mut o =
            VStack::new(Alignment::Stretch)
                .gap(vec![20])
                .padding(vec![30]);

        o.append_child({
            let mut o = Card::new(CardStyle::Raised)
                .padding(vec![30]);
            o.append_child({
                Text::new("Texts", TextStyle::H1)
                    .margin_bottom(25)
            });
            o.append_child({
                let mut o =
                    VStack::new(Alignment::Start)
                        .gap(vec![16])
                        .width("50%");
                o.append_child(Text::new("Large title", TextStyle::LargeTitle));
                o.append_child(Text::new("Title 1", TextStyle::H1));
                o.append_child(Text::new("Subtitle 1", TextStyle::Subtitle1));
                o.append_child(Text::new("Title 2", TextStyle::H2));
                o.append_child(Text::new("Subtitle 2", TextStyle::Subtitle2));
                o.append_child(Text::new("Title 3", TextStyle::H3));
                o.append_child(Text::new("Subtitle 3", TextStyle::Subtitle3));
                o.append_child(Text::new("Headline", TextStyle::Headline));
                o.append_child(Text::new("Body", TextStyle::Body));
                o.append_child(Text::new("Button", TextStyle::Button));
                o.append_child(Text::new("Label", TextStyle::Label));
                o.append_child(Text::new("Overline", TextStyle::Overline));
                o.append_child(Text::new("Caption", TextStyle::Caption));
                o.append_child(ComplexText::new(r#"
The `ComplexText` component allows you to use **markdown** annotations.

It is a *long established* fact that a reader will be **distracted** by the readable content of a page when looking at its layout. The point of using Lorem Ipsum is that it has a more-or-less normal distribution of letters, as opposed to using 'Content here, content here', making it look like readable English.
"#, TextStyle::Body));
                o
            });
            o
        });
        o.append_child({
            let mut o = Card::new(CardStyle::Raised)
                .padding(vec![30]);
            o.append_child({
                Text::new("Form", TextStyle::H1)
                    .margin_bottom(25)
            });

            o.append_child({
                let mut o = Form::new("test-form", "/").async_form();
                o.append_child({
                    let mut o =
                        VStack::new(Alignment::Start)
                            .gap(vec![16]);
                    o.append_child(TextField::new("input2", FieldType::Text)
                        .label("Label"));
                    o.append_child(
                        Button::new("submit", ButtonStyle::Filled)
                            .set_attr("type", "submit")
                    );
                    o
                });
                o
            });
            o
        });
        o.append_child({
            let mut o = Card::new(CardStyle::Raised)
                .padding(vec![30]);
            o.append_child({
                Text::new("Buttons", TextStyle::H1)
                    .margin_bottom(25)
            });
            o.append_child({
                let mut o =
                    HStack::new(Alignment::Center)
                        .gap(vec![16]);
                o.append_child({
                    Button::new("Hello", ButtonStyle::Link)
                });
                o.append_child({
                    Button::new("Hello", ButtonStyle::Flat)
                });
                o.append_child({
                    Button::new("Hello", ButtonStyle::Outlined)
                });
                o.append_child({
                    Button::new("Hello", ButtonStyle::Filled)
                });
                o
            });
            o.append_child({
                let mut o =
                    HStack::new(Alignment::Center)
                        .gap(vec![16])
                        .margin_top(20);
                o.append_child({
                    Button::new("Valider", ButtonStyle::Link)
                        .icon("check")
                });
                o.append_child({
                    Button::new("Valider", ButtonStyle::Flat)
                        .icon("check")
                });
                o.append_child({
                    Button::new("Valider", ButtonStyle::Outlined)
                        .icon("check")
                });
                o.append_child({
                    Button::new("Valider", ButtonStyle::Filled)
                        .icon("check")
                });
                o
            });
            o
        });
        o.append_child({
            let mut o = Card::new(CardStyle::Raised)
                .padding(vec![30]);
            o.append_child({
                Text::new("Text Field", TextStyle::H1)
                    .margin_bottom(25)
            });
            o.append_child({
                let mut o =
                    VStack::new(Alignment::Stretch)
                        .gap(vec![16]);
                o.append_child({
                    TextField::new("input1", FieldType::Text)
                        .placeholder("Optional placeholder")
                });
                o.append_child({
                    TextField::new("input2", FieldType::Text)
                        .label("Label")
                });
                o.append_child({
                    TextField::new("input3", FieldType::Text)
                        .label("Label")
                        .helper_text("Message d'aide")
                });
                o
            });
            o
        });
        o.append_child({
            let mut o = Card::new(CardStyle::Raised)
                .padding(vec![30]);
            o.append_child({
                Text::new("Pickers", TextStyle::H1)
                    .margin_bottom(25)
            });
            o.append_child({
                let mut o =
                    VStack::new(Alignment::Stretch)
                        .gap(vec![16]);
                o.append_child({
                    let mut o = Picker::new("Hello", "2", PickerStyle::Segmented)
                        .label("Segmented picker");
                    o.append_child({
                        PickerOption::new("Test 1", "1")
                            .icon("user")
                    });
                    o.append_child(PickerOption::new("Test 2", "2"));
                    o.append_child(PickerOption::new("Test 3", "3"));
                    o
                });
                o.append_child({
                    let mut o = Picker::new("Hello", "2", PickerStyle::RadioGroup)
                        .label("Radio group picker");
                    o.append_child({
                        PickerOption::new("Test 1 - ignored icon", "1")
                            .icon("user")
                    });
                    o.append_child(PickerOption::new("Test 2", "2"));
                    o.append_child(PickerOption::new("Test 3", "3"));
                    o
                });
                o
            });
            o
        });
        o.append_child({
            let mut o = Card::new(CardStyle::Raised)
                .padding(vec![30]);
            o.append_child({
                Text::new("Image", TextStyle::H1)
                    .margin_bottom(25)
            });
            o.append_child({
                let mut o =
                    VStack::new(Alignment::Stretch)
                        .gap(vec![16]);
                o.append_child({
                    Image::new("https://image.shutterstock.com/z/stock-photo-small-white-flowers-on-a-toned-on-gentle-soft-blue-and-pink-background-outdoors-close-up-macro-549094105.jpg")
                        .width(sp(250).as_str())
                        .height(sp(250).as_str())
                });
                o
            });
            o
        });
        o.append_child({
            let mut o = Card::new(CardStyle::Raised)
                .padding(vec![30]);
            o.append_child({
                Text::new("Popover", TextStyle::H1)
                    .margin_bottom(25)
            });
            o.append_child({
                let mut o =
                    VStack::new(Alignment::Stretch)
                        .gap(vec![16]);
                o.append_child({
                    Button::new("Open popover", ButtonStyle::Filled)
                        .popover({
                            let mut o = Popover::new();
                            o.append_child(Text::new("Popover content", TextStyle::H1));
                            o
                        })
                });
                o.append_child({
                    Button::new("Open popover 2", ButtonStyle::Filled)
                        .popover({
                            let mut o = Popover::new()
                                .placement("bottom-end");
                            o.append_child(Text::new("Popover content 2", TextStyle::H1));
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
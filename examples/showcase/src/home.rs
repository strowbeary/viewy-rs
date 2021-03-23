use viewy::components::*;
use viewy::{DefaultModifiers, sp};
use viewy::component::Appendable;

pub fn home() -> VStack {
    VStack::new(Alignment::Stretch)
        .append_child({
            TitleBar::new("Viewy showcase")
                .right_item({
                    Button::new("Login", ButtonStyle::Flat)
                        .icon("user")
                        .action("/login")
                })
                .bottom_item({
                    TextField::new("Search", FieldType::Search)
                        .placeholder("Search for everything")
                })
        })
        .append_child({
            VStack::new(Alignment::Stretch)
                .gap(vec![12])
                .padding(vec![16])
                .append_child({
                    Card::new(CardStyle::Raised)
                        .padding(vec![30])
                        .append_child({
                            Text::new("Texts", TextStyle::H1)
                                .margin_bottom(25)
                        })
                        .append_child(Divider::new())
                        .append_child({
                            VStack::new(Alignment::Start)
                                .gap(vec![16])
                                .width("50%")
                                .append_child(Text::new("Large title", TextStyle::LargeTitle))
                                .append_child(Text::new("Title 1", TextStyle::H1))
                                .append_child(Text::new("Subtitle 1", TextStyle::Subtitle1))
                                .append_child(Text::new("Title 2", TextStyle::H2))
                                .append_child(Text::new("Subtitle 2", TextStyle::Subtitle2))
                                .append_child(Text::new("Title 3", TextStyle::H3))
                                .append_child(Text::new("Subtitle 3", TextStyle::Subtitle3))
                                .append_child(Text::new("Headline", TextStyle::Headline))
                                .append_child(Text::new("Body", TextStyle::Body))
                                .append_child(Text::new("Button", TextStyle::Button))
                                .append_child(Text::new("Label", TextStyle::Label))
                                .append_child(Text::new("Overline", TextStyle::Overline))
                                .append_child(Text::new("Caption", TextStyle::Caption))
                                .append_child(ComplexText::new(r#"
The `ComplexText` component allows you to use **markdown** annotations.

It is a *long established* fact that a reader will be **distracted** by the readable content of a page when looking at its layout.
The point of using Lorem Ipsum is that it has a more-or-less normal distribution of letters,
as opposed to using 'Content here, content here', making it look like readable English.
"#, TextStyle::Body))
                        })
                })
                .append_child({
                    Card::new(CardStyle::Raised)
                        .padding(vec![30])
                        .append_child({
                            Text::new("Form", TextStyle::H1)
                                .margin_bottom(25)
                        })
                        .append_child({
                            Form::new("test-form", "/")
                                .async_form()
                                .append_child({
                                    VStack::new(Alignment::Start)
                                        .gap(vec![16])
                                        .append_child(TextField::new("input2", FieldType::Text)
                                            .label("Label"))
                                        .append_child(
                                            Button::new("submit", ButtonStyle::Filled)
                                                .set_attr("type", "submit")
                                        )
                                })
                        })
                })
                .append_child({
                    Card::new(CardStyle::Raised)
                        .padding(vec![30])
                        .append_child({
                            Text::new("Buttons", TextStyle::H1)
                                .margin_bottom(25)
                        })
                        .append_child({
                            HStack::new(Alignment::Center)
                                .gap(vec![16])
                                .append_child({
                                    Button::new("Hello", ButtonStyle::Link)
                                })
                                .append_child({
                                    Button::new("Hello", ButtonStyle::Flat)
                                })
                                .append_child({
                                    Button::new("Hello", ButtonStyle::Outlined)
                                })
                                .append_child({
                                    Button::new("Hello", ButtonStyle::Filled)
                                })
                        })
                        .append_child({
                            HStack::new(Alignment::Center)
                                .gap(vec![16])
                                .margin_top(20)
                                .append_child({
                                    Button::new("Valider", ButtonStyle::Link)
                                        .icon("check")
                                })
                                .append_child({
                                    Button::new("Valider", ButtonStyle::Flat)
                                        .icon("check")
                                })
                                .append_child({
                                    Button::new("Valider", ButtonStyle::Outlined)
                                        .icon("check")
                                })
                                .append_child({
                                    Button::new("Valider", ButtonStyle::Filled)
                                        .icon("check")
                                })
                        })
                })
                .append_child({
                    Card::new(CardStyle::Raised)
                        .padding(vec![30])
                        .append_child({
                            Text::new("Text Field", TextStyle::H1)
                                .margin_bottom(25)
                        })
                        .append_child({
                            VStack::new(Alignment::Stretch)
                                .gap(vec![16])
                                .append_child({
                                    TextField::new("input1", FieldType::Text)
                                        .placeholder("Optional placeholder")
                                })
                                .append_child({
                                    TextField::new("input2", FieldType::Text)
                                        .label("Label")
                                })
                                .append_child({
                                    TextField::new("input3", FieldType::Text)
                                        .label("Label")
                                        .helper_text("Message d'aide")
                                })
                        })
                })
                .append_child({
                    Card::new(CardStyle::Raised)
                        .padding(vec![30])
                        .append_child({
                            Text::new("Pickers", TextStyle::H1)
                                .margin_bottom(25)
                        })
                        .append_child({
                            VStack::new(Alignment::Stretch)
                                .gap(vec![16])
                                .append_child({
                                    Picker::new("Hello", "2", PickerStyle::Segmented)
                                        .label("Segmented picker")
                                        .append_child({
                                            PickerOption::new("Test 1", "1")
                                                .icon("user")
                                        })
                                        .append_child(PickerOption::new("Test 2", "2"))
                                        .append_child(PickerOption::new("Test 3", "3"))
                                }).append_child({
                                Picker::new("Hello", "2", PickerStyle::RadioGroup)
                                    .label("Radio group picker")
                                    .append_child({
                                        PickerOption::new("Test 1 - ignored icon", "1")
                                            .icon("user")
                                    })
                                    .append_child(PickerOption::new("Test 2", "2"))
                                    .append_child(PickerOption::new("Test 3", "3"))
                            })
                        })
                })
                .append_child({
                    Card::new(CardStyle::Raised)
                        .padding(vec![30])
                        .append_child({
                            Text::new("Image", TextStyle::H1)
                                .margin_bottom(25)
                        })
                        .append_child({
                            VStack::new(Alignment::Stretch)
                                .gap(vec![16]).append_child({
                                Image::new("https://image.shutterstock.com/z/stock-photo-small-white-flowers-on-a-toned-on-gentle-soft-blue-and-pink-background-outdoors-close-up-macro-549094105.jpg")
                                    .width(sp(250).as_str())
                                    .height(sp(250).as_str())
                            })
                        })
                })
                .append_child({
                    Card::new(CardStyle::Raised)
                        .padding(vec![30])
                        .append_child({
                            Text::new("Popover", TextStyle::H1)
                                .margin_bottom(25)
                        })
                        .append_child({
                            VStack::new(Alignment::Stretch)
                                .gap(vec![16])
                                .append_child({
                                    Button::new("Open popover", ButtonStyle::Filled)
                                        .popover({
                                            Popover::new()
                                                .append_child(Text::new("Popover content", TextStyle::H1))
                                        })
                                })
                                .append_child({
                                    Button::new("Open popover 2", ButtonStyle::Filled)
                                        .popover({
                                            Popover::new()
                                                .placement("bottom-end")
                                                .append_child(Text::new("Popover content 2", TextStyle::H1))
                                        })
                                })
                        })
                })
        })
}
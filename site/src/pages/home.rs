use chrono::Duration;

use viewy::components::icons::Lucide;
use viewy::components::*;
use viewy::{DefaultModifiers, scale, sp};

use crate::components::showcase_section;

pub fn home() -> VStack {
    let mut stack = VStack::new(Alignment::Stretch);
    stack.padding(vec![scale(4)])
        .gap(vec![12])
        .append_child(
            Button::new("Open Popup with vidéo", ButtonStyle::Filled)
                .popup(
                    Popup::new()
                        .append_child(
                        View::new()
                            .tag("iframe")
                            .set_attr("src", "https://www.youtube.com/embed/G603PcNkl6k?si=smNq_FJcs19a_jTH")
                            .set_attr("allow", "accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share")
                            .set_attr("allow-fullscreen", "allow-fullscreen")
                            .border("none")
                            .width("100%")
                            .height("100%")
                    )
                )
        )
        .append_child(
            Card::new(CardStyle::Outlined)
                .popup(Popup::new())
                .append_child(
                    Form::new("totofile", "")
                        .append_child(
                            FileInput::new("toto", FileInputType::Hidden)
                        )
                        .append_child(
                            Button::new("Files", ButtonStyle::Filled)
                                .attach_to_file_input("toto")
                        )
                )
        )
        .append_child(
            showcase_section("Progress bar",
                VStack::new(Alignment::Stretch)
                    .gap(vec![scale(3)])
                    .append_child(
                        ProgressBar::new()
                    )
                    .append_child(
                        ProgressBar::new()
                            .value(0.5)
                    )
            )
        )
        .append_child(
            showcase_section("Disclosure",
                VStack::new(Alignment::Stretch)
                    .gap(vec![scale(3)])
                    .append_child(
                        Disclosure::new()
                            .opener_item(
                                Text::new("Opener item content", TextStyle::H3)
                            )
                            .append_child(
                                Text::new(
                                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.",
                                    TextStyle::Body,
                                )
                                    .margin_top(scale(3))
                           )
                    )
                    .append_child(
                        Disclosure::new()
                            .opener_item(
                                Text::new("Title 1", TextStyle::H3)
                            )
                            .append_child(
                                Disclosure::new()
                                    .margin_left(24)
                                    .opener_item(
                                        Text::new("Subtitle 1", TextStyle::H3)
                                    )
                                    .append_child(
                                        Text::new(
                                            "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.",
                                            TextStyle::Body,
                                        )
                                            .margin_top(scale(3))
                                    )
                            )
                    )
            )
        ).append_child(
        showcase_section("Gauge",
            HStack::new(Alignment::Center)
                .gap(vec![scale(3)])
                .append_child(
                    Gauge::new(0.5, GaugeStyle::Radial)
                        .low(0.3)
                        .high(0.8)
                        .optimum(1.0)
                )
                .append_child(
                    Gauge::new(0.5, GaugeStyle::Radial)
                        .display_optimum_indicator()
                        .low(0.3)
                        .high(0.8)
                        .optimum(1.0)
                )
        )
    )
        .append_child(
            showcase_section("Highlighted card",
                HStack::new(Alignment::Center)
                    .gap(vec![scale(3)])
                    .append_child(
                        Card::new(CardStyle::Outlined)
                            .remove_highlight_on_submit("remove-highlight")
                            .highlighted(true)
                            .padding(vec![scale(4)])
                            .add_class("clickable")
                            .append_child(
                                Text::new("Outlined", TextStyle::Headline)
                            )
                    )
                    .append_child(
                        Card::new(CardStyle::Filled)
                            .remove_highlight_on_submit("remove-highlight")
                            .highlighted(true)
                            .padding(vec![scale(4)])
                            .add_class("clickable")
                            .append_child(
                                Text::new("Filled", TextStyle::Headline)
                            )
                    )
                    .append_child(
                        Form::new("remove-highlight", "")
                            .async_form()
                            .append_child(
                                Button::new("Remove highlight", ButtonStyle::Filled)
                                    .attach_to_form("remove-highlight")
                            )
                    )
            )
        )
        .append_child(
            showcase_section("Badge",
                VStack::new(Alignment::Start)
                    .gap(vec![16])
                    .width("50%")
                    .append_child(
                        Button::new("Label", ButtonStyle::Outlined)
                            .badge(&125)
                    )
                    .append_child(
                        Button::new("Label", ButtonStyle::Outlined)
                            .badge(&12)
                    )
                    .append_child(
                        Button::new("Label", ButtonStyle::Outlined)
                            .badge(&1)
                    )
                    .append_child(
                        Button::icon_only(Lucide::Bell, ButtonStyle::Flat)
                            .badge(&1)
                    )
                    .append_child(
                        Button::icon_only(Lucide::Bell, ButtonStyle::Flat)
                            .badge(&900)
                    )
            )
        )
        .append_child(
            showcase_section("Text",
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
            )
        )
        .append_child(
            showcase_section("Form",
                Form::new("test-form", "/")
                    .async_form()
                    .append_child(
                        VStack::new(Alignment::Start)
                            .gap(vec![16])
                            .append_child(Field::new("input2", FieldType::Text)
                                .label("Label"))
                            .append_child(
                                Button::new("submit", ButtonStyle::Filled)
                                    .set_attr("type", "submit")
                            )
                    )
            )
        )
        .append_child(
            showcase_section("Button",
                VStack::new(Alignment::Stretch)
                    .append_child(
                        HStack::new(Alignment::Center)
                            .gap(vec![16])
                            .append_child(
                                Button::new("Hello", ButtonStyle::SmallLink)
                            )
                            .append_child(
                                Button::new("Hello", ButtonStyle::Link)
                            )
                            .append_child(
                                Button::new("Hello", ButtonStyle::Flat)
                            )
                            .append_child(
                                Button::new("Hello", ButtonStyle::Outlined)
                            )
                            .append_child(
                                Button::new("Hello", ButtonStyle::Filled)
                            )
                    )
                    .append_child(
                        HStack::new(Alignment::Center)
                            .gap(vec![16])
                            .margin_top(20)
                            .append_child(
                                Button::new("Valider", ButtonStyle::SmallLink)
                                    .icon(Lucide::Check)
                            )
                            .append_child(
                                Button::new("Valider", ButtonStyle::Link)
                                    .icon(Lucide::Check)
                            )
                            .append_child(
                                Button::new("Valider", ButtonStyle::Flat)
                                    .icon(Lucide::Check)
                            )
                            .append_child(
                                Button::new("Valider", ButtonStyle::Outlined)
                                    .icon(Lucide::Check)
                            )
                            .append_child(
                                Button::new("Valider", ButtonStyle::Filled)
                                    .icon(Lucide::Check)
                            )
                    )
                    .append_child(
                        HStack::new(Alignment::Center)
                            .gap(vec![16])
                            .margin_top(20)
                            .append_child(
                                Button::icon_only(Lucide::Check, ButtonStyle::SmallLink)
                            )
                            .append_child(
                                Button::icon_only(Lucide::Check, ButtonStyle::Link)
                            )
                            .append_child(
                                Button::icon_only(Lucide::Check, ButtonStyle::Flat)
                            )
                            .append_child(
                                Button::icon_only(Lucide::Check, ButtonStyle::Outlined)
                            )
                            .append_child(
                                Button::icon_only(Lucide::Check, ButtonStyle::Filled)
                            )
                    )
            )
        )
        .append_child(
            showcase_section("Text field",
                VStack::new(Alignment::Stretch)
                    .gap(vec![16])
                    .append_child(
                        Field::new("input1", FieldType::Text)
                            .placeholder("Optional placeholder")
                    )
                    .append_child(
                        Field::new("input2", FieldType::Text)
                            .label("Label")
                    )
                    .append_child(
                        Field::new("input3", FieldType::Text)
                            .label("Label")
                            .helper_text("Message d'aide")
                    )
                    .append_child(
                        Field::new("input4", FieldType::Text)
                            .label("Label")
                            .error_message("Message d'erreur")
                    )
                    .append_child(
                        Form::new("required_test", "")
                            .append_child(
                                Field::new("input5", FieldType::Text)
                                    .label("Champ de texte")
                                    .helper_text("Indication sur le type de donnée à mettre dans le champ")
                                    .required(true)
                            )
                            .append_child(
                                Button::new("Valider", ButtonStyle::Filled)
                                    .attach_to_form("required_test")
                            )
                    )
                    .append_child(
                        Field::new("input6", FieldType::Duration(vec![
                            Duration::minutes(15),
                            Duration::minutes(75),
                        ]))
                            .label("Champ de durée")
                            .helper_text("Indication sur le type de donnée à mettre dans le champ")
                            .required(true)
                    )
            )
        )
        .append_child(
            showcase_section("Picker",
                VStack::new(Alignment::Stretch)
                    .gap(vec![16])
                    .append_child(Picker::new("test1", "2", PickerStyle::Segmented)
                            .label("Segmented picker")
                            .append_child({
                                let mut option = PickerOption::new("Test 3", "3");
                                option.icon(Lucide::Users);
                                option
                            })
                            .append_child( PickerOption::new("Test 2", "2"))
                            .append_child(PickerOption::new("Test 3", "3"))
                    )
                    .append_child(
                        Picker::new("test2", "2", PickerStyle::RadioGroup)
                            .label("Radio group picker")
                            .append_child({
                                let mut option = PickerOption::new("Test 1 - ignored icon", "1");
                                option.icon(Lucide::User);
                                option
                            })
                            .append_child(
                                PickerOption::new("Test 2", "2")
                            )
                            .append_child(
                                PickerOption::new("Test 3", "3")
                            )
                    )
                    .append_child(
                        Picker::new("test3", "2", PickerStyle::Dropdown)
                            .label("Dropdown picker")
                            .append_child({
                                let mut option = PickerOption::new("Test 1 - ignored icon", "1");
                                option.icon(Lucide::User);
                                option
                            })
                            .append_child(
                                PickerOption::new("Test 2", "2")
                            )
                            .append_child({
                                PickerOption::new("Test 3", "3")
                            })
                    )
            )
        )
        .append_child(
            showcase_section("Checkbox",
                VStack::new(Alignment::Stretch)
                    .gap(vec![16])
                    .append_child(
                        Checkbox::new("test1", "2", CheckboxStyle::Checkbox)
                            .label("Checkbox with label checked by default")
                            .is_checked(true)
                    )
                    .append_child(
                        Checkbox::new("test1", "2", CheckboxStyle::Checkbox)
                            .label("Checkbox with label")
                    )
                    .append_child(
                        Checkbox::new("test1", "2", CheckboxStyle::Checkbox)
                    )
                    .append_child(
                        Checkbox::new("switch", "2", CheckboxStyle::Switch)
                            .label("Switch button")
                    )
            )
        )
        .append_child(
            showcase_section("Image",
                VStack::new(Alignment::Stretch)
                    .gap(vec![16]).append_child(
                    Image::new("https://image.shutterstock.com/z/stock-photo-small-white-flowers-on-a-toned-on-gentle-soft-blue-and-pink-background-outdoors-close-up-macro-549094105.jpg")
                        .width(sp(250).as_str())
                        .height(sp(250).as_str())
                )
            )
        )
        .append_child(
            showcase_section("Popover",
                VStack::new(Alignment::Stretch)
                    .gap(vec![16])
                    .append_child(
                        Button::new("Open popover", ButtonStyle::Filled)
                            .popover({
                                let mut popover = Popover::new();
                                popover.placement(Placement::BottomEnd)
                                    .append_child(Text::new("Popover content 2", TextStyle::H1));
                                popover
                            })
                    )
                    .append_child(
                        Button::new("Open popover 2", ButtonStyle::Filled)
                            .popover({
                                let mut popover = Popover::new();
                                popover.placement(Placement::Bottom)
                                    .append_child(Text::new("Popover content", TextStyle::H1));
                                popover
                            })
                    )
                    .append_child(
                        Button::new("Menu with popover", ButtonStyle::Filled)
                            .popover({
                                let mut popover = Popover::new();
                                popover.placement(Placement::Bottom);
                                popover.append_child(
                                        Menu::new(MenuStyle::Vertical)
                                            .append_child(
                                                MenuItem::new("Open popup")
                                                    .popup({
                                                        let mut popup = Popup::new();
                                                        popup.append_child(Text::new("Hello", TextStyle::H1));
                                                        popup
                                                    })
                                            )
                                    );
                                popover
                            })
                    )
            )
        )
        .append_child(
            showcase_section("Avatar",
                VStack::new(Alignment::Stretch)
                    .gap(vec![16])
                    .append_child(
                        Avatar::new("Rémi Caillot", &None)
                    )
                    .append_child(
                        Avatar::new("Rémi Caillot", &Some("https://lh3.googleusercontent.com/U8h85c4x-LmiRSLFoT_zmfbdboYMc3sDycMWW-7_8P2yyyC8ZqW_K9okYg1HlG62PodD0ftCw4_6qk_nGxuOrh5u444crsEIQYJpbCH_wubucJn6DZQoo4Wl1sfOqwFBqkCL61FLaCYgcmniQ8QfuwmO-oByPNahjUxTfiIR-xnbgfhVKV4m-XImAQyJAMEdAC9pM6AamFqUSu5I0MxUFiQhOOSZffRNIN0q54QY5mYqXzUCZ5IJDkvRzp-6daoR6BgCZ07zWPiLS4VAYzTLCiTLFUyZAiYQ-hWt-FudOIgqTI8MT9wEGu4MtmJYpX6ST1_fr-lM_dxMty-UTRC6AorCmKDQOygF5X5bgcagFCGXZcjVPQVbQTE8vAMYtY1z3quXMIcgISOT8qaUG81mFJ1jX3Quc_yUPre-CHqvR_h3xuwyRYvYKjcR2METom2YYZvcJlWJcIkpm2NsTgfwTthqRk_Ve8TGW6FPvCVHEHkbjO61xVhWFVyWq-kiERuyKhJfkmNwjoQuRQy5zPjlS0OrrztUyEC91aEc1kcB4OxIyuHCHpU1oovnj20Xg004uNlm26BWbwm5OurIveffVOt4McpY30G4uPU-oXd5Y7uzC0L5WTbSEIo21wSf62MQOkASAViXvnpiTf2p_7vhUDY7B2AlxH4dBsn6gvCJ3_HVhog_ckPqQ2AA2-NFCU_B_eK0su_C31qSHi370OfmXOkTzj9F-bOTQPnNFrMblQUWgAUxJ1xmKlpUoejrhKoGE_8Q_WN7G8V5-PvV=w506-h912-no?authuser=0".to_string()))
                    )
                    .append_child(
                        HStack::new(Alignment::Center)
                            .append_child(
                                Avatar::new("Rémi Caillot", &None)
                                    .size(Size::XLarge)
                            )
                            .append_child(
                                Avatar::new("Rémi Caillot", &None)
                                    .size(Size::Large)
                            )
                            .append_child(
                                Avatar::new("Rémi Caillot", &None)
                                    .size(Size::Normal)
                            )
                    )
            )
        )
        .append_child(
            showcase_section("Tag",
                VStack::new(Alignment::Stretch)
                    .gap(vec![16])
                    .append_child(
                        Tag::new("label")
                    )
                    .append_child(
                        Tag::new("label")
                            .destructive()
                )
                    .append_child(
                        Tag::new("label")
                            .icon(Lucide::Box)
                    )
                    .append_child(
                        Tag::new("label")
                            .icon(Lucide::Box)
                            .success()
                    )
                    .append_child(
                        Tag::new("label")
                            .icon(Lucide::Box)
                            .warning()
                    )
                    .append_child(
                        Tag::new("label")
                            .icon(Lucide::Box)
                            .destructive()
                    )
                    .append_child(
                        Tag::new("label")
                            .icon(Lucide::Box)
                            .badge(&12)
                    )
            )
        )
        .append_child(
            showcase_section("Popup",
                VStack::new(Alignment::Stretch)
                    .gap(vec![16])
                    .append_child(
                        Button::new("Open popup", ButtonStyle::Filled)
                            .popup({
                                let mut popup = Popup::new();
                                popup.append_child(
                                    Text::new("Hello", TextStyle::Body)
                                );
                                popup
                            })
                    )
                    .append_child(
                        Button::new("Nested popup", ButtonStyle::Filled)
                            .popup({
                                let mut popup = Popup::new();
                                popup.append_child(
                                    Button::new("open sub popup", ButtonStyle::Filled)
                                        .popup({
                                            let mut popup = Popup::new();
                                            popup.append_child(
                                                Text::new("Hello", TextStyle::Body)
                                            );
                                            popup
                                        })
                                );
                                popup
                            })
                    )
                    .append_child(
                        Form::new("submit_on_popup_opening", "")
                            .set_attr("method", "GET")
                    )
                    .append_child(
                        Button::new("Submit form on popup opening", ButtonStyle::Filled)
                            .popup({
                                let mut popup = Popup::new();
                                popup.on_open_submit_form("submit_on_popup_opening");
                                popup.append_child(
                                    Text::new("Hello", TextStyle::H1)
                                );
                                popup
                            })
                    )
                    .append_child(
                        Button::new("Submit form on popup closing", ButtonStyle::Filled)
                            .popup({
                                let mut popup = Popup::new();
                                popup.on_close_submit_form("submit_on_popup_closing");
                                popup.append_child(
                                    Text::new("Hello", TextStyle::H2)
                                );
                                popup
                            })
                    )
            )
        )
        .append_child(
            showcase_section("Color picker",
                VStack::new(Alignment::Stretch)
                    .gap(vec![16])
                    .append_child(
                        ColorPicker::new("color-picker", ColorPickerStyle::Palette(vec![
                            Color::from_hex("#131b23"),
                            Color::from_hex("#1d5dea"),
                            Color::from_hex("#ff674d"),
                            Color::from_hex("#fba7de"),
                            Color::from_hex("#fcba04"),
                            Color::from_hex("#d4cdf4"),
                            Color::from_hex("#06d6a0"),
                            Color::from_hex("#58a4b0"),
                            Color::from_hex("#e0cba8"),
                            Color::from_hex("#a23b72"),
                        ]))
                            .label("Choisir une couleur parmi la liste")
                    )
                    .append_child(
                        ColorPicker::new("color-picker", ColorPickerStyle::Free(Some(Color::from_hex("#06d6a0"))))
                            .label("Choisir une couleur libre")
                    )
            )
        );
    stack
}

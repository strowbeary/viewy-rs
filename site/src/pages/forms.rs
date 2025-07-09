use chrono::Duration;
use viewy::components::icons::Lucide;
use viewy::components::*;
use viewy::{scale, DefaultModifiers};

use crate::components::showcase_section;



pub fn forms(richtext: Option<String>) -> VStack {
    VStack::new(Alignment::Stretch)
        .padding(vec![scale(4)])
        .gap(vec![scale(4)])
        .append_child({
            showcase_section("Main search bar", {
                Field::new("q", FieldType::MainSearchBar)
            })
        })
        .append_child({
            showcase_section("Input for durations", {
                Field::new("duration", FieldType::Duration(vec![
                    Duration::days(92),
                    Duration::minutes(43801),
                    Duration::days(15),
                    Duration::weeks(1),
                ]))
            })
        })
        .append_child({
            showcase_section("Multiple file input", {
                VStack::new(Alignment::Stretch)
                    .gap(vec![scale(4)])
                    .append_child({
                        Form::new("mfile-hidden-form", "/upload-file")
                            .set_attr("method", "POST")
                            .append_child({ Text::new("Hidden type", TextStyle::Headline) })
                            .append_child({
                                MultipleFileInput::new("mfile-hidden", FileInputType::Hidden)

                            })
                            .append_child({
                                Button::new("Select files", ButtonStyle::Filled)
                                    .icon(Lucide::Files)
                                    .attach_to_file_input("mfile-hidden")
                            })
                    })
                    .append_child({
                        Form::new("mfile-simple-form", "/upload-file")
                            .append_child({ Text::new("Simple type", TextStyle::Headline) })
                            .append_child({
                                MultipleFileInput::new("mfile-simple", FileInputType::Simple)
                            })
                    })
            })
        })
        .append_child({
            showcase_section("Multiple value text field", {
                VStack::new(Alignment::Stretch)
                    .gap(vec![scale(4)])
                    .append_child({
                       Field::new("multivalue", FieldType::Email)
                           .label("Adresses de courriel")
                           .multiple_value(vec![
                               "dev@remicaillot.fr".to_string(),
                           ])
                    })
                    .append_child({
                      Button::new("Open popup", ButtonStyle::Filled)
                          .popup(Popup::new().append_child({
                              Field::new("multivalue", FieldType::Email)
                                  .label("Adresses de courriel")
                                  .multiple_value(vec![
                                      "dev@remicaillot.fr".to_string(),
                                  ])
                          }))
                    })
            })
        })
        .append_child({
            VStack::new(Alignment::Stretch)
                .gap(vec![scale(2)])
                .append_child({ Text::new("Submit on keypress in Field component", TextStyle::H1) })
                .append_child({
                    Form::new("async-auto-submit-form", "")
                        .async_form()
                        .append_child({
                            Field::new("submit-on-keypress", FieldType::Text)
                                .label("Submit on keypress")
                                .submit_on_keypress()
                        })
                })
        })
        .append_child({
            VStack::new(Alignment::Stretch)
                .gap(vec![scale(2)])
                .append_child({ Text::new("Auto submit picker", TextStyle::H1) })
                .append_child({
                    Form::new("async-auto-submit-form", "")
                        .async_form()
                        .append_child({
                            let mut picker = Picker::new("auto-submit-picker", "2", PickerStyle::Segmented)
                                .submit_on_change(true);
                            picker.append_child({ PickerOption::new("One", "1") });
                            picker.append_child({ PickerOption::new("Two", "2") });
                            picker.append_child({ PickerOption::new("Three", "3") });
                            picker
                        })
                })
                .append_child({
                    Form::new("async-auto-submit-dropdown", "")
                        .async_form()
                        .append_child({
                            let mut picker = Picker::new("auto-submit-dropdown", "2", PickerStyle::Dropdown)
                                .submit_on_change(true);
                            picker.append_child({ PickerOption::new("One", "1") });
                            picker.append_child({ PickerOption::new("Two", "2") });
                            picker.append_child({ PickerOption::new("Three", "3") });
                            picker
                        })
                })
                .append_child({
                    Form::new("async-auto-submit-radio", "")
                        .async_form()
                        .append_child({
                            let mut picker = Picker::new("auto-submit-radio", "2", PickerStyle::RadioGroup)
                                .submit_on_change(true);
                            picker.append_child({ PickerOption::new("One", "1") });
                            picker.append_child({ PickerOption::new("Two", "2") });
                            picker.append_child({ PickerOption::new("Three", "3") });
                            picker
                        })
                })
        })
        .append_child({
            VStack::new(Alignment::Stretch)
                .gap(vec![scale(2)])
                .append_child({ Text::new("Auto submit checkbox", TextStyle::H1) })
                .append_child({
                    Form::new("async-auto-submit-form", "")
                        .async_form()
                        .append_child({
                            Checkbox::new("auto-submit-checkbox", "hello", CheckboxStyle::Checkbox)
                                .label("Checkbox à soumission automatique")
                                .auto_submit(true)
                        })
                })
        })
        .append_child({
            VStack::new(Alignment::Stretch)
                .gap(vec![scale(2)])
                .append_child({ Text::new("File input", TextStyle::H1) })
                .append_child({
                    Form::new("async-auto-submit-form", "").append_child({
                        FileInput::new("test", FileInputType::Simple)
                            .label("Label")
                            .error_message("Error message")
                    })
                })
        })
        .append_child({
            VStack::new(Alignment::Stretch)
                .gap(vec![scale(2)])
                .append_child({ Text::new("Image file input", TextStyle::H1) })
                .append_child({
                    Form::new("async-auto-submit-form", "").append_child({
                        FileInput::new("test2", FileInputType::Image)
                            .accept("image/*")
                            .error_message("Error message")
                    })
                })
        })
        .append_child({
            VStack::new(Alignment::Stretch)
                .gap(vec![scale(2)])
                .append_child({ Text::new("Rich text field", TextStyle::H1) })
                .append_child({
                    if let Some(content) = richtext {
                        Text::new(&content, TextStyle::Body)
                            .sanitization_level(SanitizationLevel::Basic)
                    } else {
                        Text::new("Soumettez le formulaire pour voir l'aperçu", TextStyle::Body)
                    }
                })
                .append_child({
                    Form::new("richtext-form", "")
                        .multipart()
                        .append_child({
                            Field::new("richtext", FieldType::RichTextArea)
                                .label("Éditeur de texte riche")
                                .required(true)
                                .value("Hello")
                        })
                        .append_child({
                            Button::new("Enregistrer", ButtonStyle::Filled)
                                .attach_to_form("richtext-form")
                        })
                })
        })
}

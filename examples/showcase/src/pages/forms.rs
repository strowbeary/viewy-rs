use viewy::components::*;
use viewy::{DefaultModifiers, sp, scale};

pub fn forms() -> VStack {
   VStack::new(Alignment::Stretch)
       .gap(vec![scale(4)])
       .append_child({
           VStack::new(Alignment::Stretch)
               .gap(vec![scale(2)])
               .append_child({
                   Text::new("Auto submit picker", TextStyle::H1)
               })
               .append_child({
                   Form::new("async-auto-submit-form", "")
                       .async_form()
                       .append_child({
                           Picker::new("auto-submit-picker", "2", PickerStyle::Segmented)
                               .submit_on_change(true)
                               .append_child({
                                   PickerOption::new("One", "1")
                               })
                               .append_child({
                                   PickerOption::new("Two", "2")
                               })
                               .append_child({
                                   PickerOption::new("Three", "3")
                               })
                       })
               })
       })
       .append_child({
           VStack::new(Alignment::Stretch)
               .gap(vec![scale(2)])
               .append_child({
                   Text::new("Auto submit checkbox", TextStyle::H1)
               })
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
               .append_child({
                   Text::new("File input", TextStyle::H1)
               })
               .append_child({
                   Form::new("async-auto-submit-form", "")
                       .append_child({
                           FileInput::new("test", FileInputType::Simple)
                               .label("Label")
                               .error_message("Error message")
                       })
               })
       })
       .append_child({
           VStack::new(Alignment::Stretch)
               .gap(vec![scale(2)])
               .append_child({
                   Text::new("Image file input", TextStyle::H1)
               })
               .append_child({
                   Form::new("async-auto-submit-form", "")
                       .append_child({
                           FileInput::new("test2", FileInputType::Image)
                               .accept("image/*")
                               .error_message("Error message")
                       })
               })
       })
       .append_child({
           VStack::new(Alignment::Stretch)
               .gap(vec![scale(2)])
               .append_child({
                   Text::new("Rich text field", TextStyle::H1)
               })
               .append_child({
                   Form::new("richtext-form", "")
                       .append_child({
                          TextField::new("richtext", FieldType::RichTextArea)
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
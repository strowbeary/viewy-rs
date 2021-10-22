use viewy::components::*;
use viewy::{DefaultModifiers, sp, scale};

pub fn forms() -> VStack {
   VStack::new(Alignment::Stretch)
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
}
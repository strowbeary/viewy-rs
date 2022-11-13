use viewy::components::{Alignment, Appendable, Button, ButtonStyle, DynamicContent, FieldType, Form, HStack, Field, VStack};
use viewy::{DefaultModifiers, scale};
use viewy::components::icons::Lucide;

pub fn dynamic_content() -> VStack {
	VStack::new(Alignment::Stretch)
		.append_child({
			Form::new("search", "/search")
				.async_form()
				.inject_into_dynamic_content("search_result")
				.append_child({
					HStack::new(Alignment::Center)
						.gap(vec![scale(2)])
						.append_child({
							Field::new("q", FieldType::Search)
								//.submit_on_keypress()
						})
						.append_child({
							Button::icon_only(Lucide::Search, ButtonStyle::Filled)
								.attach_to_form("search")
						})
				})
		})
		.append_child({
			DynamicContent::new("search_result")
		})
}
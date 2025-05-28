use viewy::components::*;

pub fn searchable_input_page() -> VStack {
    let mut stack = VStack::new(Alignment::Stretch);
    stack.append_child(
        Field::new("searchable", FieldType::Search)
            .async_datalist("https://www.omdbapi.com/?apikey=31793868&s=")
            .placeholder("Chercher dans la base imdb")
    );
    stack
}
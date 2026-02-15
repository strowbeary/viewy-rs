use rocket::serde::{Deserialize, Serialize};
use std::ops::Add;
use viewy::prelude::*;

use crate::core::adapters::book_library_adapter::BookLibraryAdapter;
use crate::core::models::book::Book;
use crate::core::ports::book_library_port::BookLibraryPort;

#[derive(Debug, Clone, Serialize, Deserialize, InteractiveComponentMessage)]
#[serde(crate = "rocket::serde")]
pub enum PaginationMessage {
    NextPage,
    PreviousPage,
}

#[derive(Debug, Clone, Serialize, Deserialize, InteractiveComponent)]
#[serde(crate = "rocket::serde")]
#[component(messages = PaginationMessage)]
pub struct BookListPaginated {
    pub page_idx: usize,
    pub displayed_books: Vec<Book>,
}

impl viewy::prelude::InteractiveComponent for BookListPaginated {
    type Message = PaginationMessage;

    fn on_message(mut self, message: Self::Message) -> Self {
        let library = BookLibraryAdapter::new();
        match message {
            PaginationMessage::NextPage => self.page_idx += 1,
            PaginationMessage::PreviousPage => {
                if self.page_idx > 0 {
                    self.page_idx -= 1;
                }
            }
        }
        self.displayed_books = library.list_books_paginated(self.page_idx, 10);
        self
    }

    fn render(self) -> Node {
        VStack::new(Alignment::Stretch)
            .add_class("counter-component")
            .gap(vec![scale(3)])
            .append_child({
                let mut book_list = VStack::new(Alignment::Stretch);
                for book in self.displayed_books {
                    book_list.append_child(Text::new(&book.title, TextStyle::H1));
                }
                book_list
            })
            .append_child(
                HStack::new(Alignment::Center)
                    .gap(vec![scale(2)])
                    .append_child(
                        Button::new("Prev", ButtonStyle::Outlined)
                            .on_click(Action::TriggerMessage(PaginationMessage::PreviousPage)),
                    )
                    .append_child(Text::new(&self.page_idx.add(1).to_string(), TextStyle::H2))
                    .append_child(
                        Button::new("Next", ButtonStyle::Outlined)
                            .on_click(Action::TriggerMessage(PaginationMessage::NextPage)),
                    ),
            )
            .into()
    }
}

#[get("/interactive-component-poc")]
pub fn interactive_component_demo() -> Page<'static> {
    let library = BookLibraryAdapter::new();
    Page::with_title("Viewy showcase – Interactive Component PoC").with_content({
        let mut main_stack = VStack::new(Alignment::Stretch);
        main_stack
            .gap(vec![scale(4)])
            .padding(vec![scale(5)])
            .append_child(Text::new(
                "PoC: composant interactif sans état serveur",
                TextStyle::H1,
            ))
            .append_child(Text::new(
                "Une seule route gère tous les composants interactifs. L'état du composant est transporté en HTML via data-v-component-state.",
                TextStyle::Body,
            ))
            .append_child(BookListPaginated { page_idx: 0, displayed_books: library.list_books_paginated(0, 10) });
        main_stack
    })
}

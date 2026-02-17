use std::collections::HashMap;

use crate::core::{
    models::book::Book, ports::book_library_port::BookLibraryPort, ports::book_library_port::Error,
};

pub struct BookLibraryAdapter {
    books: HashMap<String, Book>,
}

impl BookLibraryAdapter {
    pub fn new() -> Self {
        let mut library = BookLibraryAdapter {
            books: HashMap::new(),
        };
        let seed_books = vec![
            Book {
                id: String::from("book-001"),
                title: String::from("To Kill a Mockingbird"),
                author: String::from("Harper Lee"),
            },
            Book {
                id: String::from("book-002"),
                title: String::from("1984"),
                author: String::from("George Orwell"),
            },
            Book {
                id: String::from("book-003"),
                title: String::from("Pride and Prejudice"),
                author: String::from("Jane Austen"),
            },
            Book {
                id: String::from("book-004"),
                title: String::from("The Great Gatsby"),
                author: String::from("F. Scott Fitzgerald"),
            },
            Book {
                id: String::from("book-005"),
                title: String::from("Moby-Dick"),
                author: String::from("Herman Melville"),
            },
            Book {
                id: String::from("book-006"),
                title: String::from("War and Peace"),
                author: String::from("Leo Tolstoy"),
            },
            Book {
                id: String::from("book-007"),
                title: String::from("The Catcher in the Rye"),
                author: String::from("J.D. Salinger"),
            },
            Book {
                id: String::from("book-008"),
                title: String::from("The Hobbit"),
                author: String::from("J.R.R. Tolkien"),
            },
            Book {
                id: String::from("book-009"),
                title: String::from("Brave New World"),
                author: String::from("Aldous Huxley"),
            },
            Book {
                id: String::from("book-010"),
                title: String::from("Crime and Punishment"),
                author: String::from("Fyodor Dostoevsky"),
            },
            Book {
                id: String::from("book-011"),
                title: String::from("The Brothers Karamazov"),
                author: String::from("Fyodor Dostoevsky"),
            },
            Book {
                id: String::from("book-012"),
                title: String::from("Jane Eyre"),
                author: String::from("Charlotte Bronte"),
            },
            Book {
                id: String::from("book-013"),
                title: String::from("Wuthering Heights"),
                author: String::from("Emily Bronte"),
            },
            Book {
                id: String::from("book-014"),
                title: String::from("The Grapes of Wrath"),
                author: String::from("John Steinbeck"),
            },
            Book {
                id: String::from("book-015"),
                title: String::from("Of Mice and Men"),
                author: String::from("John Steinbeck"),
            },
            Book {
                id: String::from("book-016"),
                title: String::from("One Hundred Years of Solitude"),
                author: String::from("Gabriel Garcia Marquez"),
            },
            Book {
                id: String::from("book-017"),
                title: String::from("The Name of the Rose"),
                author: String::from("Umberto Eco"),
            },
            Book {
                id: String::from("book-018"),
                title: String::from("The Alchemist"),
                author: String::from("Paulo Coelho"),
            },
            Book {
                id: String::from("book-019"),
                title: String::from("Sapiens"),
                author: String::from("Yuval Noah Harari"),
            },
            Book {
                id: String::from("book-020"),
                title: String::from("The Road"),
                author: String::from("Cormac McCarthy"),
            },
            Book {
                id: String::from("book-021"),
                title: String::from("Dune"),
                author: String::from("Frank Herbert"),
            },
            Book {
                id: String::from("book-022"),
                title: String::from("Foundation"),
                author: String::from("Isaac Asimov"),
            },
            Book {
                id: String::from("book-023"),
                title: String::from("Neuromancer"),
                author: String::from("William Gibson"),
            },
            Book {
                id: String::from("book-024"),
                title: String::from("The Left Hand of Darkness"),
                author: String::from("Ursula K. Le Guin"),
            },
            Book {
                id: String::from("book-025"),
                title: String::from("Fahrenheit 451"),
                author: String::from("Ray Bradbury"),
            },
            Book {
                id: String::from("book-026"),
                title: String::from("Slaughterhouse-Five"),
                author: String::from("Kurt Vonnegut"),
            },
            Book {
                id: String::from("book-027"),
                title: String::from("The Handmaid's Tale"),
                author: String::from("Margaret Atwood"),
            },
            Book {
                id: String::from("book-028"),
                title: String::from("Beloved"),
                author: String::from("Toni Morrison"),
            },
            Book {
                id: String::from("book-029"),
                title: String::from("The Color Purple"),
                author: String::from("Alice Walker"),
            },
            Book {
                id: String::from("book-030"),
                title: String::from("The Kite Runner"),
                author: String::from("Khaled Hosseini"),
            },
            Book {
                id: String::from("book-031"),
                title: String::from("A Thousand Splendid Suns"),
                author: String::from("Khaled Hosseini"),
            },
            Book {
                id: String::from("book-032"),
                title: String::from("The Book Thief"),
                author: String::from("Markus Zusak"),
            },
            Book {
                id: String::from("book-033"),
                title: String::from("The Shadow of the Wind"),
                author: String::from("Carlos Ruiz Zafon"),
            },
            Book {
                id: String::from("book-034"),
                title: String::from("Life of Pi"),
                author: String::from("Yann Martel"),
            },
            Book {
                id: String::from("book-035"),
                title: String::from("The Martian"),
                author: String::from("Andy Weir"),
            },
            Book {
                id: String::from("book-036"),
                title: String::from("Project Hail Mary"),
                author: String::from("Andy Weir"),
            },
            Book {
                id: String::from("book-037"),
                title: String::from("The Midnight Library"),
                author: String::from("Matt Haig"),
            },
            Book {
                id: String::from("book-038"),
                title: String::from("Normal People"),
                author: String::from("Sally Rooney"),
            },
            Book {
                id: String::from("book-039"),
                title: String::from("Never Let Me Go"),
                author: String::from("Kazuo Ishiguro"),
            },
            Book {
                id: String::from("book-040"),
                title: String::from("Klara and the Sun"),
                author: String::from("Kazuo Ishiguro"),
            },
            Book {
                id: String::from("book-041"),
                title: String::from("The Goldfinch"),
                author: String::from("Donna Tartt"),
            },
            Book {
                id: String::from("book-042"),
                title: String::from("The Secret History"),
                author: String::from("Donna Tartt"),
            },
            Book {
                id: String::from("book-043"),
                title: String::from("Educated"),
                author: String::from("Tara Westover"),
            },
            Book {
                id: String::from("book-044"),
                title: String::from("Atomic Habits"),
                author: String::from("James Clear"),
            },
            Book {
                id: String::from("book-045"),
                title: String::from("Thinking, Fast and Slow"),
                author: String::from("Daniel Kahneman"),
            },
            Book {
                id: String::from("book-046"),
                title: String::from("Deep Work"),
                author: String::from("Cal Newport"),
            },
            Book {
                id: String::from("book-047"),
                title: String::from("The Pragmatic Programmer"),
                author: String::from("Andrew Hunt, David Thomas"),
            },
            Book {
                id: String::from("book-048"),
                title: String::from("Clean Code"),
                author: String::from("Robert C. Martin"),
            },
            Book {
                id: String::from("book-049"),
                title: String::from("Designing Data-Intensive Applications"),
                author: String::from("Martin Kleppmann"),
            },
            Book {
                id: String::from("book-050"),
                title: String::from("The Phoenix Project 1"),
                author: String::from("Gene Kim, Kevin Behr, George Spafford"),
            },
            Book {
                id: String::from("book-051"),
                title: String::from("The Phoenix Project 2"),
                author: String::from("Gene Kim, Kevin Behr, George Spafford"),
            },
            Book {
                id: String::from("book-052"),
                title: String::from("The Phoenix Project 3"),
                author: String::from("Gene Kim, Kevin Behr, George Spafford"),
            },
            Book {
                id: String::from("book-053"),
                title: String::from("The Phoenix Project 4"),
                author: String::from("Gene Kim, Kevin Behr, George Spafford"),
            },
            Book {
                id: String::from("book-054"),
                title: String::from("The Phoenix Project 5"),
                author: String::from("Gene Kim, Kevin Behr, George Spafford"),
            },
        ];

        for book in seed_books {
            let _ = library.add_book(book);
        }

        library
    }
}

impl BookLibraryPort for BookLibraryAdapter {
    fn add_book(&mut self, book: Book) -> Result<(), Error> {
        // Implementation here
        self.books.insert(book.id.clone(), book);
        Ok(())
    }

    fn remove_book(&mut self, book_id: &str) -> Result<(), Error> {
        // Implementation here
        self.books.remove(book_id);
        Ok(())
    }

    fn get_book(&self, book_id: &str) -> Result<&Book, Error> {
        // Implementation here
        self.books.get(book_id).ok_or(Error::NotFound)
    }

    fn list_books_paginated(&self, page: usize, per_page: usize) -> Vec<Book> {
        // Implementation here
        let mut books = self.books.values().cloned().collect::<Vec<Book>>();
        books.sort_by(|book1, book2| book1.title.cmp(&book2.title));

        books
            .into_iter()
            .skip(page * per_page)
            .take(per_page)
            .collect()
    }
}

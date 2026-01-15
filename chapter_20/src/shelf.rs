use std::collections::HashMap;

#[derive(Debug)]
pub struct Author {
    id: usize,
    name: String,
}

#[derive(Debug)]
pub struct Book {
    author_id: usize,
    title: String,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Connected {
    title: String,
    author: String,
}

pub fn books_with_authors_1(
    books: &[Book],
    authors: &[Author],
) -> Vec<Connected> {
    let mut connected = vec![];

    for book in books {
        for author in authors {
            if book.author_id == author.id {
                connected.push(Connected {
                    title: book.title.clone(),
                    author: author.name.clone(),
                });
            }
        }
    }
    connected
}

pub fn books_with_authors_2(
    books: &[Book],
    authors: &[Author],
) -> Vec<Connected> {
    let mut connected = vec![];
    let mut authors_hash = HashMap::<usize, &str>::new();

    for author in authors {
        authors_hash.insert(author.id, &author.name);
    }

    for book in books {
        let name = authors_hash
            .get(&book.author_id)
            .expect("Author must be set");
        connected.push(Connected {
            title: book.title.clone(),
            author: (*name).to_owned(),
        });
    }
    connected
}

pub(crate) mod sample {
    use super::*;

    pub fn authors() -> Vec<Author> {
        vec![
            Author {
                id: 1,
                name: "Virginia Woolf".into(),
            },
            Author {
                id: 2,
                name: "Leo Tolstoy".into(),
            },
            Author {
                id: 3,
                name: "Dr. Seuss".into(),
            },
            Author {
                id: 4,
                name: "J. K. Rowling".into(),
            },
            Author {
                id: 5,
                name: "Mark Twain".into(),
            },
        ]
    }

    pub fn books() -> Vec<Book> {
        vec![
            Book {
                author_id: 3,
                title: "Hop on Pop".into(),
            },
            Book {
                author_id: 1,
                title: "Mrs. Dalloway".into(),
            },
            Book {
                author_id: 4,
                title: "Harry Potter and the Sorcerer's Stone".into(),
            },
            Book {
                author_id: 1,
                title: "To the Lighthouse".into(),
            },
            Book {
                author_id: 2,
                title: "Anna Karenina".into(),
            },
            Book {
                author_id: 5,
                title: "The Adventures of Tom Sawyer".into(),
            },
            Book {
                author_id: 3,
                title: "The Cat in the Hat".into(),
            },
            Book {
                author_id: 2,
                title: "War and Peace".into(),
            },
            Book {
                author_id: 3,
                title: "Green Eggs and Ham".into(),
            },
            Book {
                author_id: 5,
                title: "The Adventures of Huckleberry Finn".into(),
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_books_with_authors_1() {
        let author = "Virginia Woolf";
        let authors = vec![Author {
            id: 1,
            name: author.into(),
        }];

        let title = "Mrs. Dalloway";
        let books = vec![Book {
            author_id: 1,
            title: title.into(),
        }];

        let connected = books_with_authors_1(&books, &authors);
        let first = connected.first().unwrap();
        assert_eq!(first.title, title);
        assert_eq!(first.author, author);
    }

    #[test]
    fn test_books_with_authors_2() {
        let author = "Virginia Woolf";
        let authors = vec![Author {
            id: 1,
            name: author.into(),
        }];

        let title = "Mrs. Dalloway";
        let books = vec![Book {
            author_id: 1,
            title: title.into(),
        }];

        let connected = books_with_authors_2(&books, &authors);
        let first = connected.first().unwrap();
        assert_eq!(first.title, title);
        assert_eq!(first.author, author);
    }
}

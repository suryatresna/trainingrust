struct Book {
    title: String,
    author: String,
    isbn: String,
    is_available: bool,
}

struct Library {
    books: Vec<Book>,
}

impl Library {
    fn new() -> Self {
        Library { books: Vec::new() }
    }

    fn add_book(&mut self, title: String, author: String, isbn: String) {
        let new_book = Book{
            title, 
            author, 
            isbn, 
            is_available: true
        };

        self.books.push(new_book);
    }

    fn find_by_isbn(&self, isbn: &str) -> Option<&Book> {
       self.books.iter().find(|book| book.isbn == isbn)
    }

    fn checkout(&mut self, isbn: &str) -> Result<String, String> {
        match self.books.iter_mut().find(|book| book.isbn == isbn) {
            Some(book) => {
                if book.is_available {
                    book.is_available = false;
                    Ok(format!("You have checked out \"{}\" ", book.title))
                } else {
                    Err(format!("Sorry, \"{}\" is currently unavailable.", book.title))
                }
            }
            None => Err("Book not found.".to_string())
        }
    }

    fn available_books(&self) -> Vec<&Book> {
        self.books.iter().filter(|book| book.is_available).collect()
    }
}

fn main() {
    let mut library = Library::new();
    
    library.add_book(
        String::from("The Rust Programming Language"),
        String::from("Steve Klabnik"),
        String::from("978-1593278281")
    );
    library.add_book(
        String::from("The Rust Dummies"),
        String::from("Jhon Lark"),
        String::from("978-1112323234")
    );

    println!("Available books firsttime: {}", library.available_books().len());

    let find_book = library.find_by_isbn("978-1112323234");
    match find_book {
        Some(book) => println!("Found book: \"{}\" by {}", book.title, book.author),
        None => println!("Book not found."),
    }
    
    match library.checkout("978-1593278281") {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("Error: {}", e),
    }
    
    println!("Available books: {}", library.available_books().len());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_library() -> Library {
        let mut library = Library::new();
        library.add_book(
            String::from("The test book"),
            String::from("Jhon Lark"),
            String::from("1234"),
        );
        library.add_book(
            String::from("The another test book"),
            String::from("Cena Will"),
            String::from("4321"),
        );
        library
    }

    #[test]
    fn test_new_library_is_empty() {
        let library = Library::new();
        assert_eq!(library.books.len(), 0);
    }
 
    #[test]
    fn test_add_books() {
        let mut library = Library::new();
        library.add_book(
            String::from("Test Book A"),
            String::from("Foo"),
            String::from("1234"),
        );

        assert_eq!(library.books.len(), 1);
    }

    #[test]
    fn test_find_isbn() {
        let library = setup_library();
        let result = library.find_by_isbn("1234");
        assert!(result.is_some());
        assert_eq!(result.unwrap().title, "The test book");
    }
}
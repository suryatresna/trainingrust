# Tutorial 2: Library Management System

## Overview
This tutorial builds on the concepts from Tutorial 1 by introducing multiple interacting structs, the `Result` type for comprehensive error handling, and unit testing. You'll create a library system that manages books and their availability.

## Learning Objectives

By completing this tutorial, you will understand:
- How multiple structs interact with each other
- Using the `Result<T, E>` type for error handling
- Advanced iterator methods (`find`, `filter`, `iter_mut`)
- Working with mutable and immutable references
- Writing and running unit tests in Rust
- Test organization with `#[cfg(test)]` modules

## Concepts Covered

### 1. Multiple Struct Interaction
```rust
struct Book {
    title: String,
    author: String,
    isbn: String,
    is_available: bool,
}

struct Library {
    books: Vec<Book>,
}
```
The `Library` struct manages a collection of `Book` structs, demonstrating composition.

### 2. Result Type for Error Handling
```rust
fn checkout(&mut self, isbn: &str) -> Result<String, String>
```
Unlike `Option`, `Result` provides detailed error information:
- `Ok(value)`: Operation succeeded with a value
- `Err(error)`: Operation failed with an error message

### 3. Finding with Iterators
```rust
fn find_by_isbn(&self, isbn: &str) -> Option<&Book> {
    self.books.iter().find(|book| book.isbn == isbn)
}
```
Uses closures and iterator methods to search through collections efficiently.

### 4. Mutable Iteration
```rust
self.books.iter_mut().find(|book| book.isbn == isbn)
```
`iter_mut()` allows you to modify items while iterating, necessary for checking out books.

### 5. Unit Testing
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_library_is_empty() {
        let library = Library::new();
        assert_eq!(library.books.len(), 0);
    }
}
```
Tests are compiled only when running `cargo test` and verify code correctness.

## Code Structure

The program consists of:

### Book Struct
- `title`: Book title
- `author`: Book author
- `isbn`: Unique identifier
- `is_available`: Checkout status

### Library Struct with Methods
- `new()`: Creates an empty library
- `add_book()`: Adds a new book to the collection
- `find_by_isbn()`: Searches for a book by ISBN
- `checkout()`: Marks a book as unavailable if possible
- `available_books()`: Returns all books currently available

## Running the Tutorial

From the project root:
```bash
# Run the program
cargo run --bin tutor2

# Run the tests
cargo test --bin tutor2
```

Expected output:
```
Available books firsttime: 2
Found book: "The Rust Dummies" by Jhon Lark
You have checked out "The Rust Programming Language"
Available books: 1
```

Test output:
```
running 3 tests
test tests::test_new_library_is_empty ... ok
test tests::test_add_books ... ok
test tests::test_find_isbn ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Understanding the Error Handling

The `checkout` method demonstrates comprehensive error handling:

```rust
fn checkout(&mut self, isbn: &str) -> Result<String, String> {
    match self.books.iter_mut().find(|book| book.isbn == isbn) {
        Some(book) => {
            if book.is_available {
                book.is_available = false;
                Ok(format!("You have checked out \"{}\"", book.title))
            } else {
                Err(format!("Sorry, \"{}\" is currently unavailable.", book.title))
            }
        }
        None => Err("Book not found.".to_string())
    }
}
```

Three possible outcomes:
1. Book found and available → `Ok` with success message
2. Book found but unavailable → `Err` with unavailable message
3. Book not found → `Err` with not found message

## Testing Strategy

The test suite demonstrates key testing concepts:

### 1. Test Setup Function
```rust
fn setup_library() -> Library {
    let mut library = Library::new();
    library.add_book(...);
    library
}
```
Creates a reusable test fixture to avoid code duplication.

### 2. Testing Empty State
```rust
#[test]
fn test_new_library_is_empty() {
    let library = Library::new();
    assert_eq!(library.books.len(), 0);
}
```

### 3. Testing Additions
```rust
#[test]
fn test_add_books() {
    let mut library = Library::new();
    library.add_book(...);
    assert_eq!(library.books.len(), 1);
}
```

### 4. Testing Search Functionality
```rust
#[test]
fn test_find_isbn() {
    let library = setup_library();
    let result = library.find_by_isbn("1234");
    assert!(result.is_some());
    assert_eq!(result.unwrap().title, "The test book");
}
```

## Exercises

Try extending the code with:

1. **Return book functionality**: Add a `return_book()` method that sets `is_available` back to `true`
2. **Test the checkout method**: Write tests for the `checkout()` method covering all three cases
3. **Search by title/author**: Add methods to find books by title or author (partial matches)
4. **Book count statistics**: Add methods to count total books and available books
5. **Due date tracking**: Add a `due_date` field to track when books should be returned
6. **Borrower tracking**: Add a `borrower: Option<String>` field to track who checked out a book

## Key Takeaways

- `Result<T, E>` is the standard way to handle operations that can fail
- `match` expressions let you handle all possible outcomes explicitly
- `iter_mut()` is necessary when you need to modify collection items
- Unit tests provide confidence that code works as expected
- Test modules are conditionally compiled with `#[cfg(test)]`
- Helper functions reduce duplication in tests

## Common Patterns

### Pattern Matching with Result
```rust
match library.checkout("isbn") {
    Ok(msg) => println!("{}", msg),
    Err(e) => println!("Error: {}", e),
}
```

### Filtering Collections
```rust
fn available_books(&self) -> Vec<&Book> {
    self.books.iter().filter(|book| book.is_available).collect()
}
```

## Next Steps

Once comfortable with this tutorial, proceed to [Tutorial 3: Shopping Cart System](../tutor3/README.md) to learn about:
- More complex struct relationships
- Advanced error handling patterns
- Quantity and state management
- Mathematical operations with floating-point numbers

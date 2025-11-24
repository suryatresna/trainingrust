# Rust Training Project

A comprehensive Rust training project containing hands-on tutorials covering fundamental to intermediate Rust programming concepts. This project demonstrates practical applications of Rust's core features including structs, enums, error handling, ownership, and testing.

## 📚 Project Overview

This training project consists of four progressive tutorials, each focusing on different aspects of Rust programming:

1. **Tutorial 1**: Student Grade Management System
2. **Tutorial 2**: Library Management System
3. **Tutorial 3**: Shopping Cart System
4. **Tutorial 4**: Task Management System (Exercise)

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (Edition 2024)
- Cargo (comes with Rust installation)

### Installation

1. Clone this repository:
   ```bash
   git clone https://github.com/suryatresna/trainingrust.git
   cd trainingrust
   ```

2. Build all tutorials:
   ```bash
   cargo build
   ```

3. Run a specific tutorial:
   ```bash
   cargo run --bin tutor1  # Student Grade System
   cargo run --bin tutor2  # Library Management
   cargo run --bin tutor3  # Shopping Cart
   cargo run --bin tutor4  # Task Management
   ```

### Running Tests

Execute the test suite:
```bash
cargo test
```

Run tests for a specific tutorial:
```bash
cargo test --bin tutor2  # Library Management tests
cargo test --bin tutor3  # Shopping Cart tests
```

## 📖 Tutorials

### Tutorial 1: Student Grade Management System
**File**: `src/tutor1/main.rs`

**Concepts Covered**:
- Structs and methods
- Vec collections
- Option type
- Basic error handling
- Mathematical calculations

**Features**:
- Create student records
- Add grades with validation
- Calculate grade averages
- Generate letter grades (A-F)

### Tutorial 2: Library Management System
**File**: `src/tutor2/main.rs`

**Concepts Covered**:
- Multiple structs interaction
- Result type for error handling
- Iterator methods (`find`, `filter`)
- Mutable references
- Unit testing with `#[cfg(test)]`

**Features**:
- Add books to library
- Search books by ISBN
- Check out books
- Track availability
- Comprehensive test suite

### Tutorial 3: Shopping Cart System
**File**: `src/tutor3/main.rs`
**Requirements**: `src/tutor3/README.md`

**Concepts Covered**:
- Complex struct relationships
- Advanced error handling patterns
- Vector manipulation
- Mathematical operations
- State management

**Features**:
- Add/remove products
- Quantity management
- Discount calculations
- Subtotal and total calculations
- Item counting

### Tutorial 4: Task Management System (Exercise)
**File**: `src/tutor4/main.rs`
**Requirements**: `src/tutor4/README.md`

**Concepts to Practice**:
- Enums with variants
- Pattern matching
- Auto-incrementing IDs
- Status tracking
- Priority management

**Exercise Goals**:
- Implement Priority and Status enums
- Create Task and TaskManager structs
- Build CRUD operations
- Add filtering and counting methods

## 🛠️ Project Structure

```
trainingrust/
├── Cargo.toml          # Project configuration
├── README.md           # This file
├── src/
│   ├── main.rs         # Simple "Hello, world!" entry point
│   ├── tutor1/
│   │   └── main.rs     # Student Grade System
│   ├── tutor2/
│   │   └── main.rs     # Library Management (with tests)
│   ├── tutor3/
│   │   ├── main.rs     # Shopping Cart System
│   │   └── README.md   # Detailed requirements
│   └── tutor4/
│       ├── main.rs     # Task Management (exercise)
│       └── README.md   # Detailed requirements
└── target/             # Build artifacts (auto-generated)
```

## 🎯 Learning Objectives

By completing these tutorials, you will gain practical experience with:

- **Rust Fundamentals**: Ownership, borrowing, and lifetimes
- **Data Structures**: Structs, enums, and collections
- **Error Handling**: `Option`, `Result`, and custom error types
- **Memory Safety**: Rust's ownership system in practice
- **Testing**: Writing and running unit tests
- **Pattern Matching**: Using `match` and `if let` expressions
- **Iterator Patterns**: Functional programming concepts
- **Project Organization**: Multi-binary Cargo projects

## 💡 Tips for Learning

1. **Start Sequential**: Begin with Tutorial 1 and progress in order
2. **Read the Code**: Understand each line before running
3. **Experiment**: Modify the code and observe the results
4. **Run Tests**: Use tests to verify your understanding
5. **Complete Exercise**: Attempt Tutorial 4 independently
6. **Review Documentation**: Reference the Rust Book when needed

## 📝 Additional Resources

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Standard Library Documentation](https://doc.rust-lang.org/std/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)

## 🤝 Contributing

This is a training project, but suggestions for improvements are welcome:

1. Fork the repository
2. Create a feature branch
3. Submit a pull request with your improvements

## 📄 License

This project is created for educational purposes. Feel free to use and modify for your learning journey.

---

**Happy Rust Learning! 🦀**
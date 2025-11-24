# Tutorial 1: Student Grade Management System

## Overview
This tutorial introduces fundamental Rust concepts through a simple student grade tracking system. You'll learn how to work with structs, methods, collections, and basic data validation.

## Learning Objectives

By completing this tutorial, you will understand:
- How to define and use structs in Rust
- How to implement methods on structs using `impl` blocks
- Working with `Vec<T>` collections
- Using the `Option<T>` type for optional values
- Basic input validation
- Iterator methods and functional programming patterns

## Concepts Covered

### 1. Structs
```rust
struct Student {
    name: String,
    grades: Vec<f64>,
}
```
Structs allow you to group related data together. Here, each student has a name and a collection of grades.

### 2. Constructor Pattern
```rust
fn new(name: String) -> Self {
    Student {
        name,
        grades: Vec::new(),
    }
}
```
The `new` associated function is a common Rust pattern for creating instances with default values.

### 3. Data Validation
```rust
fn add_grade(&mut self, grade: f64) {
    if grade >= 0.0 && grade <= 100.0 {
        self.grades.push(grade);
    }
}
```
Validates that grades are within the acceptable range (0-100) before adding them.

### 4. Iterator Methods
```rust
let sum: f64 = self.grades.iter().sum();
```
Uses iterator methods to efficiently calculate the sum of all grades.

### 5. Option Type
```rust
fn get_letter_grade(&self) -> Option<char>
```
Returns `Some(grade)` if a grade can be calculated, or `None` if not.

## Code Structure

The program consists of:
- **Student struct**: Holds student data (name and grades)
- **Methods**:
  - `new()`: Creates a new student
  - `add_grade()`: Adds a validated grade
  - `average()`: Calculates the average of all grades
  - `get_letter_grade()`: Converts numeric average to letter grade (A-F)

## Running the Tutorial

From the project root:
```bash
cargo run --bin tutor1
```

Expected output:
```
Student: Alice
Average Grade: 85.00
Letter Grade: Some('B')
```

## Exercises

Try modifying the code to:

1. **Add grade counting**: Add a method `grade_count()` that returns the number of grades
2. **Find highest/lowest**: Implement methods to find the highest and lowest grades
3. **GPA calculation**: Modify `get_letter_grade()` to return grade points (A=4.0, B=3.0, etc.)
4. **Input validation enhancement**: Reject negative quantities or grades over 100
5. **Display improvements**: Format the output differently, perhaps showing all grades

## Key Takeaways

- Structs organize related data into meaningful units
- Methods with `&mut self` can modify the struct's data
- The `Option` type safely handles cases where a value might not exist
- Iterator methods provide a functional approach to working with collections
- Data validation should happen as early as possible

## Next Steps

Once comfortable with this tutorial, proceed to [Tutorial 2: Library Management System](../tutor2/README.md) to learn about:
- Multiple struct interactions
- The `Result` type for error handling
- More advanced iterator patterns
- Unit testing with `#[cfg(test)]`

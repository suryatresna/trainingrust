# Employee Management System

A multi-module Rust project to practice Rust's module system, file organization, and project structure.

## Overview

Build an employee management system split across multiple files, demonstrating proper separation of concerns and module usage.

## Project Structure

```
src/
├── main.rs          # Main entry point with Company struct
├── employee.rs      # Employee struct and methods
├── department.rs    # Department struct and methods
└── salary.rs        # Salary calculation functions
```

---

## Module Specifications

### 1. `employee.rs`

**Employee Struct:**
```rust
pub struct Employee {
    id: u32,
    name: String,
    email: String,
    department_id: u32,
    base_salary: f64,
}
```

**Required Methods:**
- `new(id: u32, name: String, email: String, department_id: u32, base_salary: f64) -> Result<Self, String>`
  - Validate email contains "@"
  - Validate base_salary > 0.0
- `get_id(&self) -> u32`
- `get_department_id(&self) -> u32`
- `get_base_salary(&self) -> f64`
- `update_salary(&mut self, new_salary: f64) -> Result<(), String>`

---

### 2. `department.rs`

**Department Struct:**
```rust
pub struct Department {
    id: u32,
    name: String,
    budget: f64,
}
```

**Required Methods:**
- `new(id: u32, name: String, budget: f64) -> Self`
- `get_id(&self) -> u32`
- `get_name(&self) -> &str`
- `can_afford(&self, amount: f64) -> bool`

---

### 3. `salary.rs`

**Salary Calculation Functions:**

- `calculate_bonus(base_salary: f64, performance_rating: f64) -> Result<f64, String>`
  - Formula: `base_salary * (performance_rating / 10.0)`
  - Validate rating is between 0.0 and 10.0

- `calculate_annual_salary(base_salary: f64) -> f64`
  - Formula: `base_salary * 12`

- `calculate_tax(salary: f64) -> f64`
  - Tax brackets:
    - 0-50,000: 10%
    - 50,001-100,000: 20%
    - 100,001+: 30%

---

### 4. `main.rs`

**Company Struct:**
```rust
struct Company {
    employees: Vec<Employee>,
    departments: Vec<Department>,
}
```

**Required Methods:**
- `new() -> Self`
- `add_employee(&mut self, employee: Employee) -> Result<(), String>`
  - Check if department exists before adding
- `add_department(&mut self, department: Department)`
- `find_employee(&self, id: u32) -> Option<&Employee>`
- `get_employees_by_department(&self, dept_id: u32) -> Vec<&Employee>`
- `calculate_department_payroll(&self, dept_id: u32) -> f64`
  - Sum of all base salaries in department
- `give_raise(&mut self, employee_id: u32, percentage: f64) -> Result<(), String>`
  - Increase employee salary by percentage

---

## Example Usage

```rust
mod employee;
mod department;
mod salary;

use employee::Employee;
use department::Department;
use salary::{calculate_bonus, calculate_annual_salary};

fn main() {
    let mut company = Company::new();

    // Add departments
    let eng_dept = Department::new(1, String::from("Engineering"), 500000.0);
    let sales_dept = Department::new(2, String::from("Sales"), 300000.0);
    company.add_department(eng_dept);
    company.add_department(sales_dept);

    // Add employees
    let emp1 = Employee::new(
        101,
        String::from("Alice"),
        String::from("alice@company.com"),
        1,
        5000.0
    ).unwrap();

    let emp2 = Employee::new(
        102,
        String::from("Bob"),
        String::from("bob@company.com"),
        1,
        6000.0
    ).unwrap();

    company.add_employee(emp1).unwrap();
    company.add_employee(emp2).unwrap();

    // Calculate payroll
    let eng_payroll = company.calculate_department_payroll(1);
    println!("Engineering payroll: ${:.2}", eng_payroll);

    // Give raise
    company.give_raise(101, 10.0).unwrap();

    // Calculate bonus
    let bonus = calculate_bonus(5000.0, 8.5).unwrap();
    println!("Bonus: ${:.2}", bonus);
}
```

---

## Starter Code

### `employee.rs`
```rust
pub struct Employee {
    // Your fields here
}

impl Employee {
    // Your methods here
}
```

### `department.rs`
```rust
pub struct Department {
    // Your fields here
}

impl Department {
    // Your methods here
}
```

### `salary.rs`
```rust
pub fn calculate_bonus(base_salary: f64, performance_rating: f64) -> Result<f64, String> {
    // Your implementation here
}

pub fn calculate_annual_salary(base_salary: f64) -> f64 {
    // Your implementation here
}

pub fn calculate_tax(salary: f64) -> f64 {
    // Your implementation here
}
```

### `main.rs`
```rust
mod employee;
mod department;
mod salary;

use employee::Employee;
use department::Department;

struct Company {
    // Your fields here
}

impl Company {
    // Your methods here
}

fn main() {
    // Test your implementation
}
```

## Key Concepts

This project helps you learn:

1. **Module Declaration**: `mod employee;` declares a module
2. **Public API**: Use `pub` keyword to make items public across modules
3. **Imports**: `use employee::Employee;` brings items into scope
4. **Cross-module Usage**: Employees reference Department IDs
5. **Separation of Concerns**: Each file has a specific responsibility
6. **Error Handling**: Using `Result` and `Option` types
7. **File Organization**: Building a cohesive multi-file project

---

## Testing

You can create a tests module in `main.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_employee() {
        let mut company = Company::new();
        let dept = Department::new(1, String::from("Engineering"), 100000.0);
        company.add_department(dept);

        let emp = Employee::new(
            101,
            String::from("Alice"),
            String::from("alice@example.com"),
            1,
            5000.0
        ).unwrap();

        assert!(company.add_employee(emp).is_ok());
    }

    #[test]
    fn test_invalid_email() {
        let result = Employee::new(
            101,
            String::from("Bob"),
            String::from("invalid-email"),
            1,
            5000.0
        );
        assert!(result.is_err());
    }
}
```

---

## Bonus Challenges

1. Add a `promotion` module that handles promotions between departments
2. Create an `error_types` module with custom error enums instead of `String` errors
3. Implement `Display` trait for `Employee` and `Department` for pretty printing
4. Add a function to find the highest paid employee in a department
5. Implement serialization/deserialization with `serde` to save/load company data
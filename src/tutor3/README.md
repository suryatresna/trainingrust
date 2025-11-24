# Tutorial 3: Shopping Cart System

## Overview
This tutorial demonstrates a complete shopping cart implementation with product management, quantity tracking, and discount calculations. You'll work with nested structs, advanced error handling, and mathematical operations.

## Learning Objectives

By completing this tutorial, you will understand:
- Complex struct relationships and composition
- Advanced `Result` type usage with meaningful error messages
- Vector manipulation with `find`, `position`, and `swap_remove`
- Floating-point arithmetic and precision handling
- State management across related structs
- Iterator chains with `map`, `sum`, and closures

## Concepts Covered

### 1. Nested Struct Composition
```rust
struct Product {
    id: u32,
    name: String,
    price: f64,
}

struct CartItem {
    product: Product,
    quantity: u32,
}

struct ShoppingCart {
    items: Vec<CartItem>,
    discount_percent: f64,
}
```
The cart contains items, which contain products—a three-level data structure.

### 2. Smart Item Addition
```rust
match self.items.iter_mut().find(|item| item.product.id == new_item.product.id) {
    Some(item) => {
        item.quantity += quantity;
        Ok(())
    },
    None => {
        self.items.push(new_item);
        Ok(())
    }
}
```
Automatically increments quantity if the product already exists in the cart.

### 3. Efficient Item Removal
```rust
if let Some(index) = self.items.iter().position(|item| item.product.id == product_id) {
    self.items.swap_remove(index);
    return Ok(format!("Item with product ID {} successfully removed", product_id))
}
```
Uses `position()` to find the index, then `swap_remove()` for O(1) removal.

### 4. Mathematical Calculations
```rust
fn subtotal(&self) -> f64 {
    self.items.iter()
        .map(|item| item.product.price * item.quantity as f64)
        .sum()
}

fn total(&self) -> f64 {
    let subtotal = self.subtotal();
    subtotal - (subtotal * self.discount_percent / 100.0)
}
```
Demonstrates iterator chains and discount calculation logic.

### 5. Comprehensive Validation
Multiple validation patterns:
- Quantity validation (must be > 0)
- Discount range validation (0.0 to 100.0)
- Product existence validation

## Code Structure

### Product Struct
- `id`: Unique product identifier
- `name`: Product name
- `price`: Unit price

### CartItem Struct
- `product`: The product being purchased
- `quantity`: Number of units

### ShoppingCart Struct with Methods
- `new()`: Creates an empty cart with 0% discount
- `add_item()`: Adds product or increases quantity if it exists
- `remove_item()`: Removes a product completely from cart
- `update_quantity()`: Changes the quantity of an existing item
- `set_discount()`: Sets the discount percentage (0-100)
- `subtotal()`: Calculates total before discount
- `total()`: Calculates final total after discount
- `item_count()`: Returns number of unique products
- `list_items()`: Returns reference to all cart items

## Running the Tutorial

From the project root:
```bash
# Run the program
cargo run --bin tutor3

# Run the tests
cargo test --bin tutor3
```

Expected output:
```
Items in cart: 2
Item: Laptop, Quantity: 1
Item: Mouse, Quantity: 2
Item with product ID 1 successfully removed
Items in cart after removal: 1
Item: Mouse, Quantity: 2
Subtotal: $51.00
Total: $45.90
Items: 1
```

## Understanding Key Methods

### Adding Items with Deduplication
The `add_item` method prevents duplicate products:
```rust
fn add_item(&mut self, product: Product, quantity: u32) -> Result<(), String> {
    if quantity == 0 {
        return Err(String::from("Quantity must be greater than 0"))
    }

    match self.items.iter_mut().find(|item| item.product.id == new_item.product.id) {
        Some(item) => item.quantity += quantity,  // Increment existing
        None => self.items.push(new_item),        // Add new
    }
    Ok(())
}
```

### Discount Calculation
The discount is applied to the subtotal:
```rust
fn total(&self) -> f64 {
    let subtotal = self.subtotal();
    subtotal - (subtotal * self.discount_percent / 100.0)
}
```
For a 10% discount on $100: `100 - (100 * 10 / 100) = $90`

### Updating Quantities
```rust
fn update_quantity(&mut self, product_id: u32, quantity: u32) -> Result<(), String> {
    if quantity == 0 {
        return Err(String::from("Quantity must be greater than 0"))
    }

    match self.items.iter_mut().find(|item| item.product.id == product_id) {
        Some(item) => {
            item.quantity = quantity;
            Ok(())
        },
        None => Err(String::from("Product not found"))
    }
}
```

## Important Implementation Details

### Why `swap_remove` Instead of `remove`?
```rust
self.items.swap_remove(index);  // O(1) - doesn't preserve order
// vs
self.items.remove(index);       // O(n) - preserves order
```
`swap_remove` is faster because it swaps the element with the last one and pops, rather than shifting all subsequent elements.

### Type Conversion for Calculations
```rust
item.product.price * item.quantity as f64
```
The `as f64` cast is necessary because `quantity` is `u32`, but we need `f64` for multiplication with the price.

## Exercises

Try extending the functionality:

1. **Tax calculation**: Add a `calculate_tax()` method that applies tax to the total
2. **Apply discount before/after tax**: Add a tax_rate field and calculate appropriately
3. **Quantity-based discounts**: Apply larger discounts for higher quantities
4. **Product categories**: Add a category to Product and offer category-specific discounts
5. **Clear cart**: Add a `clear()` method to empty the entire cart
6. **Total item quantity**: Add a method that returns the sum of all quantities (not just unique items)
7. **Find item by ID**: Add a method to retrieve a cart item by product ID
8. **Price limits**: Add validation to reject products with negative or zero prices

## Testing Examples

The tutorial includes a basic test. Try adding more:

```rust
#[test]
fn test_add_duplicate_item() {
    let mut cart = ShoppingCart::new();
    let product = Product { id: 1, name: String::from("Test"), price: 10.0 };
    cart.add_item(product, 2).unwrap();

    let product2 = Product { id: 1, name: String::from("Test"), price: 10.0 };
    cart.add_item(product2, 3).unwrap();

    assert_eq!(cart.item_count(), 1);
    assert_eq!(cart.items[0].quantity, 5);
}

#[test]
fn test_discount_calculation() {
    let mut cart = ShoppingCart::new();
    let product = Product { id: 1, name: String::from("Item"), price: 100.0 };
    cart.add_item(product, 1).unwrap();
    cart.set_discount(10.0).unwrap();

    assert_eq!(cart.total(), 90.0);
}
```

## Key Takeaways

- Nested structs enable modeling complex real-world systems
- `if let` and `match` are both useful for handling `Option` and `Result`
- Iterator methods like `map()` and `sum()` make calculations concise
- Validation should happen at the method level, not just in `main()`
- Type conversions (`as`) are sometimes necessary for arithmetic
- Choose the right collection method (`swap_remove` vs `remove`) based on requirements

## Common Patterns

### Error Propagation with ?
```rust
cart.add_item(laptop, 1)?;
cart.set_discount(10.0)?;
```
The `?` operator propagates errors up the call stack.

### Iterator Chaining
```rust
self.items.iter()
    .map(|item| item.product.price * item.quantity as f64)
    .sum()
```

### Conditional Updates
```rust
match self.items.iter_mut().find(|item| item.product.id == id) {
    Some(item) => { /* modify */ },
    None => { /* handle error */ }
}
```

## Next Steps

Once comfortable with this tutorial, proceed to [Tutorial 4: Task Management System](../tutor4/README.md) to practice:
- Defining and using enums
- Pattern matching with enum variants
- Building a complete CRUD system independently
- Auto-incrementing IDs
- Filtering by multiple criteria
Question: Shopping Cart System
Create a shopping cart system that manages products and calculates totals with discounts.
Requirements:

Create a Product struct with:

id: u32
name: String
price: f64


Create a CartItem struct with:

product: Product
quantity: u32


Create a ShoppingCart struct with:

items: Vec<CartItem>
discount_percent: f64 (0.0 to 100.0)


Implement the following methods for ShoppingCart:

new() - Constructor that creates an empty cart with 0% discount
add_item(&mut self, product: Product, quantity: u32) -> Result<(), String> - Adds item to cart

If product already exists, increase quantity
Return Err if quantity is 0


remove_item(&mut self, product_id: u32) -> Result<String, String> - Removes item completely

Return Ok with product name if found
Return Err if not found


update_quantity(&mut self, product_id: u32, quantity: u32) -> Result<(), String> - Updates item quantity

If quantity is 0, remove the item
Return Err if product not found


set_discount(&mut self, percent: f64) -> Result<(), String> - Sets discount (0.0-100.0)

Return Err if percent is out of range


subtotal(&self) -> f64 - Returns total before discount
total(&self) -> f64 - Returns total after applying discount
item_count(&self) -> usize - Returns number of different products (not total quantity)
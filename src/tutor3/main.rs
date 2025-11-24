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

impl ShoppingCart {
    fn new() -> Self {
        ShoppingCart { 
            items: Vec::new(), 
            discount_percent:  0.0
        }
    }

    fn list_items(&self) -> &Vec<CartItem> {
        &self.items
    }

    fn add_item(&mut self, product: Product, quantity: u32) -> Result<(), String> {
        if quantity == 0 {
            return Err(String::from("Quantity must be greater than 0"))
        }

        let new_item = CartItem{
            product, quantity
        };

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
    }

    fn remove_item(&mut self, product_id: u32) -> Result<String,String> {
        if let Some(index) = self.items.iter().position(|item| item.product.id == product_id) {
            self.items.swap_remove(index);
            return Ok(format!("Item with product ID {} successfully removed", product_id))
        } 
        return Err(format!("Product ID {} not found", product_id))
        
    }

    fn subtotal(&self) -> f64 {
        self.items.iter().map(|item| item.product.price * item.quantity as f64).sum()
    }

    fn set_discount(&mut self, percent: f64) -> Result<(), String> {
        if percent < 0.0 || percent > 100.0 {
            return Err(String::from("Discount percent must be between 0 and 100"))
        }
        self.discount_percent = percent;
        Ok(())
    }

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

    fn total(&self) -> f64 {
        let subtotal = self.subtotal();
        subtotal - (subtotal * self.discount_percent / 100.0)
    }

    fn item_count(&self) -> usize {
        self.items.len()
    }
}

fn main() {
    let mut cart = ShoppingCart::new();
    
    let laptop = Product {
        id: 1,
        name: String::from("Laptop"),
        price: 999.99,
    };
    
    let mouse = Product {
        id: 2,
        name: String::from("Mouse"),
        price: 25.50,
    };
    
    cart.add_item(laptop, 1).unwrap();
    cart.add_item(mouse, 2).unwrap();

    println!("Items in cart: {}", cart.item_count()); // 2
    for item in cart.list_items() {
        println!("Item: {}, Quantity: {}", item.product.name, item.quantity);
    }

    match cart.remove_item(1) {
        Ok(msg) => println!("{}", msg),
        Err(err) => println!("{}", err),
    }

    print!("Items in cart after removal: {}\n", cart.list_items().len()); // 1
    for item in cart.list_items() {
        println!("Item: {}, Quantity: {}", item.product.name, item.quantity);
    }


    println!("Subtotal: ${:.2}", cart.subtotal());  // 1051.00
    
    cart.set_discount(10.0).unwrap();
    println!("Total: ${:.2}", cart.total());        // 945.90
    
    cart.update_quantity(2, 3).unwrap();
    println!("Items: {}", cart.item_count());       // 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_item() {
        let mut cart = ShoppingCart::new();
        let product = Product {
            id: 1,
            name: String::from("Test Product"),
            price: 50.0,
        };
        assert!(cart.add_item(product, 2).is_ok());
        assert_eq!(cart.item_count(), 1);
    }
}
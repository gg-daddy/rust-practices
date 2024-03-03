use structure::{Category, Customer, Order, Product};

fn main() {
    let customer = Customer::new(1, "John Doe".to_string(), "email".to_string());
    let product = Product::new(1, "Shirt".to_string(), 100.0, Category::Clothing);
    let order = Order::new(1, product, customer, 10);
    let total = order.calculate_order_price();
    println!("Total: {}!", total);
}

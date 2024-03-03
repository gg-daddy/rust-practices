use array_tool::vec::Intersect;
use structure_test::{Category, Customer, Order, Product};

fn main() {
    let customer = Customer::new(1, "John Doe".to_string(), "email".to_string());
    let product = Product::new(1, "Shirt".to_string(), 100.0, Category::Clothing);
    let order = Order::new(1, product, customer, 10);
    let total = order.calculate_order_price();
    println!("Total: {}!", total);

    let p1 = Product::new(2, "Book1".to_string(), 10.5, Category::Books);
    let p2 = Product::new(3, "Book2".to_string(), 10.5, Category::Books);
    let p3 = Product::new(4, "Book3".to_string(), 10.5, Category::Books);

    let vec1 = vec![&p1, &p2];
    let vec2 = vec![&p2, &p3];

    let intersect = vec1.intersect(vec2);
    println!("Intersect: {:?}", intersect);
}

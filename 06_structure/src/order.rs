use crate::customer::Customer;
use crate::product::Product;
pub struct Order {
    id: u32,
    product: Product,
    customer: Customer,
    quantity: u32,
}

impl Order {
    pub fn new(id: u32, product: Product, customer: Customer, quantity: u32) -> Order {
        Order {
            id,
            product,
            customer,
            quantity,
        }
    }
    pub fn calculate_order_price(&self) -> f64 {
        self.product.calculate_product_price() * self.quantity as f64
    }
}

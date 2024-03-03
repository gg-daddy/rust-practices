use crate::customer::Customer;
use crate::product::Product;
struct Order {
    id: u32,
    product: Product,
    customer: Customer,
    quantity: u32,
}

impl Order {
    fn calculate_total_price(&self) -> f64 {
        self.product.calculate_total_price() * self.quantity as f64
    }
}

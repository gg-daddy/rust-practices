mod category;

use crate::product::category::Category;

pub struct Product {
    id: u32,
    name: String,
    price: f64,
    category: Category,
}

impl Product {
    fn caculate_tax(&self) -> f64 {
        match self.category {
            Category::Clothing => self.price * 0.05,
            Category::Electronics => self.price * 0.18,
            Category::Grocery => self.price * 0.01,
            Category::Books => self.price * 0.0,
        }
    }

    pub fn calculate_total_price(&self) -> f64 {
        self.price + self.caculate_tax()
    }
}

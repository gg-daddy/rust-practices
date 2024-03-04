pub mod category;

use crate::product::category::Category;

#[derive(PartialEq, Debug)]
/// Represents a product.
pub struct Product {
    id: u32,
    name: String,
    price: f64,
    category: Category,
}

impl Product {
    /// Creates a new product with the specified id, name, price, and category.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the product.
    /// * `name` - The name of the product.
    /// * `price` - The price of the product.
    /// * `category` - The category of the product.
    ///
    /// # Returns
    ///
    /// A new `Product` instance.
    pub fn new(id: u32, name: String, price: f64, category: Category) -> Product {
        Product {
            id,
            name,
            price,
            category,
        }
    }

    /// Calculates the tax amount based on the product's category.
    ///
    /// # Returns
    ///
    /// The tax amount.
    fn calculate_tax(&self) -> f64 {
        match self.category {
            Category::Clothing => self.price * 0.05,
            Category::Electronics => self.price * 0.18,
            Category::Grocery => self.price * 0.01,
            Category::Books => self.price * 0.0,
        }
    }

    pub fn calculate_product_price(&self) -> f64 {
        self.price + self.calculate_tax()
    }
}

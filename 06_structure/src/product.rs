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

    /// Calculates the total price of the product including tax.
    ///
    /// # Returns
    ///
    /// The total price of the product.
    ///
    /// # Example
    ///
    /// ```
    /// use structure_test::Category;
    /// use structure_test::Product;
    ///
    /// let product = Product::new(1, "Shirt".to_string(), 20.0, Category::Clothing);
    /// let total_price = product.calculate_product_price();
    /// assert_eq!(total_price, 21.0);
    /// ```
    pub fn calculate_product_price(&self) -> f64 {
        self.price + self.calculate_tax()
    }
}

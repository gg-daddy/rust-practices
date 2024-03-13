// Problem 2: Fix the lines indicated in the code so that it compiles

use std::cell::RefCell;
struct Car {
    model: String,
    price: u32,
    status: RefCell<&'static str>,
}

impl Car {
    fn new(model: &str, price: u32) -> Self {
        Car {
            model: model.to_owned(),
            price,
            status: RefCell::new("Available"),
        }
    }

    fn sold(&self) {
        let new_status = match self.price {
            0..=50000 => "Sold - Economy",
            50001..=100000 => "Sold - Mid Range",
            _ => "Sold - Luxury",
        };
        *self.status.borrow_mut() = new_status; // fix this line
    }
}

fn main() {
    let car = Car::new("Sedan", 75000);
    car.sold();
    println!("Car Status: {:?}", car.status); // Fix this line
}

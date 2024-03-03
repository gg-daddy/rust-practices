struct Car {
    owner: String,
    year: u32,
    fuel_level: f32,
    price: u32,
}

impl Car {
    //associated function : 不需要实例化就可以调用的函数
    fn new(owner: String, year: u32) -> Self {
        Car {
            owner: owner,
            year: year,
            fuel_level: 0.,
            price: 0,
        }
    }

    fn month_assurance() -> u32 {
        100
    }

    //只读
    fn cost(&self) {
        println!("cost: {}", self.price + Self::month_assurance());
    }

    //可变
    fn refuel(&mut self, fuel: f32) {
        self.fuel_level += fuel;
    }

    //move ownership
    fn sell(mut self, new_owner: String) -> Self {
        self.owner = new_owner;
        self
    }
}

fn main() {
    let mut my_car = Car::new("John".to_string(), 2010);
    my_car.cost();
    my_car.refuel(0.5);
    println!("fuel_level: {}", my_car.fuel_level);

    let sold_car = my_car.sell("Wahaha".to_string());
    println!("sold_car owner: {}", sold_car.owner);
}

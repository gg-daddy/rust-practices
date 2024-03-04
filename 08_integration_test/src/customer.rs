pub struct Customer {
    id: u32,
    name: String,
    email: String,
}

impl Customer {
    pub fn new(id: u32, name: String, email: String) -> Customer {
        Customer { id, name, email }
    }
}

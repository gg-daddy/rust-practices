#[derive(Debug, Default)]
struct Customer {
    name: String,
    user_name: String,
    age: i32,
    email: String,
}

#[derive(Default)]
struct CustomerBuilder {
    name: String,
    user_name: Option<String>,
    age: Option<i32>,
    email: Option<String>,
}

impl CustomerBuilder {
    //在课程中 Builder new 是定义在了 Customer 结构体中，这里是定义在了 CustomerBuilder 结构体中。
    fn new(name: String) -> Self {
        Self {
            name: name,
            ..Default::default()
        }
    }

    //这里的方法返回 &mut Self，这样就可以链式调用。
    fn user_name(&mut self, user_name: String) -> &mut Self {
        self.user_name = Some(user_name);
        self
    }
    fn age(&mut self, age: i32) -> &mut Self {
        self.age = Some(age);
        self
    }
    fn email(&mut self, email: String) -> &mut Self {
        self.email = Some(email);
        self
    }
    fn build(&self) -> Customer {
        Customer {
            name: self.name.clone(),
            user_name: self.user_name.clone().unwrap_or_default(),
            age: self.age.unwrap_or_default(),
            email: self.email.clone().unwrap_or_default(),
        }
    }
}

fn main() {
    let mut builder = CustomerBuilder::new("Eason".to_string());
    let c1 = builder.build();
    println!("Customer: {:?}", c1);

    //链式调用
    builder
        .user_name("eason".to_string())
        .age(18)
        .email("test@gmail.com".to_string());

    //可以多次调用 build 方法，每次都会返回一个新的实例。 这个和 build 中使用的是 clone 方法有关。
    let c2 = builder.build();
    println!("Customer: {:?}", c2);

    let c3 = builder.build();
    println!("Customer: {:?}", c3);
}

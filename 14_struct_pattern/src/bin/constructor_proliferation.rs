#[derive(Debug, Default)]
struct Customer {
    name: String,
    user_name: String,
    age: i32,
    email: String,
}

/*
如果一个 struct 有很多个字段， 而且这些字段如果不设置，就采用 default。
一种方式是给出了很多的构造函数，但是这样会导致构造函数的数量成倍增长。 the proliferation （泛滥） of constructor functions
*/
impl Customer {
    fn new(name: String) -> Self {
        Self {
            name: name,
            ..Default::default()
        }
    }
    //rust 不支持方法重载，只能是换一个名字.
    fn new2(name: String, user_name: String) -> Self {
        Self {
            name: name,
            user_name: user_name,
            ..Default::default()
        }
    }
    fn new3(name: String, user_name: String, age: i32) -> Self {
        Self {
            name: name,
            user_name: user_name,
            age: age,
            ..Default::default()
        }
    }

    fn new4(name: String, user_name: String, age: i32, email: String) -> Self {
        Self {
            name: name,
            user_name: user_name,
            age: age,
            email: email,
        }
    }
}

fn main() {}

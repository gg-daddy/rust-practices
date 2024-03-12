//lifetime elision 仅仅适用于 fn 定义，不适用于 struct, enum, trait 等。
struct Person<'a> {
    name: &'a String,
    age: u16,
}

fn main() {
    let person;
    {
        let name = "Alice".to_string();
        person = Person {
            name: &name,
            age: 16,
        };
        println!("{}", person.name);
    }
    /*
    在 Rust 中，当一个变量的部分字段包含引用时，整个变量的生命周期都会受到这个引用生命周期的限制。
    在你的代码中，Person 结构体的 name 字段是一个引用，它的生命周期为 'a。这意味着 Person 实例的生命周期不能超过 'a。
    在 main 函数中，你在一个内部作用域创建了 name 变量，并将其引用存储在 Person 实例 person 中。
    然后，当这个内部作用域结束时，name 变量被销毁，因此 person 实例中的 name 引用变得无效。
    尽管你在作用域结束后没有直接使用 person.name，但是你试图访问 person 实例本身（例如，通过打印 person.age），这是不允许的，
    因为 person 实例的一部分（即 name 字段）已经无效。
    这就是为什么你的代码会导致编译错误。为了修复这个问题，你需要确保 name 变量在整个 person 实例的生命周期内都是有效的。
    */
    // println!("age: {}", person.age);
}

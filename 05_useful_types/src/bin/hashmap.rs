use std::collections::HashMap;

fn main() {
    let mut persons = HashMap::new();
    persons.insert("John", 25);
    persons.insert("Ben", 45);
    println!("{:?}", persons);

    //get return Option
    println!("{:?}", persons.get("John").unwrap());

    //check existence
    if persons.contains_key("Ben1") {
        println!("Ben is in the list");
    } else {
        println!("Ben is not in the list");
    }
    //可以通过 get 的结果来判断是否存在
    match persons.get("Ben") {
        Some(age) => println!("Ben's age is {}", age),
        None => println!("Ben's age is not available"),
    }

    //如果不适用 &， 则会发生所有权的转移。
    for (name, age) in &persons {
        println!("Person {} is {} years old", name, age);
    }
    println!("{:?}", persons);

    //insert 会进行覆盖
    persons.insert("JohnX", 26);
    println!("{:?}", persons.get("JohnX").unwrap());
    persons.insert("JohnX", 30);
    println!("{:?}", persons.get("JohnX").unwrap());

    //entry 会进行检查，如果不存在则插入，如果存在则返回已有的值。
    persons.entry("BenX").or_insert(50);
    println!("{:?}", persons.get("BenX").unwrap());
    persons.entry("BenX").or_insert(60);
    println!("{:?}", persons.get("BenX").unwrap());

    let mut age_frequence = HashMap::new();
    let ages = vec![1, 2, 5, 6, 7, 8, 9, 7, 8, 7, 5, 1, 100, 100, 100];
    for age in &ages {
        let frequence = age_frequence.entry(*age).or_insert(0);
        *frequence += 1;
    }
    println!("{:?}", age_frequence);
}

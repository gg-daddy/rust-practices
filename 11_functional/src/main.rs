#[derive(Debug)]
struct User {
    name: String,
    age: u16,
}

fn is_valid<V1, V2>(user: &User, v1: V1, v2: V2) -> bool
where
    V1: Fn(&str) -> bool,
    V2: Fn(u16) -> bool,
{
    v1(&user.name) && v2(user.age)
}

fn main() {
    let user1 = User {
        name: String::from("Bob"),
        age: 30,
    };

    let undefined_name = "undefnined".to_string();
    let validate_name = |name: &str| {
        let banned_name = &undefined_name;
        name.len() != 0 && name != banned_name
    };
    // println!("undefined: {}", undefined_name); // value used here after move

    let validate_age = |age: u16| age > 0;
    println!("Pretty Info: {:#?}", user1);
    println!("1=================");
    println!("Name is valid:{}", validate_name(&user1.name));
    println!("Age is valid:{}", validate_age(user1.age));
    println!("2=================");
    println!(
        "User: {:?} is valid:{}",
        user1,
        is_valid(&user1, &validate_name, validate_age)
    );
}

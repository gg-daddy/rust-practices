use std::collections::HashSet;

fn main() {
    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];

    let result = fruits
        .into_iter()
        .filter(|&word| word.starts_with("a") || word.starts_with("b"))
        .map(|word| word.to_uppercase())
        .collect::<Vec<String>>(); //turbofish syntax: ::<>
    println!("{:?}", result);

    //下面演示了 collect 可以收集到不同的类型。也说明了确实类型推断是有限制的，需要明确指定类型。
    //无论是通过 turbofish syntax 还是通过类型注解。
    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];
    let result = fruits
        .into_iter()
        .filter(|&word| word.starts_with("a") || word.starts_with("b"))
        .map(|word| word.to_uppercase())
        .collect::<HashSet<String>>();
    println!("{:?}", result);

    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];
    let result = fruits
        .into_iter()
        .filter(|&word| word.starts_with("a") || word.starts_with("b"))
        .map(|word| word.to_uppercase())
        .collect::<String>();
    println!("{:?}", result);
}

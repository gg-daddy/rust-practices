// Problem 2: Fix the code by completing the function signature

use std::vec;

#[derive(Debug)]
struct Wrapper {
    data: String,
}

fn modify_data(mut wrapper: Box<Wrapper>) -> Box<Wrapper> {
    wrapper.data = String::from("Modified");
    wrapper
}

fn main() {
    let original_wrapper = Box::new(Wrapper {
        data: String::from("Original"),
    });
    //Box 是拥有所有权的指针，所以传递给函数时，会发生所有权转移。
    //这种操作方式是一致的，可以看到下面 vec 的例子。
    let modified_wrapper = modify_data(original_wrapper);
    println!("after modified, wrapper is: {:?}", modified_wrapper);
    // println!("original wrapper is: {:?}", original_wrapper);

    let v1 = vec![String::from("Hello"), String::from("World")];
    let v2 = modify_vec(v1);
    println!("after modified, v1 is: {:?}", v2);
}

fn modify_vec(mut vec: Vec<String>) -> Vec<String> {
    vec.push(String::from("Modified"));
    vec
}

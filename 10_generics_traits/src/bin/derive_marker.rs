use std::fmt::Debug;

/// 介绍 derive 和 marker traits. 

//empty body， marker trait. 这个和 Java 的 marker interface 有类似作用。
trait Properties: Debug  + PartialEq{

}

#[derive(Debug,PartialEq)]
struct Student {
    name: String,
    age: u8,
}

impl Properties for Student{

}

fn main(){
    let s1 = Student{name: "xiaoming".to_string(), age: 16};
    let s2 = Student{name: "xiaohong".to_string(), age: 17};

    println!("s1 is {:?}", s1);
    //pretty print
    println!("s2 is {:#?}", s2);
    println!("Q: Is s1 equals s2? A: {}", s1 == s2);
}
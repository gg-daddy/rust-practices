use std::vec;

/*
Ownership Rules:
1.Each value has a variable that's it's owner.
2.A value can only have one owner at a time.
3.If the owner goes out of scope, the value will be cleaned up.
*/
fn main() {
    let s1 = String::from("hello");
    {
        //onece go out of scope, s1 will be cleaned up.
        let s2 = s1;
        println!("{}", s2);
        // println!("{}, world!", s1);
    }

    //对于基本数据类型，赋值操作是 copy 操作。对于引用类型(e.g. String)，赋值操作是 move 操作。
    let i1 = 6;
    let i2 = i1;
    println!("i1 = {}, i2 = {}", i1, i2);

    let c1 = 'a';
    let c2 = c1;
    println!("c1 = {}, c2 = {}", c1, c2);

    let vec1 = vec![1, 2, 3];
    //函数调用时，vec1的所有权被转移给了take_ownership函数。
    take_ownership(vec1);
    // println!("vec1 = {:?}", vec1);

    let vec2 = give_ownership();
    println!("function return vec2 = {:?}", vec2);

    let vec3 = vec![1, 2, 3];
    let mut vec4 = giveback_ownershipe(vec3);
    //即使是同一个类型，也不能再次使用vec3，因为vec3的所有权已经被转移给了giveback_ownershipe函数。
    // println!("function return vec3 = {:?}", vec3);
    println!("function return vec4 = {:?}", vec4);
    vec4.push(12);
    println!("after push, vec4 = {:?}", vec4);
}

fn giveback_ownershipe(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(10);
    vec
}

fn give_ownership() -> Vec<i32> {
    vec![3, 4, 5, 6]
}

fn take_ownership(mut vec: Vec<i32>) {
    vec.push(4);
    println!("vec = {:?}", vec);
}

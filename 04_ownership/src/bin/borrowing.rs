use std::vec;

/**
Borrowing Rules:
1.At any time, you can either have one mutable reference or any number of immutable references,
but not both at the same time.
2.References must always be valid.

These rules avoid below problems:
1.Data Race.
2.Dangling References.
*/
fn main() {
    let mut vec1 = vec![1, 2, 3];

    /*
    At any time, you can either have one mutable reference or any number of immutable references.
    同时只能有一个 mutable reference， 或者是多个 inmutable reference， 这两个是不能同时出现的。
    如何理解这个规则呢？
    从一个 reference 被创造，到最后一行被使用， 这个 reference 的生命周期就是这个 scope 的生命周期。
    这个声明周期内，不能再次创建一个 mutable reference， 但是可以创建多个 inmutable reference。
    */
    let vec2 = &vec1;
    let vec3 = &vec1;
    println!("vec1 = {:?}, vec2 = {:?}, vec3 = {:?}", vec1, vec2, vec3);

    let vec4 = &mut vec1;
    println!("vec4 = {:?} ", vec4);
    vec4.push(12);
    println!("vec4 = {:?}", vec4);
    println!("after updated ,vec1 = {:?}", vec1);

    //这个解释了上面的 References must always be valid 规则， 避免了 Dangling References 问题。
    //vec6 的生命周期只在 {} 内，所以 vec5 不能引用 vec6， 如果引用了就出现了 Dangling References 问题，
    //因为 vec6 已经随着 scope 被销毁了。
    let vec5 = {
        let _vec6 = vec![4; 4];
        // &vec6
    };
    println!("vec5 = {:?}", vec5);
}

use std::vec;

fn main() {
    let mut i1 = 13;
    let i1_mutable_ref = &mut i1;
    println!("i1_mutable_ref : {}", i1_mutable_ref);
    if *i1_mutable_ref == 13 {
        println!("i1_mutable_ref is 13");
    }

    //看起来 inmutable reference 是可以直接进行运算的。但是不能用于比较操作。
    let height = &5;
    let width = &4;

    let area = height + width;
    println!("{}", area);
    if *height > 4 {
        println!("height is greater than 4");
    }

    let _i2 = *i1_mutable_ref;
    *i1_mutable_ref = 14;

    let i1_immutable_ref = &i1;
    let _i3 = *i1_immutable_ref;
    println!("_i3 = {}", _i3);
    // *i1_immutable_ref = 14;

    let mut vec1 = vec![1, 2, 3];
    let vec1_mut_ref = &mut vec1;

    // let d_vec1 = *vec1_mut_ref;

    //mutable references when assigned are moved, while immutable references are copied.
    let _vec2 = vec1_mut_ref;
    // let _vec3 = vec1_mut_ref;
    let _deep_copy_vec1 = _vec2.clone();
    let _deep_copy_vec2 = _vec2.clone();

    let vec1_immutable_ref = &vec1;
    let _vec4 = vec1_immutable_ref;
    let _vec5 = vec1_immutable_ref;

    //无论是 inmutable 还是 mutable，都是可以通过 clone 来创建新的数据。
    let _deep_copy_vec1 = _vec4.clone();

    let s1 = String::from("Hello");
    let s2 = &s1;
    let s3 = &s2;
    let s4 = &s3;
    println!("s1 = {}, s2 = {}, s3 = {}, s4 = {}", s1, *s2, *s3, *s4);
}

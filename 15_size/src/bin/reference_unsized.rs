use std::mem::size_of;
fn main() {
    //这是一个包含三个 32 位整数的数组。
    //在 Rust 中，数组的大小是固定的，等于其元素的大小乘以元素的数量。
    //因此，size_of::<[i32; 3]>() 将返回 12（因为 32 位整数的大小是 4 字节）
    println!("Size of array : {}", size_of::<[i32; 3]>());

    //这是一个指向 32 位整数数组的切片。在 Rust 中，切片是一个包含指向数组的指针和数组的长度的结构体。
    //因此，size_of::<&[i32]>() 将返回切片的大小，而不是它指向的数组的大小。
    //在 32 位系统上，这将返回 8（因为指针和长度各占 4 字节）；在 64 位系统上，这将返回 16（因为指针和长度各占 8 字节）。
    println!("Size of array slice: {}", size_of::<&[i32]>());

    let num1 = [1, 2, 3];
    let num2: &[i32] = &[1, 2, 3];

    let mut sum = 0;
    for num in num1 {
        sum += num;
    }
    println!("Sum of num1: {}", sum);

    let mut sum = 0;
    for num in num2 {
        sum += num;
    }
    println!("Sum of num2: {}", sum);

    println!("Size of Circle Reference: {}", size_of::<&Circle>());
    println!("Size of Square Reference: {}", size_of::<&Square>());

    // 在 Rust 中，虚函数表（vtable）是用于实现动态分发的关键机制。当你创建一个 trait 对象（如 &dyn Trait），Rust 会创建一个包含两部分的复合数据类型：
    // 数据指针：这是一个指向实现了 trait 的具体类型的实例的指针。
    // 虚函数表（vtable）指针：这是一个指向虚函数表的指针，虚函数表是一个包含了 trait 方法的实现的函数指针的数组。
    // 当你通过 trait 对象调用一个方法时，Rust 会使用虚函数表来查找并调用正确的方法实现。这就是动态分发的过程。
    // 具体来说，当你调用 shape.area()（其中 shape 是一个 &dyn Shape 类型的变量），Rust 会做以下操作：
    // Rust 会查找 shape 的虚函数表。
    // Rust 会在虚函数表中查找 area 方法的函数指针。
    // Rust 会使用找到的函数指针来调用 area 方法。
    // 这种机制允许你在运行时处理不同的类型，而这些类型都实现了同一个 trait。这是一种形式的多态，它允许你编写更加灵活和可重用的代码。
    println!("Size of trait dyn reference: {}", size_of::<&dyn Shape>());
}

trait Shape {
    fn area(&self) -> f64;
}

#[derive(Debug)]
struct Circle {}

#[derive(Debug)]
struct Square {}

impl Shape for Circle {
    fn area(&self) -> f64 {
        3.14
    }
}
impl Shape for Square {
    fn area(&self) -> f64 {
        4.0
    }
}

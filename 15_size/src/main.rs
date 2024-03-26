use std::mem::size_of;

fn main() {
    //sized type
    println!("Size of i32: {}", size_of::<i32>());
    println!("Size of (i32,i32): {}", size_of::<(i32, i32)>());
    println!("Size of [i32;3]: {}", size_of::<[i32; 3]>());
    println!("Size of bool: {}", size_of::<bool>());

    struct Point {
        x: i32,
        y: i32,
    }
    println!("Size of struct Point: {}", size_of::<Point>());

    struct Point2 {
        x: bool,
        y: i32,
    }
    //Because of alignment, the size of Point2 is 8 bytes.
    println!("Size of struct Point2: {}", size_of::<Point2>());

    println!(
        "Size of immutable reference no matter what types being referred: {}",
        size_of::<&i32>()
    );
    println!(
        "Size of mutable reference no matter what types being referred: {}",
        size_of::<&mut Point>()
    );

    println!("Size of Option<i32>: {}", size_of::<Option<i32>>());
    println!("Size of Option<()>: {}", size_of::<Option<()>>());
    println!("Size of (): {}", size_of::<()>());
    //机器字是计算机处理器在一次操作中可以处理的数据的位数。例如，在 32 位系统中，机器字的大小是 4 字节；在 64 位系统中，机器字的大小是 8 字节。
    println!("Machine word size is: {}", size_of::<&()>());
    println!("Box<i32> size is: {}", size_of::<Box<i32>>());
    println!(
        "Function pointer size is: {}",
        size_of::<fn(i32, f32) -> i32>()
    );
    println!("Size of String is: {}", size_of::<String>());

    // ::<> is used to specify the type parameter of a function。 通常可以通过函数的参数来推断类型参数，但是有时候需要显式指定类型参数。
    println!("{}", size2::<i32>());

    //unsized types.
    //接下来两个都是 slice 类型，它们的大小是 16 字节。
    //这是因为它们是指向数组的指针，而数组的大小是 8 字节，所以指针的大小是 8 字节。
    //另外，slice 类型还包含一个长度字段，所以它的大小是 16 字节。

    //doesn't have a size known at compile-time. the trait `Sized` is not implemented for `[i32]`
    // println!("Size of [i32] is: {}", size_of::<[i32]>());
    println!("Size of [i32] is: {}", size_of::<&[i32]>());

    // println!("Size of str is: {}", size_of::<str>());
    println!("Size of &str is: {}", size_of::<&str>());

    trait MyTrait {}
    // println!("Size of dyn MyTrait is: {}", size_of::<dyn MyTrait>());
    println!("Size of dyn MyTrait is: {}", size_of::<&dyn MyTrait>());
}

fn size2<T>() -> usize {
    size_of::<T>()
}

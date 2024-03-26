use std::fmt::Debug;

/*
"Coercion" 这个词在计算机科学中通常被翻译为 "强制类型转换" 或 "隐式类型转换"。
它来源于英语单词 "coerce"，意为 "强迫" 或 "强制"。在编程语言中，"coercion" 通常指的是一种自动的、隐式的类型转换方式。
在 Rust 中，"coercion" 是一种特殊的类型转换，它在特定的上下文中自动发生，无需程序员显式指定。
例如，如果一个函数期望一个 &str 类型的参数，但你传入了一个 String 类型的变量，Rust 编译器会自动将 String 类型的变量转换（或者说 "强制"）为 &str 类型。
这种自动的、隐式的类型转换方式可以简化代码，使得程序员无需显式处理类型转换，同时也保证了类型系统的安全性和一致性。
*/
fn main() {
    let s1 = String::from("Hello World!");
    str_slice_fn(&s1); // Coercion: &String -> &str

    let l1 = [1, 2, 3, 4];
    array_slice_fn(&l1); // Coercion: &[i32; 4] -> &[i32].  unsized coercion.

    let l2 = vec![1, 2, 3, 4];
    array_slice_fn(&l2); // Coercion: &Vec<i32> -> &[i32]. defref coercion.

    let l3 = &[1, 2, 3, 4];
    array_slice_fn(l3);
}

fn str_slice_fn(s: &str) {
    println!("{}", s);
}

fn array_slice_fn<T: Debug>(s: &[T]) {
    println!("{:?}", s)
}

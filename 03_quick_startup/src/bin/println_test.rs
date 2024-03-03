fn main() {
    // single line comments
    /*
     multiple
     line
     comments
    */

    //print in single line
    print!("Hello, world!");
    print!("Good morning!");

    //escape characters
    print!("\n");

    //return carriage
    println!("Hello, World!\rGood morning!");

    //positioned arguments
    println!("Hello {1}, {2}, {0}", "World", "Good", "morning");

    //named arguments
    println!(
        "Hello {p1}, {p2}, {p3}",
        p2 = "World",
        p1 = "Good",
        p3 = "morning"
    );

    //如何从 标准输入 读取值。
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");
    // let n: f64 = input.trim().parse().unwrap();
    let n: f64 = input.trim().parse().expect("Please type a number!"); //相比 unwrap 可以提供异常信息。
    println!("input number: {}", n);

    // 开头的 _ 可以避免 unused variable 的警告。
    let _number_one = 1;
    println!("number_one: {}", _number_one);

    /*
    初始化：const 和 static 都需要在编译时就能确定值，但 const 只是一个常量表达式，而 static 是一个在内存中真实存在的位置。
    可变性：const 总是不可变的，而 static 可以是可变的（使用 static mut）。
    生命周期：const 没有固定的内存地址，它在使用时会被复制到使用它的地方，而 static 有固定的内存地址，它的生命周期是整个程序的运行期。
    内存位置：static 变量存储在程序的数据段，而 const 变量不占用任何存储空间。
    类型限制：static 变量必须是 'static 生命周期的，也就是说它们必须能够在整个程序运行期间有效。const 没有这个限制。
     */
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS: {}", MAX_POINTS);
    static MAX_POINTS_STATIC: u32 = 100_000;
    println!("MAX_POINTS_STATIC: {}", MAX_POINTS_STATIC);
}

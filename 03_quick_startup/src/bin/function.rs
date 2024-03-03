fn main() {
    test("Hello World!");
    let t1 = test2(100, 15);
    println!("t1: {}", t1);
    let t2 = test3(10, 45);
    println!("t2: {:?}", t2);

    let (product, sum, subtract) = test3(10, 45);
    println!("product: {}, sum: {}, subtract: {}", product, sum, subtract);

    //code block, 最后一个表达式的值就是返回值， 可以赋给其他的变量。
    let t3 = {
        let n1 = (sum + subtract) / 2;
        let n2 = (sum - subtract) / 2;
        n1 * n2
    };
    println!("t3: {}", t3);
}
// 如果不定义返回值，默认返回的就是 unit type ()
fn test(s: &str) {
    println!("{}", s);
}

// 最后是一个表达式，而且没有分号，其值就是返回值
fn test2(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

// 利用 tuple， 返回多个值
fn test3(num1: i32, num2: i32) -> (i32, i32, i32) {
    (num1 * num2, num1 + num2, num1 - num2)
}

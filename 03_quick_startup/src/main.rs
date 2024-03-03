fn main() {
    //默认都是不可变
    let a = 10.0;
    //下面的两种方式效果一样
    println!("a = {a}");
    println!("a = {}", 10);

    //主动声明 mut， 可变变量
    let mut b = 10;
    b = 20;
    println!("b = {}", b);

    //scoping
    {
        let c = 1.1;
        println!("c = {}", c);
    }
    // println!("c = {}", c); //cannot find value `c` in this scope

    //shadowing 变量遮蔽
    let d = 1.0;
    let d = "test shadowing";
    println!("d = {}", d);

    //constants
    const MAX_POINTS: u32 = 100_000; //constant 必须显示给出类型
    println!("MAX_POINTS = {}", MAX_POINTS);
}

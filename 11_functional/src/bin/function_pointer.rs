// Problem 1: Complete the function signature for `sum_of_squares`. 
//It must not contain any generics. 

fn add(x: u32, y: u32) -> u32 {
    x + y
}

fn square(x: u32) -> u32 {
    x * x
}

//function point use keyword fn, not Fn. Fn 是 closure trait. 
//function point 不能捕获外部的变量。
fn sum_of_squares(num: u32, sq: fn(u32) -> u32, add: fn(u32,u32) -> u32) -> u32 { 
    let mut result = 0;
    for i in 1..=num {
        result = add(result, sq(i));
    }
    result
}

//function pointer 也实现了Fn trait, 所以可以使用trait bound。 
fn sum_of_squares2<V1, V2>(num: u32, sq: V1, add: V2) -> u32 
where
    V1: Fn(u32) -> u32,
    V2: Fn(u32,u32) -> u32,
{ 
    let mut result = 0;
    for i in 1..=num {
        result = add(result, sq(i));
    }
    result
}

fn main() {
    let num = 4;
    let sum = sum_of_squares(num, square, add);
    println!("Sum of squares from 1 to {} = {}", num, sum);
    
    let sum2 = sum_of_squares2(num, square, add);
    println!("Sum of squares from 1 to {} = {}", num, sum2);

    let test = 10;
    let add_closure = |x: u32, y: u32| {
        x + y + test
    };
    let square_closure = |x: u32| x * x;
    sum_of_squares2(num, square_closure, add_closure);

    //上面的 add_clousre 捕获了外部的参数 test, 所以不能使用function pointer, 只能使用closure。
    //如果闭包没有捕获外部的参数，就可以应用在使用function pointer 的地方。
    // sum_of_squares(num, square_closure, add_closure);
}

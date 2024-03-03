extern crate chrono;

fn main() {
    let a = 50;

    // if else 是一个表达式，可以返回值。注意尾巴上的 ;。
    let b = if a < 50 {
        'A'
    } else if a < 100 {
        'B'
    } else {
        'C'
    };
    println!("b: {}", b);

    //matching
    let a = 50;
    match a {
        1..=50 => println!("One"),
        _ => println!("Something else"),
    }

    //matching with return value， 此时，尾巴上的 ; 不能省略。
    let a = 100;
    let c = match a {
        1..=50 => "One",
        _ => "Something else",
    };
    println!("c: {}", c);

    let mut loop_times = 2;
    //simple loop, 默认无限循坏，除非 break.
    loop {
        //print current timestamp and formatted with yyyy-MM-dd
        let current_time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        println!("Current timestamp: {}", current_time);
        // break two seconds.
        std::thread::sleep(std::time::Duration::from_secs(2));
        loop_times -= 1;
        if loop_times == 0 {
            break;
        }
    }

    //可以用 label + break， 指定跳出哪一层循环。
    'outlabel: loop {
        break 'outlabel;
    }

    //loop with break， 可以是一个表达式。 注意最后的 ； 不能省略。
    let b = loop {
        break 10;
    };
    println!("loop exp with break value, b: {}", b);

    //使用 for loop 迭代数据。
    let data = vec![1; 10];
    for i in data {
        println!("i: {}", i);
    }

    //使用 while loop
    let mut num = 10;
    while num >= 0 {
        num -= 1;
    }

}

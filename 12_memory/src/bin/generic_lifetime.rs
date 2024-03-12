/*
generic lifetime: 泛型生命周期.
最主要的目的是为了解决函数的参数和返回值的生命周期问题， 避免出现 dangling reference 的问题。
dangling reference: 悬空引用， 即引用了一个已经被释放的内存。
每个 reference 都有一个生命周期， 即它的有效范围。因为是引用，要考虑这个引用的生命周期和它所引用的对象的生命周期。

结合前面的 checking rules， reference must be valid. 也就是说， reference 的生命周期必须是有效的， 其有效范围必须在它所引用的对象的生命周期内。
否则就会出现 dangling reference 的问题。
*/

fn main() {
    let i = 5;
    let j = 10;
    let r = picking_value(&i, &j);
    println!("r :{}", r);

    let r1;
    {
        let k = 20;
        r1 = picking_value(&i, &k);
        println!("r1 :{}", r1); // r1 只能在当前的 scope 内使用， 因为 k 的生命周期是在当前的 scope 内。
    }

    let r2;
    {
        let k = 20;
        r2 = picking_value2(&i, &k); //即使 k 的生命周期在当前的 scope 内，但是返回的生命周期是 'static, 所以 r2 可以在当前的 scope 外使用。
    }
    println!("r1 :{}", r2);
}

//返回的生命周期是 i 和 j 的生命周期中的最短的那个。
fn picking_value<'a>(i: &'a i32, j: &'a i32) -> &'a i32 {
    if rand::random() {
        i
    } else {
        j
    }
}

fn picking_value2(i: &i32, j: &i32) -> &'static i32 {
    let y: &'static i32 = &18;
    y
}

fn picking_value3(i: &i32, j: &i32) {
    println!("i :{}", i);
}

fn picking_value4<'a>(i: &'a i32, test: &str) -> &'a i32 {
    i
}

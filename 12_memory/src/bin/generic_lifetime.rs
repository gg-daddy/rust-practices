/*
generic lifetime: 泛型生命周期
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

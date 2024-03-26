use std::mem::size_of;
fn main() {
    println!("size_of::<()>() = {}", size_of::<()>());
    println!("size_of::<[i32; 0]>() = {}", size_of::<[i32; 0]>());
    println!("size_of::<()>() = {}", size_of::<()>());
    println!("size_of::<EmptyStruct>() = {}", size_of::<EmptyStruct>());
    // let never_end = never_returns();

    // typle 实现了 Copy trait
    let x = ();
    let y = x;
    let z = x;

    //对于 empty struct， 是 move 语义
    let a = EmptyStruct;
    let b = a;
    // let c = a;// value borrowed here after move
}
struct EmptyStruct;

fn never_return_always_run() -> ! {
    loop {
        println!("This function never returns!")
    }
}

fn never_return_panic() -> ! {
    panic!("This is a panic function!")
}

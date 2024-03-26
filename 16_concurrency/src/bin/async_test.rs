use futures::executor::block_on;

async fn hello_world() -> i32 {
    println!("hello, world1!");
    120
}

fn main() {
    let future = hello_world2(); // Nothing is printed
    block_on(future);
}

async fn hello_world2() {
    println!("hello, world2!");
    let r = hello_world().await;
    println!("r: {}", r);
}

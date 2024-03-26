use std::thread;
use std::time::Duration as duration;
fn main() {
    let value = String::from("hello");
    let handle = thread::spawn(move || {
        for i in 1..=15 {
            println!(
                "hi number {} from the spawned thread! moved value: {}",
                i, value
            );
            thread::sleep(duration::from_secs(1));
        }
    });
    thread::sleep(duration::from_secs(10));

    // println!("hi number {} from the main thread!", value); // 上面的闭包中使用了value，所以value被move了，这里不能再使用value。
    for i in 1..10 {
        println!("hi number {} from the main thread!", i);
    }
    handle.join().unwrap();
}

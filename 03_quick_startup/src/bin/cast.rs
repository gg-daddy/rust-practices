fn main() {
    /*
    The 'as' keyword in Rust performs type casting, but it is lossy,
    meaning that unnecessary information can be discarded.
    When we use 'as' to convert between types, there is a risk of losing data without any warning.
    In the given example, the variable 'y' is assigned the value 4,294,967,296,
    which exceeds the maximum value that can be stored in a u32 type.
    As a result, the value is truncated. It is surprising that neither the Rust compiler, nor the Clippy,
    or the runtime generates any warning or error to indicate that data loss has occurred.
     */
    let x: u64 = 4_294_967_296;
    let y = x as u32;
    println!("x: {}, y: {}", x, y);
    if x == y as u64 {
        println!("x equals y.");
    } else {
        println!("x does not equal y.");
    }
}

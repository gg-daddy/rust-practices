fn main() {
    //integer types
    let a: i8 = -10;
    println!("a: {}", a);

    //unsigned integer
    let b: u8 = 10;

    //floating point
    let c: f32 = 10.0;

    //character
    let d = 'a';

    //string
    let e = "Hello, World!";
    println!("e: {}", e);

    //boolean
    let f = false;
    println!("f: {}", f);

    //type aliasing
    type Age = u8;
    let g: Age = 30;
    println!("Age g: {}", g);

    //type casting
    let f1: f64 = 1.0;
    let f2: f32 = f1 as f32;
    println!("f2: {}", f2);

    //boolean 
    let b1 = true;
    let b2 = false;
    if b1 != b2 {
        println!("b1 and b2 are not equal");
    }
}

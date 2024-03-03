struct S {
    x: i32,
}
 
 
fn main() {
    let mut s1 = S { x: 2 };
    let v = &mut s1;
    v.x += 1;
    s1.x += 1;
    print!("{}{}", v.x, s1.x);
}
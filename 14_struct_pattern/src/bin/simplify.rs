/*
The decomposition of a struct enables us to borrow fields independently of each other.

This decomposition of structs often leads to a better design,
with smaller and therefore easily manageable units of functionality.
*/
struct A {
    b: B,
    c: C,
}

struct B {
    f1: u32,
}

struct C {
    f2: u32,
    f3: u32,
}

fn fn1(a: &mut B) -> &u32 {
    &a.f1
}

fn fn2(a: &mut C) {
    let x = a.f2 + a.f3;
    println!("{}", x);
}

fn main() {
    let mut a = A {
        b: B { f1: 1 },
        c: C { f2: 2, f3: 3 },
    };
    let r1 = fn1(&mut a.b);
    println!("{}", r1);
    fn2(&mut a.c);
    println!("{}", r1);
}

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
    //通过避免借用整个结构体，可以避免借用检查。
    let r1 = fn1(&mut a.b); //只借用了 a.b
    println!("{}", r1);
    fn2(&mut a.c); //只借用了 a.c， 和上面的 fn1 借用不冲突。避免了借用检查。
    println!("{}", r1);
}

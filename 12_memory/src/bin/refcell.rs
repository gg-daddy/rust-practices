use std::{cell::RefCell, rc::Rc};

fn main() {
    // let x = RefCell::new(42);
    // let mut y = x.borrow_mut();
    // *y += 1;
    // println!("{:?}", x);
    // drop(y);
    // println!("{:?}", x);

    let mut x = 10;
    let x1 = &x;
    let x2 = &x;
    let mut x3 = &mut x;
    println!("{}", x3);
    //这个违反了 Rust 的借用规则，所以会报错。
    // println!("{} {}", x1, x2);

    let a = RefCell::new(5);
    let b = a.borrow();
    let c = a.borrow();
    //下面这行代码在编译期不会报错，但是运行时报错。
    //RefCell 是一种在运行时检查借用规则的类型。
    //而且感觉比前面使用 & 会更加严格，也许是因为 RefCell 是在运行时检查的。
    // let d = a.borrow_mut();
    drop(b);
    drop(c);
    let d = a.borrow_mut(); //主动 drop 之后就可以再次 borrow_mut 了。
    println!("{}", d);

    let a = RefCell::new(5);
    let mut b = a.borrow_mut(); //即使 a 是不可变的，但是可以通过 borrow_mut 获取可变引用， 并对其进行修改。
    *b = 10;
    println!("{:?}", a); //RefCell { value: <borrowed> }
    drop(b);
    println!("{:?}", a); //主动 drop 之后，就可以看到 value 的值了。

    //RefCell 可以和 Rc 一起使用，这样就可以在多个地方共享一个可变引用了。
    let a = Rc::new(RefCell::new(String::from("Hellow")));
    let b = Rc::clone(&a);
    *b.borrow_mut() = String::from("World");
    println!("{:?}", a);
}

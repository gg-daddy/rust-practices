/*
smart pointers， 智能指针。 Rust 中的智能指针是一类数据结构，它们表现类似指针，但是也拥有额外的元数据和功能。
通过 & 和 &mut 引用的是普通的引用，而智能指针通常是实现了 Deref 和 Drop trait 的结构体。

*/
fn main() {
    let x = 5;
    let y = &x;
    let mut z = Box::new(x);
    *z = 20;
    println!("x,y,z : {},{},{}", x, y, z);
    let z1 = Box::new(y);
    println!("z1 : {:?}", z1);

    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("{:#?}", list);
    //在上面的 List 中， Nil variant 没有占用内存空间，但是也不得不使用 Box 放到 heap 上。
    //下面的 List2 使用 Option 来代替 Box，这样就不需要占用内存空间了。
    // None 和 Box::new(Nil) 表达的含义是类似的，但是从 memory 和 performance 角度看是不同的。
    let list2 = List2::Cons(1, Some(Box::new(List2::Cons(2, None))));
    println!("{:#?}", list2);

    let p1 = Person {
        name: "zhangsan",
        age: 20,
    };
    println!("p1 : {:?}", p1);

    let p2 = Box::new(Person {
        name: "lisi",
        age: 30,
    });
    println!("p2 : {:?}", p2);

    //因为 p1 是在栈上的，所以赋值给 p3 时，会发生拷贝，而 p2 是在堆上的，所以赋值给 p4 时，只是拷贝了指针。
    let p3 = p1;
    println!("p3 : {:?}", p3);

    let p4 = p2;
    println!("p4 : {:?}", p4);

    //expected `HugeData`, found `SmallData`
    // let vec = vec![Box::new(HugeData {}), Box::new(SmallData {})];
    //上面的代码会报错，因为 vec! 宏会自动推断类型，而 Box::new(HugeData {}) 和 Box::new(SmallData {}) 的类型不一样。
    //显示指定类型，而且使用 trait object，就不会报错了。
    let vec1: Vec<Box<dyn Storage>> = vec![Box::new(HugeData {}), Box::new(SmallData {})];
    for v in vec1 {
        v.storage_name()
    }
}

struct HugeData {}
struct SmallData {}

trait Storage {
    fn storage_name(&self);
}

impl Storage for HugeData {
    fn storage_name(&self) {
        println!("HugeData");
    }
}
impl Storage for SmallData {
    fn storage_name(&self) {
        println!("SmallData");
    }
}

#[derive(Debug)]
enum List {
    //如果不使用Box，编译器会报错：error[E0072]: recursive type `List` has infinite size.
    Cons(i32, Box<List>),
    Nil,
}

#[derive(Debug)]
enum List2 {
    //如果不使用Box，编译器会报错：error[E0072]: recursive type `List` has infinite size.
    Cons(i32, Option<Box<List2>>),
}

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u16,
}

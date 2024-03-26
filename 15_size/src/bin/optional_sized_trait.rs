use std::fmt::Debug;

#[derive(Debug)]
struct OptionalFieldStruct<T: ?Sized> {
    field1: u32,
    field2: T,
}

fn print_fn<T: Debug + ?Sized>(t: &T) {
    println!("{:?}", t);
}

fn main() {
    let o1 = OptionalFieldStruct {
        field1: 1,
        field2: [1, 2, 3, 4, 5], //定义的时候使用 ?Sized， 此处就可以使用 raw array slice。
    };
    println!("{:?}", o1);

    let s1 = "your name!";
    print_fn(s1);

    /*
         Parameter Type:    T           &T          &T
    Function call Input:    &str        &str        &&str
            Resolves To:    T = &str    T = str     T = &str
     */
}

/*
Lifetime Elision Rules:

1. The first rule is that each parameter that is a reference gets its own lifetime parameter annotation.
2. The second rule is if there is exactly one input lifetime parameter that lifetime is assigned to all output lifetimes.
3. And finally, if there are multiple input lifetime parameters, but one of them is a reference to self
or a mutable reference to self, the lifetime of self is assigned to all the output lifetime parameters.
*/

fn main() {
    let s1 = String::from("hello");
    let return_str = return_str(&s1);
    println!("return_str :{}", return_str);

    let return_str2 = return_str2(&s1, &10);
    println!("return_str2 :{}", return_str2);

    let t = Test { a: &s1 };

    let ann = &"World!".to_string();
    let r1 = t.announcement(ann);
    println!("r1 :{}", r1);

    let r2 = t.return_ann(ann);
    println!("r2 :{}", r2);
}

//Rule 2. The second rule is if there is exactly one input lifetime parameter that lifetime is assigned to all output lifetimes.
fn return_str(s: &String) -> &String {
    s
}

//如果上面三个规则都不满足，那么就会报错。需要手工指定生命周期。
fn return_str2<'a>(s: &'a String, s2: &i32) -> &'a String {
    println!("s2 :{}", s2);
    s
}

struct Test<'a> {
    a: &'a String,
}

impl Test<'_> {
    //Rule 3. And finally, if there are multiple input lifetime parameters,
    //but one of them is a reference to self or a mutable reference to self,
    //the lifetime of self is assigned to all the output lifetime parameters.
    fn announcement(&self, announcement: &String) -> &String {
        println!("announcement :{}", announcement);
        self.a
    }

    fn return_ann<'a>(&self, announcement: &'a String) -> &'a String {
        println!("announcement :{}", announcement);
        announcement
    }
}

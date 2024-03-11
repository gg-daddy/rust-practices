/*
concrete lifetime: 具体生命周期
*/
fn main() {
    let i = 5;
    let j = i;
    println!("i :{i}");

    let i;
    {
        let j = 32;
        i = &j;
        println!("i :{i}");
    }

    let s1 = String::from("hello");
    let s2 = s1;
    // println!("s1 :{s1}"); //value borrowed here after move

    /*
    The rust uses another concept called non-lexical lifetimes (非词法声明周期).
    Non-lexical lifetimes aim to relax some of the strictness imposed by the typical lifetimes.
    This is achieved by analyzing the actual usage of references in the code, rather than solely relying on scopes.
    Simply put, the non-lexical lifetimes are lifetimes which are not tied to scopes.
     */
    let mut v1 = vec![1, 2, 3, 4, 5];
    //对于 ref_v1 的生命周期，从它的声明开始，直到它的最后一次使用结束。
    let ref_v1 = &v1;
    // let mut v2 = &mut v1; //mutable reference 和 inmutable reference 不能同时存在， 这违反了 borrow checker 的规则, 编译会失败。
    println!("v1 :{:?}", ref_v1);
}

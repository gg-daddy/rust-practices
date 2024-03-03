fn main() {
    let mut vec1 = vec![1, 2, 3];
    take_inmutable_reference(&vec1);
    take_mutable_reference(&mut vec1);
    println!("vec1 = {:?}", vec1);

    //可以利用 shadowing， 重新使用 vec1。
    let vec1 = take_ownership(vec1);
    println!("vec1 = {:?}", vec1);

    let vec2 = give_ownership();
    println!( "vec2 = {:?}", vec2);
}

/*
从设计的时候，就是最小权限原则。
1. 如果只需要读取数据，就使用 inmutable reference。
2. 如果需要修改数据，就使用 mutable reference。
3. 如果需要所有权，就使用 move 操作。
*/
fn take_inmutable_reference(vec: &Vec<i32>) {
    println!("vec = {:?}", vec);
}

fn take_mutable_reference(vec: &mut Vec<i32>) {
    vec.push(12);
    println!("vec = {:?}", vec);
}

fn take_ownership(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(4);
    println!("vec = {:?}", vec);
    vec
}

/*
 * 如果是返回一个新的数据结构，而且在外面还需要，就必须给 Ownership。
 */
fn give_ownership() -> Vec<i32> {
    vec![3, 4, 5, 6]
}

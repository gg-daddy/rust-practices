/*
在 Rust 中，Rc 是一个引用计数类型，用于使多个所有者共享数据。
Rc 的全称是 Reference Counting，即引用计数。当你需要在多处同时访问同一数据，但又不想克隆数据时，可以使用 Rc。
Rc 通过在堆上存储数据和引用计数来工作。每当一个新的引用到 Rc 的数据被创建时，引用计数就会增加。
当引用被丢弃时，引用计数就会减少。当引用计数降至 0 时，数据就会被清理。

需要注意的是，Rc 只能用于单线程环境。如果你需要在多线程环境中共享数据，应该使用 Arc（Atomic Reference Counting）。
另外，Rc 不支持数据的内部可变性。如果你需要修改 Rc 中的数据，可以使用 Cell 或 RefCell。

Smart pointer has many use cases.

For instance, in graph data structure we have multiple edges that may point to the same node.
Conceptually, the node is owned by all the edges that point to it.
A node shouldn't be cleaned up unless it doesn't have any edges pointing to it, and so has no owners.

Another use case is in doubly linked list, which we will cover in the upcoming section.
*/
use std::rc::Rc;

#[derive(Debug)]
enum List<T> {
    Node(T, Rc<List<T>>),
    Nil,
}
fn main() {
    //在这个例子中，我们创建了一个新的 Rc 包含值 5，然后我们克隆了这个 Rc。
    //这并不会导致数据的复制，而只是增加了引用计数。
    let five = Rc::new(5);
    println!("{}", Rc::strong_count(&five));
    let shared_five = Rc::clone(&five);
    println!("{}", Rc::strong_count(&shared_five));

    //下面的例子，如果使用 Box ， 会发生所有权转移，导致 list2 和 list3 无法使用 list 了。
    //使用 Rc ，可以解决这个问题。list2 和 list3 也是 list 的所有者。
    let list = Rc::new(List::Node(1, Rc::new(List::Node(2, Rc::new(List::Nil)))));
    println!("{:?}, count: {}", list, Rc::strong_count(&list));
    {
        let list2: List<i32> = List::Node(3, Rc::clone(&list));
        println!("{:?}, count: {}", list2, Rc::strong_count(&list));
        let list3 = List::Node(4, Rc::clone(&list));
        println!("{:?}, count: {}", list3, Rc::strong_count(&list));
        // list2 和 list3 在这里离开作用域，引用计数减少。
    }
    println!("{:?}, count: {}", list, Rc::strong_count(&list));
}

/*
单链表
*/
fn main() {
    let empty_list = LinkedList { head: None };
    println!("{:?}", empty_list);

    let mut linked_list = LinkedList::new();
    linked_list.add(1);
    linked_list.add(2);
    linked_list.add(3);
    linked_list.add(4);
    linked_list.print();

    linked_list.remove();
    linked_list.remove();
    linked_list.print();

    println!("{:?}", linked_list.peak());
    linked_list.print2();
}

impl LinkedList {
    fn new() -> LinkedList {
        LinkedList { head: None }
    }

    fn add(&mut self, element: i32) {
        let new_head = Box::new(Node {
            element: element,
            next: self.head.take(), //self.head.take() 的作用是取出 self.head 的值（如果有的话），并将 self.head 设置为 None。
        });
        self.head = Some(new_head);
    }

    fn remove(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.element
        })
    }

    fn print(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            print!("{} -> ", node.element);
            current = &node.next;
        }
        println!("None");
    }

    /*
    list_traversal.as_ref() 是将 list_traversal 转换为引用类型的 Option。
    as_ref 方法是 Option 类型的一个方法，它将 Option<T> 转换为 Option<&T>。
    这样做的好处是，你可以在不取出 Option 中的值的情况下查看它，因为 as_ref 不会改变 Option 的所有权。
    然后，unwrap() 方法是 Option 类型的一个方法，它会返回 Option 中的值。
    如果 Option 是 None，那么 unwrap() 会引发 panic。因此，使用 unwrap() 时需要确保 Option 是 Some，否则程序会在运行时崩溃。
    */
    fn print2(&self) {
        let mut list_traversal = &self.head;
        while !list_traversal.is_none() {
            println!("{:?}", list_traversal.as_ref().unwrap().element);
            list_traversal = &list_traversal.as_ref().unwrap().next;
        }
    }

    fn peak(&self) -> Option<i32> {
        self.head.as_ref().map(|node| node.element)
    }
}

#[derive(Debug)]
struct LinkedList {
    head: Pointer,
}

#[derive(Debug)]
struct Node {
    element: i32,
    next: Pointer,
}

type Pointer = Option<Box<Node>>;

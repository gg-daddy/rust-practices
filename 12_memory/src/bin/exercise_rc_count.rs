use std::rc::Rc;
#[derive(Debug)]
struct ListNode<T> {
    value: T,
    next: Option<Rc<ListNode<T>>>,
}

fn main() {
    let node_3 = Rc::new(ListNode {
        value: 3,
        next: None,
    });

    let node_2 = Rc::new(ListNode {
        value: 2,
        next: Some(Rc::clone(&node_3)),
    });

    let node_1 = Rc::new(ListNode {
        value: 1,
        next: Some(Rc::clone(&node_2)),
    });

    assert_eq!(Rc::strong_count(&node_1), 1);
    assert_eq!(Rc::strong_count(&node_2), 2);
    assert_eq!(Rc::strong_count(&node_3), 2); //这个可以仔细思考下.
}

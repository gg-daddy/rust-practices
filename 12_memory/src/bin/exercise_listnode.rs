// enum、generic、Box
#[derive(Debug)]
enum ListNode<T> {
    //TODO: Declare an enum variant called Node, with Box pointer for the next node of type 'T'
    //TODO: Another variant for the placeholder for the end of the list
    Node(T, Box<ListNode<T>>),
    Nil,
}

fn main() {
    // Create a linked list representing: Node(1, Node(2, Node(3, Node(4, None))))
    let list = ListNode::Node(
        1,
        Box::new(ListNode::Node(
            2,
            Box::new(ListNode::Node(
                3,
                Box::new(ListNode::Node(4, Box::new(ListNode::Nil))),
            )),
        )),
    );
    println!("{:?}", list);
}

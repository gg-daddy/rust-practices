use std::{cell::RefCell, fmt::Debug, rc::Rc};

fn main() {
    let mut list = DoubleLinkedList::new();
    list.add_to_head(1);
    list.add_to_head(2);
    list.add_to_head(3);
    list.print();
    list.remove_from_head();
    list.print();
}

#[derive(Debug)]
struct DoubleLinkedList {
    head: Pointer,
    tail: Pointer,
}

impl DoubleLinkedList {
    fn new() -> Self {
        DoubleLinkedList {
            head: None,
            tail: None,
        }
    }

    fn add_to_head(&mut self, element: i32) {
        let new_node: Rc<RefCell<Node>> = Node::new(element);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_node.clone());
                new_node.borrow_mut().next = Some(old_head);
                self.head = Some(new_node); // ownership transfer here, and there is no need to use clone.
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node); // ownership transfer here, and there is no need to use clone.
            }
        }
    }

    fn add_to_tail(&mut self, element: i32) {
        /* Put your Code Here*/
        let new_tail = Node::new(element);

        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail.clone());
                self.tail = Some(new_tail);
            }

            None => {
                self.tail = Some(new_tail.clone());
                self.head = Some(new_tail);
            }
        }
    }

    fn remove_from_head(&mut self) -> Option<i32> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    self.tail.take();
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().element
        })
    }

    fn remove_from_tail(&mut self) -> Option<i32> {
        /*Add Code Here */
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().element
        })
    }

    fn print(&self) {
        let mut current = self.head.clone();
        while let Some(node) = current {
            print!("{} ->", node.borrow().element);
            current = node.borrow().next.clone();
        }
        println!("None")
    }
}

#[derive(Debug)]
struct Node {
    element: i32,
    next: Pointer,
    prev: Pointer,
}

impl Node {
    fn new(element: i32) -> NodeWraper {
        Rc::new(RefCell::new(Node {
            element,
            next: None,
            prev: None,
        }))
    }
}
type NodeWraper = Rc<RefCell<Node>>;
type Pointer = Option<NodeWraper>;

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

fn main() {
    test_drop_when_out_scope();
    test_rc_recycle();
    test_weak_ref();
    test_weak_ref_parent();
}

fn test_weak_ref_parent() {
    let leaf = Rc::new(MultiChildNode {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(MultiChildNode {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    //下面的 * 不能省略，因为 borrow_mut 返回的是 RefCell 的 RefMut，而不是 &mut
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!("branch: {:?}", branch);
}

fn test_weak_ref() {
    let n1 = Rc::new(RefCell::new(WeakNode {
        value: 1,
        next: None,
    }));
    let n2 = Rc::new(RefCell::new(WeakNode {
        value: 2,
        next: None,
    }));
    println!(
        "Before linking, n1 strong count: {}, weak count: {} . n2 strong count: {}, weak count: {}",
        Rc::strong_count(&n1),
        Rc::weak_count(&n1),
        Rc::strong_count(&n2),
        Rc::weak_count(&n2)
    );
    n1.borrow_mut().next = Some(Rc::downgrade(&n2));
    n2.borrow_mut().next = Some(Rc::downgrade(&n1));
    println!(
        "After linking, n1 strong count: {}, weak count: {} . n2 strong count: {}, weak count: {}",
        Rc::strong_count(&n1),
        Rc::weak_count(&n1),
        Rc::strong_count(&n2),
        Rc::weak_count(&n2)
    );
    println!("n1 created. {:?}", n1); // weak ref 默认不会打印出来，所以不会有循环调用问题。
}

fn test_rc_recycle() {
    let n1 = Rc::new(RefCell::new(RcNode {
        value: 1,
        next: None,
    }));
    let n2 = Rc::new(RefCell::new(RcNode {
        value: 2,
        next: None,
    }));
    n1.borrow_mut().next = Some(Rc::clone(&n2));
    n2.borrow_mut().next = Some(Rc::clone(&n1));
    // println!("n1 created. {:?}", n1); //fatal runtime error: stack overflow.
}

fn test_drop_when_out_scope() {
    {
        let n1 = RcNode {
            value: 1,
            next: None,
        };
        println!("n1 created. {:?}", n1);
    }
    println!("n1 dropped.");
}

#[derive(Debug)]
struct RcNode {
    value: i32,
    next: Option<Rc<RefCell<RcNode>>>,
}

impl Drop for RcNode {
    fn drop(&mut self) {
        println!("Dropping Node with value {}", self.value);
    }
}

#[derive(Debug)]
struct WeakNode {
    value: i32,
    next: Option<Weak<RefCell<WeakNode>>>,
}

impl Drop for WeakNode {
    fn drop(&mut self) {
        println!("Dropping WeekNode with value {}", self.value);
    }
}

#[derive(Debug)]
struct MultiChildNode {
    value: i32,
    parent: RefCell<Weak<MultiChildNode>>,
    children: RefCell<Vec<Rc<MultiChildNode>>>,
}

use std::{cell::RefCell, rc::Rc};

fn main() {
    let file = Rc::new(RefCell::new(File { active_user: 0 }));

    let user1 = User {
        file: Rc::clone(&file),
    };
    user1.file.borrow_mut().active_user += 1;
    println!("activate file user: {}", file.borrow().active_user);
    let user2 = User {
        file: Rc::clone(&file),
    };
    user2.file.borrow_mut().active_user += 1;
    println!("activate file user: {}", file.borrow().active_user);
}

struct File {
    active_user: u32,
}

struct User {
    file: Rc<RefCell<File>>,
}

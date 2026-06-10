use crate::CellList::{CellCons, CellNil};
use crate::InfList::{InfCons, InfNil};
use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

#[derive(Debug)]
enum CellList {
    CellCons(Rc<RefCell<i32>>, Rc<CellList>),
    CellNil,
}

#[derive(Debug)]
enum InfList {
    InfCons(i32, RefCell<Rc<InfList>>),
    InfNil,
}

impl InfList {
    fn tail(&self) -> Option<&RefCell<Rc<InfList>>> {
        match self {
            InfCons(_, item) => Some(item),
            InfNil => None,
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    println!("\n\n=== Box type ===\n\n");

    let b = Box::new(5);
    println!("b = {b}");

    let x = 5;
    assert_eq!(5, x);

    //let list = Cons(1, Cons(2, Cons(3, Nil)));

    let y = Box::new(x);
    //let y = &x;

    assert_eq!(5, *y);

    // Both lines 10 and 11 would give errors with below assertion
    //assert_eq!(5, y);

    let z = 5;
    let a = MyBox::new(z);

    assert_eq!(5, z);
    assert_eq!(5, *a); // This will only work when the Deref trait is defined for MyBox type

    println!("\n\n=== Custom Smart Pointer ===\n\n");

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointer created!");

    let e = CustomSmartPointer {
        data: String::from("someone's stuff to drop early"),
    };

    let f = CustomSmartPointer {
        data: String::from("anyone's stuff"),
    };

    //e.drop(); // This will not work as Rust does not allow explicit calling Drop trait's drop method to avoid double free bug.
    drop(e);

    println!("CustomSmartPointer will be dropped before the end of main");

    println!("\n\n=== Reference counting smart pointer ===\n\n");

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c got out of scope = {}", Rc::strong_count(&a));

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(CellCons(Rc::clone(&value), Rc::new(CellNil)));

    println!("\n\n=== Interior mutability with RefCell ===\n\n");

    let b = CellCons(Rc::new(RefCell::new(3)), Rc::clone(&a));

    let c = CellCons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");

    println!("\n\n=== Memory leak using ref cycles ===\n\n");

    let a = Rc::new(InfCons(5, RefCell::new(Rc::new(InfNil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(InfCons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack.
    // println!("a next item = {:?}", a.tail());

    println!("\n\n=== Reference Cycle ===\n\n");

    println!("\n\n=== Weak reference ===\n\n");

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    println!(
        "leaf strong = {} weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {} weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );

        println!(
            "leaf strong = {} weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    println!(
        "leaf strong = {} weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    println!("\n\n=== CustomSmartPointer drops ahead ===\n\n")
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

// Using Box with List would not work since Box moves the interior element.
//enum List {
//    Cons(i32, Box<List>),
//    Nil,
//}
//
//fn main() {
//    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
//    let b = Cons(3, Box::new(a));
//    let c = Cons(4, Box::new(a));
//}

pub trait Messenger {
    fn send(&self, m: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over quota!")
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You have exceeded over 90% of your quota!")
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You have exceeded over 75% of your quota!")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        // sent_messages: Vec<String>,
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                // sent_messages: vec![],
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // self.sent_messages.push(String::from(message));
            self.sent_messages.borrow_mut().push(String::from(message));

            // Using borrow_mut() second time in inline invocation does not seem to give error.
            // Maybe it will give error when we try to save the value to a variable.
            // self.sent_messages.borrow_mut().push(String::from(message));

            // let mut one_borrow = self.sent_messages.borrow_mut();

            // This will give error
            // let mut two_borrow = self.sent_messages.borrow_mut();
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

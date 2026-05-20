use crate::List::{Cons, Nil};
use std::ops::Deref;
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
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

    println!("CustomSmartPointer dropped before the end of main");

    let g = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let h = Cons(3, Rc::clone(&g));
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

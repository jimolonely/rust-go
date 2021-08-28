use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a count={}", Rc::strong_count(&a));
    println!("a tail={:?}", a.tail());
    
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a count={}", Rc::strong_count(&a));
    println!("b count={}", Rc::strong_count(&b));
    println!("b tail={:?}", b.tail());
    
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    println!("b count={}", Rc::strong_count(&b));
    println!("a count={}", Rc::strong_count(&a));

    // println!("a tail={:?}", a.tail()); // 栈溢出
}
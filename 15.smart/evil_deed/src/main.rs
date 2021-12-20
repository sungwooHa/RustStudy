
use std::rc::Rc;
use std::cell::RefCell;
use List::{Cons, Nil};

#[derive(Debug)]
enum List{
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List{
    fn tail(&self) -> Option<&RefCell<Rc<List>>>{
        match self{
            Cons(_, item) => Some(item),
            Nil=>None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a)); //1
    println!("a next item = {:?}", a.tail()); //a(5) -> none

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a)))); //clone을 통해 rc++

    println!("a rc count after b creation = {}", Rc::strong_count(&a)); //2
    println!("b initial rc count = {}", Rc::strong_count(&b)); //1
    println!("b next item = {:?}", b.tail()); //b(10) -> a(5) -> none

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b); // a(5) -> b(10)   : 순환 시작
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b)); // 2
    println!("a rc count after changing a = {}", Rc::strong_count(&a)); // 2

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail()); 
    // a->b->a->b->...... bomb!
}
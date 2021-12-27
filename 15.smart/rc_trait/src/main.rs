#[derive(Debug)]
enum List{
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;


fn main() {

    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil))); //rc++;
    println!("count after creating a = {}", Rc::strong_count(&a)); //1

    let b = Cons( Rc::new(RefCell::new(6)), Rc::clone(&a)); //rc++
    println!("count after creating b = {}", Rc::strong_count(&a)); //2
    {
        let c = Cons( Rc::new(RefCell::new(7)), Rc::clone(&a)); //rc ++
        println!("count after creating c = {}", Rc::strong_count(&a)); //3, scope 나가면서 rc--

    }

    println!("a before = {:?}", a);
    println!("b before = {:?}", b);

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);

    println!("count after c goes out of scope = {}", Rc::strong_count(&a)); //2
}
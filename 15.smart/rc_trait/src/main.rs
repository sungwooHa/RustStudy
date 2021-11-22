enum List{
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil))))); //rc ++ 
    println!("count after creating a = {}", Rc::strong_count(&a)); //1
    let b = Cons(3, Rc::clone(&a)); //rc++
    println!("count after creating b = {}", Rc::strong_count(&a)); //2
    {
        let c = Cons(4, Rc::clone(&a)); //rc ++
        println!("count after creating c = {}", Rc::strong_count(&a)); //3, scope 나가면서 rc--
    }

    println!("count after c goes out of scope = {}", Rc::strong_count(&a)); //2
}
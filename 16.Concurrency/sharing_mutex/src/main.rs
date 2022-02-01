use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        {
            println!("prev handle num: {}", &counter.lock().unwrap());
        }

        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;

            {
                println!("in handle num: {}", &num);
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
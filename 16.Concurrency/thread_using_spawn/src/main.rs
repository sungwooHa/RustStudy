use std::thread;
use std::time::Duration;

fn spawn_test(){
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn main() {
    spawn_test();
    for i in 1..10 {
        println!("main func lifetime number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
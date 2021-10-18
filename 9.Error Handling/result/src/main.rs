use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // println!("Hello, world!");

    // let fileName = "hello.txt";
    // let f = File::open(fileName);


    // let f = match f {
    //     Ok(file) => file,
    //     Err(ref error) if error.kind() == ErrorKind::NotFound => {
    //         match File::create(fileName){
    //             Ok(fc) => fc,
    //             Err(e) => {
    //                 panic!("Tried to create file but there was a problem : {:?}", e)
    //             },
    //         }
    //     },
    //     Err(error) => {
    //         panic!("there was a problem opening the file : {:?}", error)
    //     },
    // };

    let f = File::open("helloa.txt").expect("Failed to open hello.txt");
}

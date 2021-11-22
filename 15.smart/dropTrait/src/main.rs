struct CustomSmartPointer {
    data : String
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self){
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    println!("Hello, world!");

    let c = CustomSmartPointer {data : String::from("my stuff") };
    let d = CustomSmartPointer {data : String::from("other stuff")};

    drop(c);
    drop(d);
    println!("CustomSmartPointers created.");
}

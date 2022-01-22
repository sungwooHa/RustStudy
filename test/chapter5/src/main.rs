#[derive(Debug)]
struct IpAddrStruct{
    v4 : (u8,u8,u8,u8),
    v6 : String,
}

#[derive(Debug)]
enum IpAddr{
    V4(u8, u8, u8, u8),//-> 4개의 u8, 튜플구조체인가?
    V6(String),
}

fn main() {
    // Create example variables
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);
    
    // Variable a matches Foo::Bar
    if let Foo::Bar = a {
        println!("a is foobar");
    }
    
    // Variable b does not match Foo::Bar
    // So this will print nothing
    if let Foo::Bar = b {
        println!("b is foobar");
    }
    
    // Variable c matches Foo::Qux which has a value
    // Similar to Some() in the previous example
    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    // Binding also works with `if let`
    if let Foo::Qux(value @ 100 @ 10) = c {
        println!("c is one hundred");
    }
}
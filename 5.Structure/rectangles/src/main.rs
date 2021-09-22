


fn main() {
    
    // let length : u32 = 50;
    // let width : u32 = 30;


    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(length, width)
    // );

    // //tuple
    // let rectangle_measurement = (50,30);
    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area_tuple(rectangle_measurement)
    // );

    //structure
    let rectangle_instance = Rectangle {length : 50, width : 30};
    
    println!("{} ", rectangle_instance.area());
    
    let rect1 = Rectangle { length: 50, width: 30 };
    let rect2 = Rectangle { length: 40, width: 10 };
    let rect3 = Rectangle { length: 45, width: 60 };
    
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    
}

// //function
// fn area(length : u32, width : u32) -> u32 {
//     length*width
// }

#[derive(Debug)]
struct Rectangle{
    length : u32,
    width : u32,
}

impl Rectangle{
    fn area(&self) ->u32 {
        self.length * self.width
    }

    fn can_hold(&self, other : &Rectangle) -> bool{
        self.length > other.length && self.width > other.width
    }
}
//structure
// fn area_structure(rectangle : &Reactangle) -> u32{
//     rectangle.length * rectangle.width
// }

// //tuple
// fn area_tuple(dimension : (u32, u32)) -> u32{
    //     dimension.0* dimension.1
//


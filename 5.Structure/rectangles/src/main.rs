


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
    let rectangle_instance = Reactangle {length : 50, width : 30};
    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area_structure(&rectangle_instance)
    // );

    println!(
        "{:?} test",  rectangle_instance
    );
}

// //function
// fn area(length : u32, width : u32) -> u32 {
//     length*width
// }

#[derive(Debug)]
struct Reactangle{
    length : u32,
    width : u32,
}

//structure
// fn area_structure(rectangle : &Reactangle) -> u32{
//     rectangle.length * rectangle.width
// }

// //tuple
// fn area_tuple(dimension : (u32, u32)) -> u32{
//     dimension.0* dimension.1
// }


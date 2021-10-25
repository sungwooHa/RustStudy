

fn get_largest(list : &[i32]) -> i32{
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn get_smallest(list :&[i32]) -> i32{
    let mut smallest = list[0];

    for &item in list.iter() {
        if item < smallest {
            smallest = item;
        }
    }

    smallest
}

fn main() {
    println!("Hello, world!");

    let number = vec![10,20,30,40,50];

    let largest = number.iter().max().unwrap();
    let smallest = number.iter().min().unwrap();

    println!("{} is largest number ", largest);
    println!("{} is smallest number ", smallest);

    println!("{} is largest_custom number ", get_largest(&number));
    println!("{} is smallest_custom number ", get_smallest(&number));

}

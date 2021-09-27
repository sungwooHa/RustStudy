mod lib;

use lib::exam::exam_2;

fn main() {
    println!("Hello, exam 2!");

    let input : String = exam_2::input_string();
    println!("input string : '{}', length : {}", input, input.len());
    println!("piglatin : {}", exam_2::convert_to_pig_latin(input));
}

mod lib;

use lib::exam::exam_1;

fn main() {
    println!("Hello, exam 2!");

    let input : String = exam_1::input_string();
    println!("input string : '{}', length : {}", input, input.len());
    println!("piglatin : {}", exam_1::convert_to_pig_latin(input));
}

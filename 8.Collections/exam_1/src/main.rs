mod lib;
use lib::exam::exam_1;

fn main() {
    println!("Hello, exam 1");

    let v = exam_1::get_temp_vec(30, 0..20);
    
    println!("min value : {}", exam_1::get_min(&v).unwrap());
    println!("max value : {}", exam_1::get_max(&v).unwrap());
    println!("average value : {}", exam_1::get_average(&v).unwrap());
    println!("median value : {}", exam_1::get_median(&v).unwrap());
    println!("mode value : {}", exam_1::get_mode(&v).unwrap());
}

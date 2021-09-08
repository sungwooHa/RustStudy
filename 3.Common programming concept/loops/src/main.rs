
fn plus_one(x: i32) -> i32 
{
    x + 1
}


fn main() 
{
    let mut tmp_number = 1;

    while tmp_number != 10
    {
        println!("result of function : {}", plus_one(tmp_number));
        tmp_number += 1;
    }


    println!("Hello, world!");
}

fn main() 
{

    println!("the value of number is: {}", get_number(true));
    println!("the value of number is: {}", get_number(false));
}

fn get_number(x : bool) -> i32
{
    let number = if x
    {
        5
    }
    else
    {
        6
    };

    number
}

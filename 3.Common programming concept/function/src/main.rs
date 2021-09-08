fn main() 
{
    println!("Hello, world!");

    let x = five();
    antoher_function02(x);
    
    println!("plus one : {}", plus_one(x));
}

// fn another_function01()
// {
//     println!("Another Function");
// }

fn antoher_function02(x : i32)
{
    println!("the value of x is { }", x);
}

// fn another_function03(x : i32, y : i32)
// {
//     println!(" x : {} , y : {} ", x , y );
// }

fn five() -> i32
{
    5
}

fn plus_one(x : i32) -> i32
{
    x+1
}
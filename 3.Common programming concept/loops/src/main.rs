fn main() 
{
    'outer: loop {
        'inner1 : loop{
            'inner2 : loop{
                break 'inner1;
            }
            println!("inner2 done");
            break;
        }
        println!("inner1 done");
        break;
    }
    println!("outer done");
}
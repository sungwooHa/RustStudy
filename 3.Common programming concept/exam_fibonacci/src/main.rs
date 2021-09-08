
//1,1,2,3,5,8

fn fibonacci(number :u32) -> u32 {
    
    let mut result = 0;

    let mut n1 = 0;
    let mut n2 = 1;

    if number == 1
    {
        result = n1;
    }
    else if number == 2
    {
        result = n2;
    }
    else
    {
        let mut i = 2;
        while i < number
        {
           result = n1 + n2;
           n1 = n2;
           n2 = result;

           i = i+1;
        }
    }

    result
}

fn main() {

    for i in 1..15 
    {
        println!("{}'s fibonacci = {}",i, fibonacci(i));
    }
}

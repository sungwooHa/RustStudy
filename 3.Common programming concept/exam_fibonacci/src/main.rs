use std::io;

//1,1,2,3,5,8

fn fibonacci_recursive(number :u32) -> u32 {

    if number == 0 {
        panic!(" 0 is not a right argument to fibonacci");
    }
    match number {
        1 | 2  => 1,
        _ => fibonacci_recursive(number-1) + fibonacci_recursive(number-2),
    }
}

fn fibonacci(number : u32) -> u32 {
    if number == 0 {
        panic!("0 is not a right argument to fibonacci");
    }

    let mut result = 0;

    let mut n1 = 1;
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

    loop {
        println!("n's fibonacci, type 'n'");
        println!("type \"exit\" to end the program");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("fail to read 'n'");

        if input.trim() == "exit"{
            println!("Goody bye");
            break;
        }

        let input : u32 = match input.trim().parse()
        {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid type, type only positive integer");
                continue
            },
        };


        println!("{}'s fibonacci(recursive) = {}", input, fibonacci_recursive(input));
        println!("{}'s fibonacci(repeat) = {}", input, fibonacci(input));
        println!("--------------------------");

    }
}

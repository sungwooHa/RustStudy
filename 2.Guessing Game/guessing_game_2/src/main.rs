use std::io;
use rand::Rng;
use std::cmp::Ordering;

struct RandomRange{
   min_value : u32,
   max_value : u32,
}

fn get_random_range_from_user() -> Result<RandomRange, &'static str>{
    println!("please input range (min, max)");

    let mut range = String::new();
    let range = match io::stdin()
        .read_line(&mut range){
            Ok(_) => range.trim(),
            Err(_) => {
                return Err("Failed to read line");
            },
        };
    
    if range.is_empty(){
        return Err("Invalid range (empty value)");
    }

    let range : Vec<&str> = range.split_whitespace().collect();

    match range.as_slice(){
        [range_min, range_max] =>{
            let min_value = match range_min.parse::<u32>(){
                Ok(numb) => numb,
                Err(_) => {
                    return Err("Invalid type (range -> min)");
                }
            };

            let max_value = match range_max.parse::<u32>(){
                Ok(numb) => numb,
                Err(_) => {
                    return Err("Invalid type (range -> max)");
                }
            };

            if min_value > max_value {
                Err("invalid range, max range must bigger than min range")
            }
            else{
                Ok( RandomRange{ min_value, max_value})
            }
        }
        _ => Err("Invalid range")
    }
}

fn get_guess_number_from_user() -> Result<u32, &'static str>{
    println!("Guess number !");

    let mut guess = String::new();
    let guess = match io::stdin().read_line(&mut guess){
        Ok(_) => guess,
        Err(_) => {
            return Err("Failed to read line");
        }
    };

    match guess.trim().parse(){
        Ok(numb) => Ok(numb),
        Err(_) => {
            return Err("It isn't digit number");
        },
    }
} 

fn main() {
    loop{
        let secret_number = match get_random_range_from_user() {
            Ok(random_range) => {
                println!("min value : {}, max value : {}", random_range.min_value, random_range.max_value);
                rand::thread_rng().gen_range(random_range.min_value..random_range.max_value + 1)
            },
            Err(err) => {
                eprintln!("ERROR Range : {}", err);
                continue
            }
        };

        println!("random number : {}", secret_number);

        loop{
            
            let guess_number = match get_guess_number_from_user() {
                Ok(numb) => numb,
                Err(err) => {
                    eprintln!("ERROR Guess : {}", err);
                    continue;
                }
            };

            println!("You guessed : {}", guess_number);

            match guess_number.cmp(&secret_number) {
                Ordering::Less => println!("Tool Small"),
                Ordering::Greater => println!("Too Big"),
                Ordering::Equal => {
                    println!("Collect");
                    break;
                },
            }
        }
    }
}
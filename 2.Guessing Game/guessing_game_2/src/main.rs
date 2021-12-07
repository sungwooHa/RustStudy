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
            if range_min > range_max {
                Err("invalid range, max range must bigger than min range")
            }
            else{
                Ok( RandomRange{
                    min_value : range_min.parse::<u32>().unwrap(), 
                    max_value : range_max.parse::<u32>().unwrap()
                })
            }
        }
        _ => Err("Invalid range")
    }
}

fn main() {
    loop{
        let secret_number = match get_random_range_from_user() {
            Ok(random_range) => {
                println!("min value : {}, max value : {}", random_range.min_value, random_range.max_value);
                rand::thread_rng().gen_range(random_range.min_value..random_range.max_value)
            },
            Err(err) => {
                eprintln!("invalid range of random : {}", err);
                continue
            }
        };

        println!("random number : {}", secret_number);

        loop{
            println!("Guess Number");

            let mut guess = String::new();
            let guess = match io::stdin().read_line(&mut guess){
                Ok(_) => guess,
                Err(_) => {
                    println!("Failed to read line");
                    continue;
                }
            };
            
            let guess : u32 = match guess.trim().parse(){
                Ok(num) => num,
                Err(_) => {
                    println!("It isn't digit number");
                    continue;
                }
            };

            println!("You guessed : {}", guess);

            match guess.cmp(&secret_number) {
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
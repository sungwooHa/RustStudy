use std::io;

fn fahrenheit_to_celcius(temperature : i32) -> i32
{
    (temperature*9/5) + 32
}

fn celcius_to_fahrenheit(temperature : i32) -> i32
{
    (temperature-32)*5/9
}

fn main() {

    loop    
    {
        println!("Convert Temperature");
        println!("1 : Fahrenheit(°F) to Celsius(°C)");
        println!("2 : Celsius(°C) to Fahrenheit(°F)");
        println!("-999 : exit");

        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Failed to read typing");

        let option: i32 = match option.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        match option {
            1 => println!("Fahrenheit(°F) to Celsius"),
            2 => println!("Celsius(°C) to Fahrenheit(°F)"),
            -999 =>
            {
                println!("Exit");
                break;
            }
            _ =>
            {
                println!("invalid number");
                continue;
            },
        }

        loop
        {
            println!("go to menu(-999)");
            println!("temperature : ");


            let mut temperature = String::new();
            io::stdin().read_line(&mut temperature).expect("Failed to read temperature");
    
            let temperature: i32 = match temperature.trim().parse()
            {
                Ok(num) => num,
                Err(_) => {
                    println!("invalid number");
                    continue
                },
            };

            if temperature == -999
            {
                println!("go to menu");
                break;
            }

            match option{
                1  =>  println!("input Temperature : { }°F, convert : {} °C", temperature, fahrenheit_to_celcius(temperature)),
                2  =>  println!("input Temperature : { }°C, convert : {} °F", temperature, celcius_to_fahrenheit(temperature)),
                _ => (),
            }
        }
    }
}

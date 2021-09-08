

fn get_day_string (day : u32) -> String{
    
    let day = match day{
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "fourth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eight",
        9 => "ninth",
        10 =>"tenth",
        11 =>"eleventh",
        12 =>"twelfth",
        _ => "",
    };
    day.to_string()
}

fn print_day_gift(day : u32){

    let day_lyric = match day {
        1 => "A Partridge in a Pear Tree",
        2 => "Turtle Doves",
        3 => "French Hens",
        4 => "Calling Birds",
        5 => "Golden Rings",
        6 => "Geese a Laying",
        7 => "Swans a Swimming",
        8 => "Maids a Milking",
        9 => "Ladies Dancing",
        10 =>"Lords a Leaping",
        11 =>"Pipers Piping",
        12 =>"Drummers Drumming",
        _ => "",
    };

    if day == 1
    {
        println!("{}", day_lyric);
    }
    else 
    {
        println!("{} {}", day, day_lyric);
    }
}

fn main() {
    println!("Hello, world!");

    for i in 1..13 //1~12를 의미.
    {
        println!("on the { } day of Christmas", get_day_string(i));
        println!("my true love sent to me:");
        for n in (1..i+1).rev() {

            if i != 1 && n == 1
            {
                print!("and ");
            }

            print_day_gift(n);

        }

        println!("");
    }
}

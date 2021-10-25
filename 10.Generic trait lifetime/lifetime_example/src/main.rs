use std::fmt::Display;

fn longest<'a>(x:&'a str, y: &'a str) -> &'a str{
    if x.len() > y.len(){
        x
    }else{
        y
    }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str  where T:Display{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }

}

struct ImportantExcerpt<'a> {
    part : &'a str,
}

fn main() {
    {
        let string1 = String::from("abcd");
        let string2 = String::from("xyz");
        let result1 = longest(string1.as_str(), string2.as_str());
        println!("1. the longest string is {}", result1);

        let result2 = longest_with_an_announcement(string1.as_str(), string2.as_str(), String::from("ann test"));
        println!("2. the longest string is {}", result2);
    }

    {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.')
            .next()
            .expect("Could not find a '.'");
        let i = ImportantExcerpt { part: first_sentence };

        println!("first sentence : {:?}", i.part);
    }
}

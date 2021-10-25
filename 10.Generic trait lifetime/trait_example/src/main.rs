mod lib;

use lib::Summarizable;
use lib::largest;
use lib::Tweet;

fn main() {
    println!("Hello, world!");

    let tweet = Tweet {
        username : String::from("horst_ebooks"),
        content : String::from("of course, as you probably already know, people"),
        reply : false,
        retweet : false,
    };

    println!("1 new Tweet : {}", tweet.author_summary());

    let numbers = vec![34,50,25,100,63];
    let result = largest(&numbers);
    println!("the largest number is {}", result);

    let chars = vec!['y', 'm', 'a', 'q'];
    let result = largest(&chars);
    println!("The largest char is {}", result);
}



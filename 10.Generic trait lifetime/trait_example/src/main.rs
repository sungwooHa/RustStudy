mod lib;

use lib::Summarizable;
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
}



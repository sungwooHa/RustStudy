pub trait Summarizable {
    fn author_summary(&self) -> String;
    fn summary(&self) ->String{
        String::from("(Read more...)")
    }
}

pub fn notify<T: Summarizable>(item : T){
    println!("Breaking  news! {}", item.summary());
}

pub fn largest<T : PartialOrd + Copy>(list: &[T]) -> T{
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}


pub struct NewsArticle{
    pub headline : String,
    pub location : String,
    pub author : String,
    pub content : String,
}

impl Summarizable for NewsArticle{
    fn author_summary(&self) -> String {
        format!("@{}", self.author)
    }
    // fn summary(&self) -> String{
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }
}

pub struct Tweet{
    pub username : String,
    pub content : String,
    pub reply : bool,
    pub retweet : bool,
}

impl Summarizable for Tweet {
    fn author_summary(&self) -> String {
        format!("@{}", self.username)
    }
}
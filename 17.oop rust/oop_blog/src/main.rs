mod lib;
use lib::*;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad a lunch today");

    let post = post.request_review();
    let post = post.approve();

    assert_eq!("I ate a salad a lunch today", post.content());
}
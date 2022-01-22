#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        active: user1.active,
        username: String::from("someusername123"),
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    dbg!(&user1);
    dbg!(&user2);

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

}
#[derive(Debug)]
struct User<'a, 'b>{
	username: &'a str,
	email: &'b str,
	sign_in_count: u64,
	active: bool,
}

fn main() {

    let email = String::from("someone@example.com");
    let name = String::from("someusername123");

    let user1 = User {
        email: &email,
        username: &name,
        active: true,
        sign_in_count: 1,
    };

    println!("{:?}", user1);
}

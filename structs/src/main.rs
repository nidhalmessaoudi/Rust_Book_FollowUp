fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("user549"),
        email: String::from("someone@example.com"),
        sign_in_count: 1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0,0, 0);

    let subject = AlwaysEqual;

    println!("User email is: {}", user1.email);

    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        email: String::from("user2@example.com"),
        ..user1
    };

    println!("Updated user email is: {}", user1.email);
    println!("The user2 username is: {}", user2.username);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1
    }
}
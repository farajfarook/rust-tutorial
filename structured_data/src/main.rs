fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        sign_in_count: 0,
        active: false,
    };
    user1.email = String::from("anothr@example.com");

    let user2 = build_user(String::from("aa@aa"), String::from("bb"));
    let user3 = User {
        email: String::from("cc@cc"),
        username: String::from("dd"),
        ..user2
    };

    println!("{}", user2.username);
    println!("{}", user3.sign_in_count);
    println!("{}", user3.active);

    // Tuple struct
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{}", black.0);
    println!("{}", origin.2);
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

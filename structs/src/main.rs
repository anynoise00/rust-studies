struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = build_user(String::from("someone@example.com"), String::from("someone123"));
    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1 // Struct update syntax
    };

    display_user_info(user2);

    // Tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn display_user_info(user: User) {
    println!("Username: {}\nEmail: {}\nIs Active: {}\nTimes Signed In: {}",
        user.username, user.email, user.active, user.sign_in_count);
}
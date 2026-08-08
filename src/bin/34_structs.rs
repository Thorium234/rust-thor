struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = build_user(
        String::from("user2@example.com"),
        String::from("user2"),
    );

    let user3 = User {
        email: String::from("user3@example.com"),
        ..user2
    };

    println!("User 3 email: {}", user3.email);
    println!("User 3 username: {}", user3.username);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someonename123"),
        active: true,
        sign_in_count: 1,
    };

    // let name = user1.username;
    user1.username = String::from("wallace123");

    let user2 = build_user(
        String::from("kyle@mail.com"),
        String::from("kyle123")
    );

    let user3 = User {
        email: String::from("james@email.com"),
        username: String::from("james"),
        ..user2
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}
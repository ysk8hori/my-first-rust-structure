#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_could_get_name() {
        let user = User {
            username: String::from("hello"),
            email: String::from("hello@example.com"),
            sign_in_count: 1,
            active: true,
        };
        assert_eq!(user.username, "hello");
    }

    #[test]
    fn it_could_change_name() {
        let mut user = build_user("hello", "hello@example.com");
        user.username = String::from("Yosuke");
        assert_eq!(user.username, "Yosuke");
    }

    #[test]
    fn use_struct_update_syntax() {
        let user1 = build_user("username", "email");
        let user2 = User {
            username: String::from("コピー"),
            email: String::from("email2@example.com"),
            ..user1
        };
        assert_eq!(user1.active, user2.active);
    }
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(username: &str, email: &str) -> User {
    User {
        username: username.to_string(),
        email: email.to_string(),
        sign_in_count: 0,
        active: true,
    }
}

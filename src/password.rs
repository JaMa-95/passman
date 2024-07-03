pub mod password_module {
    #[derive(Debug)]
    pub struct Password {
        pub user: String,
        pub platform: String,
        pub password: String,
    }

    impl Password {
        pub fn new(user: &str, platform: &str, password: &str) -> Password {
            Password {
                user: user.to_string(),
                platform: platform.to_string(),
                password: password.to_string(),
            }
        }
    }
}
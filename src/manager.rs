

pub mod manager_module {
    use std::error::Error;
    use std::path::Path;
    use std::fmt;
    use crate::database::database_module::Database;
    use crate::password::password_module::Password;


    #[derive(Debug)]
    pub struct PasswordManagerError {
        pub message: String,
    }

    impl PasswordManagerError {
        fn new(message: &str) -> PasswordManagerError {
            PasswordManagerError {
                message: message.to_string(),
            }
        }
    }

    impl From<rusqlite::Error> for PasswordManagerError {
        fn from(error: rusqlite::Error) -> Self {
            PasswordManagerError {
                message: error.to_string(),
            }
        }
    }

    impl fmt::Display for PasswordManagerError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.message)
        }
    }

    impl Error for PasswordManagerError {}

    pub struct PasswordManager {
        database: Database,
        user: String,
        master_key: String,
    }

    impl PasswordManager {
        pub fn new() -> PasswordManager {
            let master_key: String = "password".to_string();
            PasswordManager {
                database: Database::new(),
                user: "".to_string(),
                master_key,
            }
        }

        pub fn login(&mut self, user: &str, login_master_key: &str) -> Result<bool, PasswordManagerError> {
            if login_master_key == self.master_key {
                self.user = user.to_string();
                self.database.connect_to_file(format!("{}.db", user).as_str());
                Ok(true)
            } else {
                Err(PasswordManagerError::new("Invalid master key"))
            }
        }

        pub fn register(&mut self, user: &str, login_master_key: &str) -> Result<(), PasswordManagerError>{
            let file_name: String = format!("{}.db", user);
            if  Path::new(file_name.as_str()).exists(){
                return Err(PasswordManagerError::new(format!("User {:?} already exists", user).as_str()));
            }
            self.database.create_file(file_name.as_str());
            self.set_master_key(login_master_key);
            Ok(())
        }

        pub fn set_master_key(&mut self, login_master_key: &str) {
            self.master_key = login_master_key.to_string();
        }

        pub fn add_password(&self, password: &Password) {
            self.database.add_password(password);
        }

        pub fn get_password(&self, platform: &str) -> Password {
            self.database.get_password(platform).unwrap()
        }
    }
}
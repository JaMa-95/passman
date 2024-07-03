use std::error::Error;
use std::fmt;
use std::path::Path;

mod frontend;
use frontend::run;

mod database;
use database::database_module::Database;
mod password;
use password::password_module::Password;

#[derive(Debug)]
struct PasswordManagerError {
    message: String,
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

struct PasswordManager {
    database: Database,
    user: String,
    master_key: String,
}

impl PasswordManager {
    fn new() -> PasswordManager {
        let master_key: String = "password".to_string();
        PasswordManager {
            database: Database::new(),
            user: "".to_string(),
            master_key,
        }
    }

    fn login(&mut self, user: &str, login_master_key: &str) -> bool {
        if login_master_key == self.master_key {
            self.user = user.to_string();
            self.database.connect_to_file(format!("{}.db", user).as_str());
            true
        } else {
            false
        }
    }

    fn register(&mut self, user: &str, login_master_key: &str) -> Result<(), PasswordManagerError>{
        let file_name: String = format!("{}.db", user);
        if  Path::new(file_name.as_str()).exists(){
            return Err(PasswordManagerError::new(format!("User {:?} already exists", user).as_str()));
        }
        self.database.create_file(file_name.as_str());
        self.set_master_key(login_master_key);
        Ok(())
    }

    fn set_master_key(&mut self, login_master_key: &str) {
        self.master_key = login_master_key.to_string();
    }

    fn add_password(&self, password: &Password) {
        self.database.add_password(password);
    }

    fn get_password(&self, platform: &str) -> Password {
        self.database.get_password(platform).unwrap()
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    run();
    let mut pm: PasswordManager = PasswordManager::new();
    let result: Result<(), PasswordManagerError> = pm.register("Jakob", "password");
    match result {
        Ok(_) => {
            println!("User registered {:?}", "Jakob");
            pm.login("Jakob", "password");
            pm.add_password(&Password::new("Jakob", "rust.org", "rustpassword"));
            let password: Password = pm.get_password("rust.org");
            println!("{:?}", password);
        }
        Err(error) => println!("{}", error),
    }
    Ok(())
}
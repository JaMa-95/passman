pub mod database_module {
    use rusqlite::{Connection, Result};
    use std::fmt;
    use std::error::Error;
    
    use crate::password::password_module::Password;

    #[derive(Debug)]
    pub struct DatabaseError {
        message: String,
    }

    impl DatabaseError {
        pub fn new(message: &str) -> DatabaseError {
            DatabaseError {
                message: message.to_string(),
            }
        }
    }

    impl From<rusqlite::Error> for DatabaseError {
        fn from(error: rusqlite::Error) -> Self {
            DatabaseError {
                message: error.to_string(),
            }
        }
    }

    impl fmt::Display for DatabaseError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.message)
        }
    }

    impl Error for DatabaseError {}

    pub struct Database {
        conn: Connection,
    }

    impl Database {
        pub fn new() -> Database {
            let conn: Connection = Connection::open(":memory:").unwrap(); 
            Database { conn }
        }

        pub fn connect_to_file(&mut self, file_name: &str) {
            self.conn = Connection::open(file_name).unwrap();
        }

        pub fn create_file(&mut self, file_name: &str) {
            self.conn = Connection::open(file_name).unwrap();
            self.conn.execute(
                "CREATE TABLE password (
                    user  TEXT NOT NULL,
                    platform  TEXT NOT NULL,
                    password  TEXT NOT NULL
                )",
                (),
            )
            .unwrap();
        }

        pub fn add_password(&self, password: &Password) {
            self.conn
                .execute(
                    "INSERT INTO password (user, platform, password) VALUES (?1, ?2, ?3)",
                    &[&password.user, &password.platform, &password.password],
                )
                .unwrap();
        }

        pub fn get_password(&self, platform: &str) -> Result<Password, DatabaseError> {
            let mut stmt = self
                .conn
                .prepare("SELECT user, platform, password FROM password WHERE platform = ?1")?;

            let _password_iter = stmt.query_map(&[&platform], |row| {
                Ok(Password {
                user: row.get(0)?,
                platform: row.get(1)?,
                password: row.get(2)?,
                })
            }).map_err(|error| DatabaseError::from(error));

            if let Some(password) = _password_iter.unwrap().next() {
                return Ok(password?);
            }

            Err(DatabaseError::new("No password found"))
        }
    }
}
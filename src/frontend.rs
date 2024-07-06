#[derive(PartialEq)]
enum Commands {
    Login,
    Logout,
    Add,
    Remove,
    Get,
    Register, 
    None, 
    Quit,
    Enter,
    Delete
}

pub mod frontend {
    use crate::manager::manager_module::{PasswordManager, PasswordManagerError};
    use crate::password::password_module::Password;
    use std::error::Error;
    use std::io::Stdout;
    use std::{io::{self, stdout}, process::Command};
    use ratatui::{
        crossterm::{
            event::{self, Event, KeyCode},
            terminal::{
                disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
            },
            ExecutableCommand,
        },
        prelude::*,
        widgets::*,
    };

    use super::Commands;
use std::thread;
use std::time::Duration;

    pub struct Frontend {
        pm: PasswordManager,
        terminal: Terminal<CrosstermBackend<Stdout>>,
    }

    impl Frontend {
        pub fn new() -> Frontend {
            Frontend {
                pm: PasswordManager::new(),
                terminal: Terminal::new(CrosstermBackend::new(stdout())).unwrap(),
            }
        }

        pub fn run(&mut self) -> io::Result<()> {
            enable_raw_mode()?;
            stdout().execute(EnterAlternateScreen)?;
            let mut should_quit = false;
            while !should_quit {
                self.terminal.draw(ui_startup)?;
                let command = self.handle_events()?;
                if command == Commands::Login {
                    println!("Login");
                    self.terminal.clear()?;
                    self.login();
                    self.show_menu();
                } else if command == Commands::Register {
                    let output = Command::new("clear").output().unwrap();
                    println!("{}", String::from_utf8_lossy(&output.stdout));
                    println!("Register");
                } else if command == Commands::Quit {
                    should_quit = true;
                }
            }
            self.terminal.clear()?;
            disable_raw_mode()?;
            stdout().execute(LeaveAlternateScreen)?;
            Ok(())
        }

        fn show_menu(&mut self) {
            let mut should_quit = false;
            while !should_quit {
                self.terminal.draw(|f| ui_startup(f)).unwrap();
                let command = self.handle_events().unwrap();
                if command == Commands::Quit {
                    should_quit = true;
                }
            }
        }

        fn handle_events(&mut self) -> io::Result<Commands> {
            if event::poll(std::time::Duration::from_millis(50))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('l') => {
                            return Ok(Commands::Login);
                        }
                        KeyCode::Char('r') => {
                            return Ok(Commands::Register);
                        }
                        KeyCode::Char('q') => {
                            return Ok(Commands::Quit);
                        }
                        KeyCode::Enter => {
                            return Ok(Commands::Enter);
                        }
                        _ => {}
                    }
                }
            }
            Ok(Commands::None)
        }

        fn login(&mut self) -> Result<bool, Box<dyn Error>>{
            let username: String = self.get_user_input("Login".to_string(), " Username: ".to_string(), false).unwrap_or_default();
            let password: String = self.get_user_input( "Login".to_string(), " Master password: ".to_string(), true).unwrap_or_default();
            let result = self.pm.login(&username, &password);
            match result {
                Ok(_) => {
                    self.terminal.draw(|f| display_(f, "Login successful", "Login"))?;
                    Ok(true)

                }
                Err(e) => {
                    self.terminal.draw(|f| display_(f, e.message.as_str(), "Login"))?;
                    thread::sleep(Duration::from_secs(2));
                    self.login()
                }
            }
        }

        fn get_user_input(&mut self, title: String, msg: String, encrypt: bool) -> Result<String, Box<dyn Error>> {
            let mut username: Input = Input::new();
            let mut should_quit = false;
            let mut msg_new = format!("{}{}", msg, username.message);
            self.terminal.draw(|f| display_(f, &msg, &title))?;
            while !should_quit {
                
                if let Ok(input) = get_input() {
                    if input == KeyCode::Enter {
                        should_quit = true;
                    } else if input == KeyCode::Backspace {
                        username.remove();
                        if (encrypt) {
                            msg_new = format!("{}{}", msg, username.message_encrypt);
                        } else {
                            msg_new = format!("{}{}", msg, username.message);
                        }
                        self.terminal.clear()?;
                        self.terminal.draw(|f| display_(f, &msg_new, &title))?;
                    } else if let KeyCode::Char(c) = input {
                        username.add(c);
                        if (encrypt) {
                            msg_new = format!("{}{}", msg, username.message_encrypt);
                        } else {
                            msg_new = format!("{}{}", msg, username.message);
                        }
                        self.terminal.clear()?;
                        self.terminal.draw(|f| display_(f, &msg_new, &title))?;
                    }
                }
            }
            Ok(username.message)
        }
    }

    fn get_input() -> io::Result<KeyCode> {
        loop {
            if event::poll(std::time::Duration::from_millis(50))? {
                if let Event::Key(key) = event::read()? {
                    return Ok(key.code);
                }
            }
        }
    }

    fn ui_startup(frame: &mut Frame) {
        frame.render_widget(
            Paragraph::new(" Login        [l] \n Register     [r] \n Quit         [q]").block(Block::bordered().title("Passman")),
            frame.size(),
        );
    }

    fn display_(frame: &mut Frame, msg: &str, title: &str) {
        frame.render_widget(
            Paragraph::new(msg).block(Block::bordered().title(title)),
            frame.size(),
        );
    }

    struct Input {
        message: String,
        message_encrypt: String
    }
    
    impl Input {
        fn new() -> Input {
            Input {
                message: "".to_string(),
                message_encrypt: "".to_string()
            }
        }
    
        fn add(&mut self, c: char) {
            self.message.push(c);
            self.message_encrypt.push('*');
        }
    
        fn remove(&mut self) {
            self.message.pop();
            self.message_encrypt.pop();
        }
    }
}

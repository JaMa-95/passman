use std::error::Error;

mod manager;
mod database;
mod password;
mod frontend;
use frontend::frontend::Frontend;

fn main() -> Result<(), Box<dyn Error>> {
    let mut frontend = Frontend::new();
    let _ = frontend.run();

    Ok(())
}
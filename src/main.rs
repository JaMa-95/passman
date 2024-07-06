use std::error::Error;

mod manager;
mod database;
mod password;
mod frontend;
use frontend::frontend::run;

fn main() -> Result<(), Box<dyn Error>> {
    run();

    Ok(())
}
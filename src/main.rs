use std::error::Error;

mod expr;
mod value;
mod r#type;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    Ok(())
}

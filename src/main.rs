mod cli;
mod color;
mod locator;
mod options;
mod worker;

use cli::Cli;
use std::{error::Error};
use color::Colorize;

pub type GenericResult<T> = Result<T, Box<dyn Error>>;

fn try_main() -> GenericResult<String>{
    let mut cli = Cli::new();
    let mut locator = cli.parse()?;
    let pattern = cli.pattern.unwrap();
    let path = cli.path.unwrap();

    
    println!(
        "Searching for {} in {}",
        pattern,
        path.display()
    );
    
    if let Some(content) = cli.inline {
        locator.search(Some(&path), Some(content))?;
        return Ok("Success".to_string());
    } else {
        locator.search(Some(&path), None)?;
    }

    locator.join_all_threads();

    Ok("Success".to_string())
}

fn main() {
    match try_main() {
        Ok(msg) => println!("{}", msg),
        Err(err) => {
            eprintln!("{err}\n", err = err.to_color(color::Color::Red));
            Cli::help();
            std::process::exit(1);
        }
    }
}

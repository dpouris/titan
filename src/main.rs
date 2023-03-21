mod cli;
mod color;
mod locator;
mod options;
mod worker;

use cli::Cli;
use std::{error::Error};

pub type GenericResult<T> = Result<T, Box<dyn Error>>;

fn main() -> GenericResult<()> {
    let mut cli = Cli::new();
    let mut locator = cli.parse().expect("could not parse arguments");
    let pattern = cli.pattern.unwrap();
    let path = cli.path.unwrap();

    
    println!(
        "Searching for {} in {}",
        pattern,
        path.display()
    );
    
    if let Some(content) = cli.inline {
        locator.search(Some(&path), Some(content))?;
        return Ok(());
    } else {
        locator.search(Some(&path), None)?;
    }

    locator.join_all_threads();

    Ok(())
}

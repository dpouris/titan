mod cli;
mod lib;

use cli::Cli;
use lib::utils::{Color, Colorize};
use std::{error::Error, time};

pub type GenericResult<T> = Result<T, Box<dyn Error>>;

fn main() -> GenericResult<()> {
    let mut cli = Cli::new();
    let mut locator = cli.parse().expect("could not parse arguments");
    let path = cli.path.unwrap();

    println!(
        "Searching for {} in {}...",
        cli.pattern.unwrap(),
        path.display()
    );

    let start = time::Instant::now();

    locator.search(&path)?;

    println!("\nLocator found {} match(es) in `{}` and took {}μs",
        locator.amount,
        &path.display(),
        start
            .elapsed()
            .as_micros()
            .to_string()
            .to_color(Color::Blue)
    );


    Ok(())
}

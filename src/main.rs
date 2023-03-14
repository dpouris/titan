mod cli;
mod lib;

use cli::Cli;
use lib::utils::{Color, Colorize};
use std::{error::Error, time::{self, Duration}, sync::atomic::{AtomicUsize, Ordering}, thread};

pub type GenericResult<T> = Result<T, Box<dyn Error>>;

static GLOBAL_THREAD_COUNT: AtomicUsize = AtomicUsize::new(0);

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

    // block until all threads are done
    while GLOBAL_THREAD_COUNT.load(Ordering::SeqCst) != 0 {
        thread::sleep(Duration::from_millis(1)); 
    }

    println!(
        "\nLocator found {} match(es) in `{}` and took {}μs",
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

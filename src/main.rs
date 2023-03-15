mod cli;
mod color;
mod locator;
mod options;

use cli::Cli;
use std::{
    error::Error,
    sync::atomic::{AtomicUsize, Ordering},
    thread,
    time::Duration,
};

pub type GenericResult<T> = Result<T, Box<dyn Error>>;

static GLOBAL_THREAD_COUNT: AtomicUsize = AtomicUsize::new(0);

fn main() -> GenericResult<()> {
    let mut cli = Cli::new();
    let mut locator = cli.parse().expect("could not parse arguments");
    let path = cli.path.unwrap();

    println!(
        "Searching for {} in {}",
        cli.pattern.unwrap(),
        path.display()
    );

    locator.search(&path)?;

    // block until all threads are done
    while GLOBAL_THREAD_COUNT.load(Ordering::SeqCst) != 0 {
        thread::sleep(Duration::from_millis(1));
    }

    Ok(())
}

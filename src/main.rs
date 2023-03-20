mod cli;
mod color;
mod locator;
mod options;
mod worker;

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
    let pattern = cli.pattern.unwrap();
    let path = cli.path.unwrap();

    if let Some(content) = cli.content {
        locator.search_content(&pattern, content);
        return Ok(());
    }

    println!(
        "Searching for {} in {}",
        pattern,
        path.display()
    );

    locator.search(&path)?;

    locator.join_all_threads();

    // block until all threads are done
    while GLOBAL_THREAD_COUNT.load(Ordering::SeqCst) != 0 {
        thread::sleep(Duration::from_millis(1));
    }

    Ok(())
}

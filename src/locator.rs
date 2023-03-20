use self::process::{process_chunk, read_chunks};
use crate::color::{Color, Colorize};
use crate::options::Options;
use crate::worker::threadpool::ThreadPool;
use crate::GenericResult;
use std::{fs::File, io::BufReader, path::PathBuf};
mod process;

const CHUNK_SIZE: usize = 1024 * 1024; // 1 MB

pub struct Locator {
    pub pattern: String,
    pub amount: usize,
    threadpool: ThreadPool,
    options: Options,
}

#[derive(Debug)]
pub struct Match {
    pub content: String,
    pub location: String,
}

impl Match {
    fn new(content: String, location: String) -> Self {
        Match { content, location }
    }
}

impl Into<String> for Match {
    fn into(self) -> String {
        format!("{}: {}", self.location, self.content.clone())
    }
}

impl FromIterator<Match> for Vec<String> {
    fn from_iter<T: IntoIterator<Item = Match>>(iter: T) -> Self {
        iter.into_iter()
            .map::<String, _>(|item| item.into())
            .collect()
    }
}

impl Locator {
    pub fn new(tp: ThreadPool, pattern: String, opts: Options) -> Locator {
        Locator {
            pattern,
            amount: 0,
            threadpool: tp,
            options: opts,
        }
    }

    pub fn search(&mut self, path: &PathBuf) -> GenericResult<()> {
        if let Some(ext) = path.extension() {
            if self
                .options
                .ignore
                .contains(&ext.to_str().unwrap().to_string())
            {
                return Ok(());
            }
        }

        if path.is_file() {
            search_file(&self.options, self.pattern.clone(), &path)?;
        } else if path.is_dir() {
            search_dir(&self.threadpool, &self.options, self.pattern.clone(), &path)?;
        }

        Ok(())
    }

    pub fn search_content(&self, pattern: &String, content: String) -> GenericResult<()> {
        let buf_reader = BufReader::new(content.as_bytes());
        let chunk_size = CHUNK_SIZE;

        let chunks = read_chunks(buf_reader, chunk_size);

        let handles = chunks
            .into_iter()
            .map(|chunk| process_chunk(&pattern, chunk, false))
            .collect::<Vec<Vec<Match>>>();

        let matches = handles.into_iter().flatten().collect::<Vec<Match>>();

        if !matches.is_empty() {
            let matches = matches.into_iter().collect::<Vec<String>>().join("\n");
            println!("\n{matches}");
        }

        Ok(())
    }

    pub fn join_all_threads(self) {
        self.threadpool.join_all()
    }
}

fn search_dir(
    tp: &ThreadPool,
    opts: &Options,
    pattern: String,
    path: &PathBuf,
) -> GenericResult<()> {
    let dir = path.read_dir();

    if let Err(reason) = &dir {
        if !opts.hide_errors {
            handle_cannot_open_path(path.display().to_color(Color::Blue), reason.to_color(Color::Red));
        }
        return Ok(());
    }

    if opts.verbose {
        println!("Directory: {dir}", dir = path.display().to_color(Color::Blue));
    }

    for path in dir? {
        let path = path?.path();
        let pathname = path.display().to_string();
        let pathname = pathname.split("/").last().unwrap();

        if !opts.hidden && pathname.starts_with('.') {
            continue;
        }

        if path.is_file() {
            if opts.is_parallel {
                let t_pattern = pattern.clone();
                let t_path = path.clone();
                let t_opts = opts.clone();

                tp.execute(move || {
                    if let Err(_) = search_file(&t_opts, t_pattern.clone(), &t_path) {
                        return;
                    }
                });
            } else {
                if let Err(_) = search_file(&opts, pattern.clone(), &path) {
                    return Ok(());
                };
            }
        }

        if path.is_dir() && opts.is_recursive {
            search_dir(&tp, &opts, pattern.clone(), &path)?;
        }
    }

    Ok(())
}

fn search_file(opts: &Options, pattern: String, path: &PathBuf) -> GenericResult<()> {
    let file = File::open(path)?;
    let buf_reader = BufReader::new(&file);
    let chunk_size = CHUNK_SIZE;

    let chunks = read_chunks(buf_reader, chunk_size);

    let handles = chunks
        .into_iter()
        .map(|chunk| process_chunk(&pattern, chunk, opts.invert_match))
        .collect::<Vec<Vec<Match>>>();

    let matches = handles.into_iter().flatten().collect::<Vec<Match>>();

    if !matches.is_empty() {
        let matches = matches.into_iter().collect::<Vec<String>>().join("\n");
        println!("\n{}\n{matches}", path.display().to_color(Color::Blue));
    }

    Ok(())
}

fn handle_cannot_open_path(pathname: String, reason: String) {
    eprintln!("{pathname}: {reason}");
}

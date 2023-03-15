use crate::color::{Color, Colorize};
use crate::options::Options;
use crate::{GenericResult, GLOBAL_THREAD_COUNT};
use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
    path::PathBuf,
    sync::atomic::Ordering,
    thread,
    time::Duration,
};

pub struct Locator {
    pub pattern: String,
    pub amount: usize,
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
        iter.into_iter().map::<String, _>(|item| item.into()).collect()
    }
}

impl Locator {
    pub fn new(pattern: String, opts: Options) -> Locator {
        Locator {
            pattern,
            amount: 0,
            options: opts,
        }
    }

    pub fn search(&mut self, path: &PathBuf) -> GenericResult<()> {
        if let Some(ext) = path.extension() {
            if self.options.ignore.contains(&ext.to_str().unwrap().to_string()) {
                return Ok(());
            }
        }

        if path.is_file() {
            search_file(self.pattern.clone(), &path)?;
        } else if path.is_dir() {
            search_dir(&self.options, self.pattern.clone(), &path)?;
        }

        Ok(())
    }
}

fn search_dir(options: &Options, pattern: String, dir_path: &PathBuf) -> GenericResult<()> {
    // if file extension is in options.ignore

    let dir = dir_path.read_dir();

    if let Err(reason) = &dir {
        handle_cannot_open_path(dir_path.display().to_string(), reason.to_string());
        return Ok(());
    }

    for path in dir? {
        let path = path?.path();
        let pathname = path.display().to_string();
        let pathname = pathname.split("/").last().unwrap();

        if !options.hidden && pathname.starts_with('.') {
            continue;
        }

        if path.is_file() {
            if options.is_parallel {
                let t_pattern = pattern.clone();
                let t_path = path.clone();
                // if active threads surpass 20 block from spawning a new one.
                // like a queue
                while GLOBAL_THREAD_COUNT.load(Ordering::SeqCst) > 30 {
                    thread::sleep(Duration::from_millis(1));
                }
                // add 1 to thread count
                GLOBAL_THREAD_COUNT.fetch_add(1, Ordering::SeqCst);
                thread::spawn(move || {
                    if let Err(_) = search_file(t_pattern.clone(), &t_path) {
                        GLOBAL_THREAD_COUNT.fetch_sub(1, Ordering::SeqCst);
                        return;
                    }
                    // remove 1 from thread count
                    GLOBAL_THREAD_COUNT.fetch_sub(1, Ordering::SeqCst);
                });
            } else {
                if let Err(_) = search_file(pattern.clone(), &path) {
                    return Ok(());
                };
            }
        }

        if path.is_dir() && options.is_recursive {
            search_dir(&options, pattern.clone(), &path)?;
        }
    }

    Ok(())
}

fn read_chunks<R: Read>(mut reader: BufReader<R>, chunk_size: usize) -> Vec<String> {
    let mut chunks = vec![];

    let mut chunk = String::with_capacity(chunk_size);
    let bytes_read = reader.read_line(&mut chunk);
    if let Err(_) = bytes_read {
        return vec![];
    }
    let mut bytes_read = bytes_read.unwrap();
    while bytes_read > 0 {
        let remaining_capacity = chunk_size - chunk.len();
        if remaining_capacity < bytes_read {
            // If the remaining capacity in the chunk is not enough to hold the entire next line, split the line
            let split_pos = remaining_capacity
                + chunk.as_bytes()[remaining_capacity..]
                    .iter()
                    .position(|b| *b == b'\n')
                    .unwrap_or(bytes_read - remaining_capacity);
            let rest = chunk.split_off(split_pos);
            chunks.push(chunk);
            chunk = rest;
        }

        bytes_read = match reader.read_line(&mut chunk) {
            Err(_) => 0,
            Ok(bytes) => bytes,
        }
    }

    // Add the last chunk if there is any remaining data
    if chunk.len() > 0 {
        chunks.push(chunk);
    }

    chunks
}

fn process_chunk(pattern: String, chunk: String) -> Vec<Match> {
    let mut matches = vec![];
    let mut line_idx = 0;
    for line in chunk.lines() {
        line_idx += 1;
        if !line.contains(&pattern) {
            continue;
        }

        let line = line.replace(&pattern, &pattern.to_color(Color::Red));
        let new_match = Match::new(
            line.to_owned(),
            line_idx.to_color(Color::Yellow),
        );

        matches.push(new_match)
    }
    matches
}

fn search_file(pattern: String, file_path: &PathBuf) -> GenericResult<()> {
    let file = File::open(file_path)?;
    let buf_reader = BufReader::new(&file);
    let chunk_size = 1024 * 1024; // 1 MB

    let chunks = read_chunks(buf_reader, chunk_size);

    let handles = chunks
        .into_iter()
        .map(|chunk| {
            let pattern_clone = pattern.clone();
            thread::spawn(move || process_chunk(pattern_clone, chunk))
        })
        .collect::<Vec<_>>();

    let matches = handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .flatten()
        .collect::<Vec<Match>>();

    if !matches.is_empty() {
        let matches = matches
            .into_iter()
            .collect::<Vec<String>>()
            .join("\n");
        println!("\n{}\n{matches}", file_path.display().to_color(Color::Blue));
    }

    Ok(())
}

fn handle_cannot_open_path(pathname: String, reason: String) {
    println!("{pathname}: {reason}");
}

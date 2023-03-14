use super::utils::{Color, Colorize};
use crate::{cli::ArgKey, GenericResult, GLOBAL_THREAD_COUNT};
use regex::Regex;
use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    os::unix::prelude::MetadataExt,
    path::PathBuf, thread, sync::atomic::Ordering, time::Duration,
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

impl Display for Match {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("#{}: {}", self.location, self.content.clone()))
    }
}

#[derive(Debug, Clone)]
pub struct Options {
    pub is_parallel: bool,
    pub ignore: Vec<String>,
    pub include: Vec<String>,
    pub is_recursive: bool,
    pub is_verbose: bool,
    pub show_help: bool,
    pub hidden: bool,
}

impl Options {
    pub fn new() -> Self {
        Options {
            is_parallel: true,
            ignore: vec![],
            include: vec![],
            is_recursive: false,
            is_verbose: false,
            show_help: false,
            hidden: false,
        }
    }

    pub fn update<'a>(&mut self, arg: &'a ArgKey) {
        match arg {
            ArgKey::Short(short_key) => match *short_key {
                "r" => self.is_recursive = true,
                "v" => self.is_verbose = true,
                _ => {}
            },
            ArgKey::Long(long_key) => match *long_key {
                "single-thread" => self.is_parallel = false,
                "recursive" => self.is_recursive = true,
                "verbose" => self.is_verbose = true,
                "help" => self.show_help = true,
                "hidden" => self.hidden = true,
                _ => {}
            },
            ArgKey::LongWithArgs((long_key, options)) => match *long_key {
                "include" => self.include = options.iter().map(|str| str.to_string()).collect(),
                "ignore" => self.ignore = options.iter().map(|str| str.to_string()).collect(),
                _ => {}
            },
        };
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
        if path.is_file() {
            search_file(self.pattern.clone(), &path)?;
        } else if path.is_dir() {
            search_dir(&self.options, self.pattern.clone(), &path)?;
        }

        Ok(())
    }
}

fn search_dir(options: &Options, pattern: String, dir_path: &PathBuf) -> GenericResult<()> {
    let options = options.clone();
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
                while GLOBAL_THREAD_COUNT.load(Ordering::SeqCst) > 20 {
                    thread::sleep(Duration::from_millis(1));
                }
                // add 1 to thread count
                GLOBAL_THREAD_COUNT.fetch_add(1, Ordering::SeqCst);
                thread::spawn(move || {
                    search_file(t_pattern.clone(), &t_path).expect("could not search file");
                    // remove 1 from thread count
                    GLOBAL_THREAD_COUNT.fetch_sub(1, Ordering::SeqCst);
                });
            } else {
                search_file(pattern.clone(), &path)?;
            }
        }

        if path.is_dir() && options.is_recursive {
            search_dir(&options, pattern.clone(), &path)?;
        }
    }

    Ok(())
}

fn search_file(pattern: String, file_path: &PathBuf) -> GenericResult<()> {
    let file = File::open(file_path);
    if let Err(reason) = file {
        handle_cannot_open_path(file_path.display().to_string(), reason.to_string());
        return Ok(());
    }

    let file = file?;
    let file_len = file.metadata()?.size() as usize;
    let file_buffer = BufReader::with_capacity(file_len, file);
    let mut matches: Vec<Match> = vec![];

    for (idx, line) in file_buffer.lines().enumerate() {
        if let Err(_) = line {
            continue;
        }
        let line = line?;
        if !line.contains(&pattern) {
            continue;
        }
        
        let idx_of_match = line.find(&pattern).unwrap();
        let pattern_match = line.split_at(idx_of_match).1.split_at(pattern.len()).0;
        // self.amount += line.matches(&pattern).collect::<Vec<&str>>().len();
        let line = line.replace(&pattern, &pattern_match.to_color(Color::Red));
        let new_match = Match::new(
            line.trim().to_string(),
            (idx + 1).to_string().as_str().to_color(Color::Yellow),
        );
        matches.push(new_match);
    }

    if matches.len() > 0 {
        println!(
            "\n{}",
            file_path.display().to_string().to_color(Color::Blue)
        );
        let matches = matches
            .iter()
            .map(|m| format!("{m}"))
            .collect::<Vec<String>>()
            .join("\n");
        println!("{matches}");
    }

    Ok(())
}

fn handle_cannot_open_path(pathname: String, reason: String) {
    println!("{pathname}: {reason}");
}

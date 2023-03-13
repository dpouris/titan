use crate::{GenericResult, cli::ArgKey};
use regex::Regex;
use super::utils::{Color, Colorize};
use std::{
    fmt::Display,
    io::{BufRead, BufReader},
    os::unix::prelude::MetadataExt,
    path::PathBuf,
    str, fs::File, thread::{self},
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

#[derive(Debug)]
pub struct Options {
    pub is_parallel: bool,
    pub ignore: Option<Vec<String>>,
    pub include: Option<Vec<String>>,
    pub is_recursive: bool,
    pub show_help: bool,
    pub hidden: bool
}

impl Options {
    pub fn new() -> Self {
        Options {
            is_parallel: true,
            ignore: None,
            include: None,
            is_recursive: false,
            show_help: false,
            hidden: false
        }
    }

    pub fn update<'a>(&mut self, arg: &'a ArgKey) {
        match arg {
            ArgKey::Short(short_key) => match *short_key {
                "r" => self.is_recursive = true,
                _ => {}
            },
            ArgKey::Long(long_key) => match *long_key {
                "single-thread" => self.is_parallel = false,
                "recursive" => self.is_recursive = true,
                "help" => self.show_help = true,
                "hidden" => self.hidden = true,
                _ => {}
            },
            ArgKey::LongWithArgs((long_key, options)) => match *long_key {
                "include" => self.include = Some(options.iter().map(|str| str.to_string()).collect()),
                "ignore" => self.ignore = Some(options.iter().map(|str| str.to_string()).collect()),
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
            self.search_file(&path)?;
        } else if path.is_dir() {
            self.search_dir(&path)?;
        }

        Ok(())
    }

    fn search_dir(&mut self, dir_path: &PathBuf) -> GenericResult<()> {
        let dir = dir_path.read_dir();

        if let Err(reason) = &dir {
            handle_cannot_open_path(dir_path.display().to_string(), reason.to_string());
            return Ok(());
        }

        for path in dir? {
            let path = path?.path();
            let pathname = path.display().to_string();
            let pathname = pathname.split("/").last().unwrap();

            if !self.options.hidden && pathname.starts_with('.') { continue }
            
            if self.options.is_parallel {
                todo!() // implement threads most likely extract the search functionality outside of Locator so that
                // we don't borrow self for the lifetime of the new thread which might outlive the main thread
            } else {
                if path.is_file() {
                    self.search_file(&path)?;
                }
                
                if path.is_dir() && self.options.is_recursive {
                    self.search_dir(&path)?;
                }
            }

        }

        Ok(())
    }

    fn search_file(&mut self, file_path: &PathBuf) -> GenericResult<()> {
        let file = File::open(file_path);
        if let Err(reason) = file {
            handle_cannot_open_path(file_path.display().to_string(), reason.to_string());
            return Ok(());
        }

        let file = file?;
        let file_len = file.metadata()?.size() as usize;
        let file_buffer = BufReader::with_capacity(file_len, file);
        let mut matches = vec![];
        
        for (idx, line) in file_buffer.lines().enumerate() {
            if let Err(_) = line {
                continue;
            }
            let line = line?;
            if !line.contains(&self.pattern) {
                continue;
            }
            let idx_of_match = line.find(&self.pattern).unwrap();
            let pattern_match = line.split_at(idx_of_match).1.split_at(self.pattern.len()).0;
            self.amount += line.matches(&self.pattern).collect::<Vec<&str>>().len();
            let line = line.replace(&self.pattern, &pattern_match.to_color(Color::Red));
            let new_match = Match::new(
                line.trim().to_string(),
                (idx + 1).to_string().as_str().to_color(Color::Yellow),
            );
            matches.push(new_match);
        }

        if matches.len() > 0 {
            println!("\n{}", file_path.display().to_string().to_color(Color::Blue));
            let matches = matches.iter().map(|m| format!("{m}")).collect::<Vec<String>>().join("\n");
            println!("{matches}");
        }

        Ok(())
    }
}

fn handle_cannot_open_path(pathname: String, reason: String) {
    println!("{pathname}: {reason}");
}
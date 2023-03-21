use std::io::{self, BufRead};
use std::{env::args, path::PathBuf};

use crate::worker::threadpool::ThreadPool;
use crate::GenericResult;
use crate::{locator::Locator, options::Options};

pub use self::parser::{parse_args, ArgKey};

extern crate atty;

pub struct Cli {
    pub pattern: Option<String>,
    pub path: Option<PathBuf>,
    pub inline: Option<String>
}

mod parser;

impl Cli {
    pub fn new() -> Self {
        Cli {
            pattern: None,
            path: None,
            inline: None
        }
    }

    pub fn parse(&mut self) -> GenericResult<Locator> {
        let (pattern, path, parsed_args) = parse_args(args());
        let mut options = Options::new();

        for arg in parsed_args {
            options.update(&arg);
        }

        if options.show_help {
            Self::help();
            Self::exit(None)
        }

        if pattern.is_none() && path.is_none() {
            Self::exit(Some("missing <pattern> argument"))
        }

        let path = match path {
            Some(path) => path,
            None => {
                if !atty::is(atty::Stream::Stdin) {
                    let content: Vec<String> = io::stdin()
                        .lock()
                        .lines().filter(|line| line.is_ok()).map(|line| line.unwrap()).collect();
                    self.inline = Some(String::from(content.join("\n")));
                } 
                PathBuf::from(".")
            }
        };


        if !path.is_file() && !path.is_dir() {
            let reason = format!(
                "`{}` is neither a valid file path nor a valid dir path",
                path.display()
            );
            Self::exit(Some(&reason))
        }

        let pattern = pattern.unwrap();

        self.pattern = Some(pattern.clone());
        self.path = Some(path);

        Ok(Locator::new(
            ThreadPool::new(17),
            pattern,
            options,
        ))
    }

    pub fn exit(reason: Option<&str>) -> ! {
        use std::process;
        if let Some(reason) = reason {
            println!("\n{reason}");
        }
        process::exit(1);
    }

    pub fn help() {
        println!(
            "
USAGE:
    grile <pattern> <path>

OPTIONS:
"
        );
    }
}

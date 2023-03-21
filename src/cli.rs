use std::io::{self, BufRead};
use std::{env::args, path::PathBuf};

use regex::{RegexBuilder};

use crate::worker::threadpool::ThreadPool;
use crate::GenericResult;
use crate::{locator::Locator, options::Options};

pub use self::parser::{parse_flags, ArgKey};

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
        let (pattern, path, parsed_args) = parse_flags(args());
        let mut options = Options::new();

        for arg in parsed_args {
            options.update(&arg);
        }

        if options.show_help {
            Self::help();
            Self::exit(None)
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
        let pattern = format!(r"{}", &pattern);
        let mut builder = RegexBuilder::new(&pattern);
        let re_pattern = builder.case_insensitive(options.is_case_insensitive).build().unwrap();

        self.pattern = Some(pattern.clone());
        self.path = Some(path);

        Ok(Locator::new(
            ThreadPool::new(17),
            re_pattern,
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
"USAGE:
    titan <PATTERN> [FILES] [FLAGS] [OPTIONS]
        
FLAGS:
    -i, --invesensitive    Perform case-insensitive matching
    -r, --recursive           Search directories recursively
    -v, --invert_match           Select non-matching lines
    -h, --help              Show this help message and exit
    --hidden                 Search hidden files and directories
    -x, --show_errors            Do not display error messages
    --verbose                Show additional information during execution
    --single-thread            Disable parallel execution
        
OPTIONS:
    --ignore <PATTERN>...           Exclude files or directories matching the specified pattern(s)
    --include <EXTENSION>...    Search files with the specified extension(s)
        
ARGS:
    <PATTERN>    Specify the regex pattern to match
    <FILES>      Specify the file(s) or directory(ies) to search (optional) "
        );
    }
}

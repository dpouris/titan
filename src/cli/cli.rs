use crate::lib::{Locator, LocatorOptions};
use crate::GenericResult;

use super::parser::parse_args;
use std::{env::args, path::PathBuf};
pub struct Cli {
    pub pattern: Option<String>,
    pub path: Option<PathBuf>,
}

impl Cli {
    pub fn new() -> Self {
        Cli {
            pattern: None,
            path: None,
        }
    }

    pub fn parse(&mut self) -> GenericResult<Locator> {
        let arguments = &args().collect::<Vec<String>>()[1..];
        let parsed_args = parse_args(arguments);
        let mut options = LocatorOptions::new();

        for arg in parsed_args {
            options.update(&arg);
        }

        if options.show_help {
            Self::help();
            Self::exit(None)
        }

        if arguments.len() == 0 {
            Self::exit(Some("missing <pattern> and <path> arguments"))
        }

        if arguments.len() == 1 {
            Self::exit(Some("missing <path> argument"))
        }

        let path = PathBuf::from(arguments[1].to_owned());

        if !path.is_file() && !path.is_dir() {
            let reason = format!(
                "`{}` is neither a valid file path nor a valid dir path",
                arguments[1]
            );
            Self::exit(Some(&reason))
        }

        self.pattern = Some(arguments[0].to_owned());
        self.path = Some(path);

        Ok(Locator::new(arguments[0].to_owned(), options))
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

use std::{env::Args, path::PathBuf};

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum ArgKey {
    Short(String),
    Long(String),
    LongWithArgs((String, Vec<String>)),
}

// State machine for parsing arguments
#[derive(Debug, Eq, PartialEq)]
enum State {
    Pattern,
    Path,
    Options
}

pub fn parse_args(args: Args) -> (Option<String>, Option<PathBuf>, Vec<ArgKey>) {
    // parse the arguments with a state machine
    let mut state = State::Pattern;
    let mut pattern = None;
    let mut path = None;
    let mut parsed_flags = vec![];

    for arg in args.skip(1) {
        match state {
            State::Pattern => {
                pattern = Some(arg);
                state = State::Path;
            }
            State::Path => {
                if arg.starts_with("-") {
                    state = State::Options;
                    parsed_flags.push(parse_arg(&arg));
                } else {
                    path = Some(PathBuf::from(arg));
                    state = State::Options;
                }
            }
            State::Options => {
                parsed_flags.push(parse_arg(&arg))
            }
        }
    }

    (pattern, path, parsed_flags)
}

fn parse_arg(arg: &str) -> ArgKey {
    if arg.starts_with("--") {
        let stripped_flag = arg.strip_prefix("--").unwrap();
        if arg.contains("=") {
            let flag = stripped_flag.split("=").collect::<Vec<&str>>();
            let value = flag
                .last()
                .unwrap()
                .split(",")
                .map(|str| str.trim())
                .map(|str| str.to_string())
                .collect::<Vec<String>>();
            return ArgKey::LongWithArgs((flag[0].to_owned(), value));
        } else {
            return ArgKey::Long(stripped_flag.to_string());
        }
    } 
    return ArgKey::Short(arg[1..].to_owned());
}


// #[derive(Debug, Eq, Hash, PartialEq)]
// pub enum ArgKey<'args> {
//     Short(&'args str),
//     Long(&'args str),
//     LongWithArgs((&'args str, Vec<&'args str>)),
// }

// pub fn parse_args<'args>(args: &'args [String]) -> Vec<ArgKey> {
//     let mut parsed_flags = vec![];
//     for flag in args.iter() {
//         if flag.starts_with("--") {
//             let stripped_flag = flag.strip_prefix("--").unwrap();
//             if flag.contains("=") {
//                 let flag = stripped_flag.split("=").collect::<Vec<&str>>();
//                 let value = flag
//                     .last()
//                     .unwrap()
//                     .split(",")
//                     .map(|str| str.trim())
//                     .collect::<Vec<&str>>();
//                 parsed_flags.push(ArgKey::LongWithArgs((flag.first().unwrap(), value)));
//                 continue;
//             }
//             parsed_flags.push(ArgKey::Long(stripped_flag));
//         } else if flag.starts_with("-") && flag.len() == 2 {
//             let key = ArgKey::Short(flag.strip_prefix("-").unwrap());
//             parsed_flags.push(key);
//         }
//     }

//     parsed_flags
// }
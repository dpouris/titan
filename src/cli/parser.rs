use std::{env::Args, path::PathBuf, iter::Skip};

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

pub fn parse_flags(args: Args) -> (Option<String>, Option<PathBuf>, Vec<ArgKey>) {
   // parse the arguments with a state machine
   let mut state = State::Pattern;
   let mut pattern = None;
   let mut path = None;
   let mut parsed_flags = vec![];

   let mut args_iter = args.skip(1);

   while let Some(arg) = parse_arg(&mut args_iter) {
       match state {
           State::Pattern => {
               match arg {
                   ArgKey::Long(s) => {
                       pattern = Some(s);
                       state = State::Path;
                   },
                   ArgKey::Short(s) => {
                       pattern = Some(s);
                       state = State::Path;
                   },
                   ArgKey::LongWithArgs(_) => {
                       parsed_flags.push(arg);
                   }
               }
           },
           State::Path => {
               match arg {
                   ArgKey::Long(s) => {
                       path = Some(PathBuf::from(s));
                       state = State::Options;
                   },
                   ArgKey::Short(s) => {
                       path = Some(PathBuf::from(s));
                       state = State::Options;
                   },
                   ArgKey::LongWithArgs(_) => {
                       state = State::Options;
                       parsed_flags.push(arg);
                   }
               }
           },
           State::Options => {
               parsed_flags.push(arg);
           }
       }
   }

   (pattern, path, parsed_flags)
}

fn parse_arg(args: &mut Skip<Args>) -> Option<ArgKey> {
    if let Some(arg) = args.next() {
        if arg.starts_with("--") {
            let stripped_flag = arg.strip_prefix("--").unwrap();
            if let Some(pos) = stripped_flag.find('=') {
                let flag_name = stripped_flag[..pos].to_owned();
                let values = stripped_flag[pos+1..].split(',').map(|s| s.trim().to_owned()).collect();
                return Some(ArgKey::LongWithArgs((flag_name, values)));
            } else if let Some(value) = args.next() {
                return Some(ArgKey::LongWithArgs((stripped_flag.to_owned(), vec![value])));
            } else {
                return Some(ArgKey::Long(stripped_flag.to_owned()));
            }
        } else {
            return Some(ArgKey::Short(arg[1..].to_owned()));
        }
    }
    None
}
use std::path::PathBuf;

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum ArgKey {
    Short(String),
    Long(String),
    LongWithArgs((String, Vec<String>)),
}

pub fn parse_flags(args: std::env::Args) -> Result<(String, Option<PathBuf>, Vec<ArgKey>), String> {
    let mut args_iter = args.skip(1);
    let mut pattern = String::new();
    let mut path = None;

    // Check for the "--help" flag
    if let Some(arg) = args_iter.next() {
        if arg == "--help" {
            return Ok((pattern, path, vec![ArgKey::Long(String::from("help"))]));
        } else {
            pattern = arg;
        }
    } else {
        return Err(String::from("Missing required argument: pattern"));
    }

    let mut parsed_flags = vec![];

    if let Some(arg) = args_iter.next() {
        if !arg.starts_with('-') {
            path = Some(PathBuf::from(arg));
        } else {
            let mut path_iter = std::iter::once(arg.clone());
            while let Some(arg) = parse_arg(&mut path_iter) {
                parsed_flags.push(arg);
            }
        }
    }
    while let Some(arg) = parse_arg(&mut args_iter) {
        parsed_flags.push(arg);
    }
    
    Ok((pattern, path, parsed_flags))
}


fn parse_arg(args: &mut impl Iterator<Item = String>) -> Option<ArgKey> {
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
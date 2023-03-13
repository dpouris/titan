#[derive(Debug, Eq, Hash, PartialEq)]
pub enum ArgKey<'args> {
    Short(&'args str),
    Long(&'args str),
    LongWithArgs((&'args str, Vec<&'args str>)),
}

pub fn parse_args<'args>(args: &'args [String]) -> Vec<ArgKey> {
    let mut parsed_flags = vec![];
    for flag in args.iter() {
        if flag.starts_with("--") {
            let stripped_flag = flag.strip_prefix("--").unwrap();
            if flag.contains("=") {
                let flag = stripped_flag.split("=").collect::<Vec<&str>>();
                let value = flag
                    .last()
                    .unwrap()
                    .split(",")
                    .map(|str| str.trim())
                    .collect::<Vec<&str>>();
                parsed_flags.push(ArgKey::LongWithArgs((flag.first().unwrap(), value)));
                continue;
            }
            parsed_flags.push(ArgKey::Long(stripped_flag));
        } else if flag.starts_with("-") && flag.len() == 2 {
            let key = ArgKey::Short(flag.strip_prefix("-").unwrap());
            parsed_flags.push(key);
        }
    }

    parsed_flags
}

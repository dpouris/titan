use crate::cli::ArgKey;

#[derive(Debug, Clone)]
pub struct Options {
    pub is_parallel: bool,
    pub ignore: Vec<String>,
    pub include_ext: Vec<String>,
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
            include_ext: vec![],
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
                "include" => self.includes(options.iter().map(|str| str.to_string()).collect()),
                "ignore" => self.ignores(options.iter().map(|str| str.to_string()).collect()),
                _ => {}
            },
        };
    }

    pub fn ignores(&mut self, ignore_list: Vec<String>) {
        self.ignore.extend(ignore_list);
    }

    pub fn includes(&mut self, include_list: Vec<String>) {
        self.include_ext.extend(include_list);
    }
}

use crate::cli::ArgKey;

#[derive(Debug, Clone)]
pub struct Options {
    pub is_parallel: bool,
    pub ignore: Vec<String>,
    pub include_ext: Vec<String>,
    pub is_recursive: bool,
    pub invert_match: bool,
    pub show_help: bool,
    pub hidden: bool,
    pub show_errors: bool,
    pub verbose: bool,
    pub ignore_case: bool,
}

impl Options {
    pub fn new() -> Self {
        Options {
            is_parallel: true,
            ignore: vec![],
            include_ext: vec![],
            is_recursive: false,
            invert_match: false,
            show_help: false,
            hidden: false,
            show_errors: false,
            verbose: false,
            ignore_case: false,
        }
    }

    pub fn update<'a>(&mut self, arg: &'a ArgKey) {
        match arg {
            ArgKey::Short(short_key) => match short_key.as_str() {
                "r" => self.is_recursive = true,
                "v" => self.invert_match = true,
                "x" => self.show_errors = true,
                "i" => self.ignore_case = true,
                "h" => self.show_help = true,
                _ => {}
            },
            ArgKey::Long(long_key) => match long_key.as_str() {
                "single-thread" => self.is_parallel = false,
                "recursive" => self.is_recursive = true,
                "invert-match" => self.invert_match = true,
                "help" => self.show_help = true,
                "hidden" => self.hidden = true,
                "show-errors" => self.show_errors = true,
                "verbose" => self.verbose = true,
                "ignore-case" => self.ignore_case = true,
                _ => {}
            },
            ArgKey::LongWithArgs((long_key, options)) => match long_key.as_str() {
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

# TITAN - Tiny and Intuitive Text Analysis Tool for Rust-based Patterns

TITAN is a command-line program written in Rust that allows you to quickly search for patterns in large text files and directories. TITAN is designed to be faster than traditional search tools like grep, and it is built entirely in Rust, making it a lightweight and efficient option for text analysis.

## Installation

To install TITAN, you can use the following command to clone the repository and run the included installation script:

```sh
$ git clone --depth 1 https://github.com/dpouris/titan.git
$ cd titan
$ ./install
```

This will download the latest version of TITAN from the official GitHub repository and install it on your system.

## Usage

To use TITAN, you can enter the following command:

```sh
$ titan <PATTERN> [FILES or DIRECTORY] [FLAGS] [OPTIONS]
```

Where `PATTERN` is the regular expression or pattern you want to search for, and `FILE` or `DIRECTORY` is the name of the file (or directory) you want to search, **if empty it is implied that TITAN will search the current working directory**. If you specify a directory, TITAN will search all files in that directory but not its subdirectories unless the `-r, --recursive` flag is specified.

TITAN supports a number of flags and options that allow you to customize the behavior of the program, including:

## Flags
 - `-i, --ingore-case`:           Perform case-insensitive matching
 - `-r, --recursive`:             Search directories recursively
 - `-v, --invert_match`:          Select non-matching lines
 - `-h, --help`:                  Show this help message and exit
 - `--hidden`:                    Search hidden files and directories
 - `-x, --show_errors`:           Do not display error messages
 - `--verbose`:                   Show additional information during execution
 - `--single-thread`:             Disable parallel execution

## Options
- `--ignore <PATTERN>...`       Exclude files or directories matching the specified pattern(s)
- `--include <EXTENSION>...`    Search files with the specified extension(s)

For more information on how to use these options, you can run the following command:

```sh
$ titan --help
```

## Performance

TITAN is built entirely in Rust, making it a lightweight and efficient option for text analysis. While it doesn't use advanced algorithms to achieve better speed, Rust's speed and efficiency make it possible to search large text files quickly and efficiently.

## License

TITAN is released under the MIT License. For more information on the MIT License, please see the LICENSE.md file in the project repository.

## Contact

If you have any questions or comments about TITAN, you can contact me at `jimpouris0@gmail.com`
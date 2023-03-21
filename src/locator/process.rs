use std::io::{BufRead, BufReader, Read};

use regex::Regex;

use crate::color::{Color, Colorize};

use super::Match;

pub(crate) fn read_chunks<R: Read>(mut reader: BufReader<R>, chunk_size: usize) -> Vec<String> {
    let mut chunks = vec![];

    let mut chunk = String::with_capacity(chunk_size);
    let bytes_read = reader.read_line(&mut chunk);
    if let Err(_) = bytes_read {
        return vec![];
    }
    let mut bytes_read = bytes_read.unwrap();
    while bytes_read > 0 {
        let remaining_capacity = chunk_size - chunk.len();
        if remaining_capacity < bytes_read {
            // If the remaining capacity in the chunk is not enough to hold the entire next line, split the line
            let split_pos = remaining_capacity
                + chunk.as_bytes()[remaining_capacity..]
                    .iter()
                    .position(|b| *b == b'\n')
                    .unwrap_or(bytes_read - remaining_capacity);
            let rest = chunk.split_off(split_pos);
            chunks.push(chunk);
            chunk = rest;
        }

        bytes_read = match reader.read_line(&mut chunk) {
            Err(_) => 0,
            Ok(bytes) => bytes,
        }
    }

    // Add the last chunk if there is any remaining data
    if chunk.len() > 0 {
        chunks.push(chunk);
    }

    chunks
}

pub(crate) fn process_chunk(pattern: &Regex, chunk: String, invert_match: bool) -> Vec<Match> {
    let mut matches = Vec::with_capacity(chunk.lines().count());
    let mut line_idx = 0;
    // use regex to find matches and replace the matches with colored text
    for line in chunk.lines() {
        line_idx += 1;
        if pattern.is_match(line) && !invert_match {
            let content = pattern.replace_all(line, |caps: &regex::Captures| {
                                                        caps[0].to_color(Color::Red).to_string()
                                                    })
                                                    .to_string();
            matches.push(Match::new(content, line_idx.to_color(Color::Yellow)));
        } else if !pattern.is_match(line) && invert_match {
            matches.push(Match::new(line.to_color(Color::Red), line_idx.to_color(Color::Yellow)));
        }
    }

    matches
}

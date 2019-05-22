use lazy_static::lazy_static;
use regex::{Captures, Regex};

pub mod constants;

pub fn extract_header(line: &str) -> Option<Captures> {
    lazy_static! {
        static ref NAME_RE: Regex = Regex::new(
            r"Name: (?P<name>.+) Training Program: (?P<training_program>\d+) ID Number: (?P<id>\d+)",
        )
        .unwrap();
    }

    NAME_RE.captures(line)
}

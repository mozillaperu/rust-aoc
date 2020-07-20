use std::{collections::HashSet, fs};
#[path="common.rs"]
mod common;

pub fn run() -> i32 {
    let frequencies = common::read_input().unwrap();

    let mut seen = HashSet::new();
    seen.insert(0);

    let mut current = 0;
    loop {
        for frequency in frequencies.iter() {
            current += frequency;
            if seen.contains(&current) {
                return current;
            }
            seen.insert(current);
        }
    }
}

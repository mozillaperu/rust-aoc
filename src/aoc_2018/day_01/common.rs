use std::{fs, num};

pub fn read_input() -> Result<Vec<i32>, num::ParseIntError>{
    fs::read_to_string("src/aoc_2018/day_01/input.txt").unwrap()
        .lines()
        .map(|line| line.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()
}

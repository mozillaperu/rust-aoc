use std::fs;

pub fn run() {
    let result = fs::read_to_string("day_1.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .sum::<i32>();
    println!("{:?}", result);
}

use std::fs;

pub fn run() {
    let frequencies = fs::read_to_string("day_1.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

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

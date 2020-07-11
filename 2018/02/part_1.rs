use std::{collections::HashMap, fs};

fn frequency_table(word: String) -> HashMap<char, i32> {
    let mut freq = HashMap::new(); // Character -> count
    for character in word.chars() {
        if !freq.contains_key(&character) {
            freq.insert(character.clone(), 0);
        }
        let count = freq.get_mut(&character).unwrap();
        *count += 1;
    }
    freq
}

fn analyze_word(word: String) -> (bool, bool) {
    let frequency_table = frequency_table(word);
    let two_count = frequency_table.values().any(|val| *val == 2);
    let three_count = frequency_table.values().any(|val| *val == 3);
    return (two_count, three_count);
}

pub fn run() {
    let results = fs::read_to_string("day_2.txt")
        .unwrap()
        .lines()
        .map(|line| analyze_word(line.to_string()))
        .collect::<Vec<(bool, bool)>>();
    let two_counts = results.iter().filter(|(two_count, _)| *two_count).count();
    let three_counts = results.iter().filter(|(_, three_count)| *three_count).count();
    println!("{:?}", two_counts* three_counts);
}

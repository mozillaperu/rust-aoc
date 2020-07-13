use std::{fs, num};

fn resulting_frequency(input: String) -> Result<i32, num::ParseIntError> {
    Ok(input
        .lines()
        .map(|line| line.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .sum())
}

pub fn run() -> Result<(), num::ParseIntError> {
    let input = fs::read_to_string("src/aoc_2018/day_01/input.txt").unwrap();
    let result = resulting_frequency(input)?;
    println!("{:?}", result);
    Ok(())
}

#[test]
fn first_input_returns_3() -> Result<(), num::ParseIntError> {
    let input = "+1\n+1\n+1".to_string();
    let actual = resulting_frequency(input)?;
    assert_eq!(3, actual);
    Ok(())
}

#[test]
fn second_input_returns_0() -> Result<(), num::ParseIntError> {
    let input = "+1\n+1\n-2".to_string();
    let actual = resulting_frequency(input)?;
    assert_eq!(0, actual);
    Ok(())
}

#[test]
fn third_input_returns_minus_6() -> Result<(), num::ParseIntError> {
    let input = "-1\n-2\n-3".to_string();
    let actual = resulting_frequency(input)?;
    assert_eq!(-6, actual);
    Ok(())
}

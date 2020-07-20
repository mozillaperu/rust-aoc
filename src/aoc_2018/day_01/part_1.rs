use std::num;
#[path="common.rs"]
mod common;

fn resulting_frequency(input: Vec<i32>) -> Result<i32, num::ParseIntError> {
    Ok(input
        .iter()
        .sum())
}

pub fn run() -> Result<(), num::ParseIntError> {
    let input = common::read_input().unwrap();
    let result = resulting_frequency(input)?;
    println!("{:?}", result);
    Ok(())
}

#[test]
fn first_input_returns_3() -> Result<(), num::ParseIntError> {
    let input = vec![1,1,1];
    let actual = resulting_frequency(input)?;
    assert_eq!(3, actual);
    Ok(())
}

#[test]
fn second_input_returns_0() -> Result<(), num::ParseIntError> {
    let input = vec![1, 1, -2];
    let actual = resulting_frequency(input)?;
    assert_eq!(0, actual);
    Ok(())
}

#[test]
fn third_input_returns_minus_6() -> Result<(), num::ParseIntError> {
    let input = vec![-1,-2,-3];
    let actual = resulting_frequency(input)?;
    assert_eq!(-6, actual);
    Ok(())
}

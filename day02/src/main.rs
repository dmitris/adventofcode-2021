use itertools::Itertools;
use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;
    let result1 = part1(&input)?;
    writeln!(io::stdout(), "part 1: {}", result1)?;
    let result1 = part2(&input)?;
    writeln!(io::stdout(), "part 2: {}", result1)?;
    Ok(())
}

fn part1(input: &str) -> Result<i32> {
    let mut horizontal_pos = 0;
    let mut depth = 0;
    for line in input.lines() {
        let (direction, change) = line
            .splitn(2, ' ')
            .collect_tuple()
            .ok_or_else(|| format!("bad input: {}", line))?;
        let n = change.parse::<i32>()?;
        match direction {
            "forward" => horizontal_pos += n,
            "down" => depth += n,
            "up" => depth -= n,
            _ => return Err(format!("bad direction: {} in line: {}", direction, line).into()),
        }
    }

    Ok(horizontal_pos * depth)
}

fn part2(input: &str) -> Result<i32> {
    let mut aim = 0;
    let mut horizontal_pos = 0;
    let mut depth = 0;
    for line in input.lines() {
        let (direction, change) = line
            .splitn(2, ' ')
            .collect_tuple()
            .ok_or_else(|| format!("bad input: {}", line))?;
        let n = change.parse::<i32>()?;
        match direction {
            "forward" => {
                horizontal_pos += n;
                depth += aim * n;
            }
            "down" => aim += n,
            "up" => aim -= n,
            _ => return Err(format!("bad direction: {} in line: {}", direction, line).into()),
        }
    }

    Ok(horizontal_pos * depth)
}

#[test]
fn test_part1() {
    let input = include_str!("../input/test01.txt");
    assert_eq!(150, part1(&input).unwrap());
}

#[test]
fn test_part1_full() {
    let input = include_str!("../input/input.txt");
    assert_eq!(1990000, part1(&input).unwrap());
}

#[test]
fn test_part2() {
    let input = include_str!("../input/test01.txt");
    assert_eq!(900, part2(&input).unwrap());
}

#[test]
fn test_part2_full() {
    let input = include_str!("../input/input.txt");
    assert_eq!(1975421260, part2(&input).unwrap());
}

#[test]
fn test_bad_input1() {
    let input = include_str!("../input/bad01.txt");
    let res1 = part1(&input);
    assert!(res1.is_err());
    let res2 = part2(&input);
    assert!(res2.is_err());
}

#[test]
fn test_bad_input2() {
    let input = include_str!("../input/bad02.txt");
    let res1 = part1(&input);
    assert!(res1.is_err());
    let res2 = part2(&input);
    assert!(res2.is_err());
}

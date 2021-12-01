use itertools::Itertools;
use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;
    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let cnt = input.lines()
        .filter_map(|n| n.parse::<u32>().ok())
        .tuple_windows()
        .filter(|(a,b)| a < b)
        .count();

    writeln!(io::stdout(), "part1: {}", cnt)?;
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut nums: Vec<u32> = Vec::new();
    for line in input.lines() {
        let n = line.parse::<u32>()?;
        nums.push(n)
    }
    let cnt: u32 = nums
        .iter()
        .tuple_windows::<(_, _, _, _)>()
        .fold(0, |acc, tp| {
            if tp.3 > tp.0 {
                acc + 1
            } else {
                acc
            }
        });

    writeln!(io::stdout(), "part2: {}", cnt)?;
    Ok(())
}

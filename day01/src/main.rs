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
    // let sum  = input.lines()
    //     .filter_map(|s| s.parse::<usize>().ok())
    //     .tuple_windows()
    //     .filter(|(a,b)| *a < *b )
    //     .count();

    let sum = input.lines()
        .map(|line| line.parse().unwrap())
        .fold((0u16, u16::MAX), |(sum, prev), curr| {
            (if curr > prev { sum + 1 } else { sum }, curr)
        }).0;
    writeln!(io::stdout(), "part1: {}", sum)?;
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut nums: Vec<u32> = Vec::new();
    for line in input.lines() {
        let n = line.parse::<u32>()?;
        nums.push(n)
    }
    let cnt = nums
        .iter()
        .tuple_windows::<(_, _, _, _)>()
        .filter(|(&a, _, _, &d)| d > a)
        .count();

    writeln!(io::stdout(), "part2: {}", cnt)?;
    Ok(())
}

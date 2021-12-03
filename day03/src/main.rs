use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;
    let result = part1(&input)?;
    writeln!(io::stdout(), "part 1: {}", result)?;
    // let result = part2(&input)?;
    // writeln!(io::stdout(), "part 2: {}", result)?;
    Ok(())
}

fn part1(input: &str) -> Result<usize> {
    let mut track: Vec<i16> = vec![];
    for (i, line) in input.lines().enumerate() {
        if i == 0 {
            for _ in 0..line.len() {
                track.push(0); // initialise
            }
        } else { // validate the same length of each record
            if line.len() != track.len() {
                return Err(format!("bad input line {} - must be exactly {} chars (each 0 or 1)", line, track.len()).into());
            }
        }
        let b = line.as_bytes();
        for j in 0..track.len() {
            match b[j] {
                0x30 => track[j] -= 1,
                0x31 => track[j] += 1,
                _ => return Err(format!("bad input line {} only '0' and '1' allowed", line).into()),
            }
        }        
    }
    let (gamma, epsilon) = track_to_number(&track)?;
    Ok(gamma*epsilon)
}

fn track_to_number(v: &Vec<i16>) -> Result<(usize, usize)> {
    let (mut gamma, mut epsilon) = (0, 0);
    for (i, n) in v.into_iter().enumerate() {
        if *n == 0 {
            return Err(format!("equal number of 0s and 1s in column {} (zero-based)", i).into());
        } else {
            gamma <<= 1;
            epsilon <<= 1;
        }
        if *n > 0 {
            gamma += 1; // set the least significant bit
        } else {
            epsilon += 1;
        }
    }
    Ok((gamma,epsilon))
}
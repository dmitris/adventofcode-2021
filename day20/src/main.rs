use std::io::{self, Read, Write}; // , Lines};

type Result<T> = ::std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;
    let (algo, data) = read_input(&input)?;
    let result = part1(&algo, &data)?;
    writeln!(io::stdout(), "part 1: {}", result)?;
    // let result = part2(&nums, &boards)?;
    // writeln!(io::stdout(), "part 2: {}", result)?;
    Ok(())
}

fn read_input(s: &str)  -> Result<([u8; 512], Vec<u8>)> {
    let algo: [u8; 512] = [0; 512];
    let data = Vec::with_capacity(10000);
    Ok( (algo, data))
}

fn part1(algo: &[u8; 512], data: &[u8]) -> Result<usize> {
    Ok(42)
}
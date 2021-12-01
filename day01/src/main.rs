use std::io::{stdin, BufRead};
use itertools::Itertools;

fn main() {
    let cnt = stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.parse::<u32>())
        .filter_map(Result::ok)
        .tuple_windows()
        .fold(0, |acc, tp: (u32, u32)| if tp.1 > tp.0 {
                acc + 1
        } else {
            acc
        }); 
    
    println!("larger than the previous measurement: {}", cnt)
}

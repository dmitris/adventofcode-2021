use std::io::{stdin, BufRead};
use itertools::Itertools;

fn main() {
    // part 2
    let cnt = stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.parse::<u32>())
        .filter_map(Result::ok)
        .tuple_windows::<(_, _, _, _)>()
        .fold(0, |acc, tp: (u32, u32, u32, u32)| {
            if tp.1 + tp.2 + tp.3 > tp.0 + tp.1 + tp.2 {
                acc + 1
            } else {
                acc
            }
        }); 
    
    println!("answer part two: {}", cnt)
}

// part 1
// fn main() {
//     let cnt = stdin()
//         .lock()
//         .lines()
//         .filter_map(Result::ok)
//         .map(|line| line.parse::<u32>())
//         .filter_map(Result::ok)
//         .tuple_windows()
//         .fold(0, |acc, tp: (u32, u32)| if tp.1 > tp.0 {
//                 acc + 1
//         } else {
//             acc
//         });

//     println!("larger than the previous measurement: {}", cnt)
// }
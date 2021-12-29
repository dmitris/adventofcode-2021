use itertools::Itertools;
use std::io::{self, Read, Write, Lines};

type Result<T> = ::std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;
    let all_points = read_input(&input)?;
    let result = part1(&all_points)?;
    writeln!(io::stdout(), "part 1: {}", result)?;
    // let result = part2(&nums, &boards)?;
    // writeln!(io::stdout(), "part 2: {}", result)?;
    Ok(())
}

#[derive(Debug)]
struct Point (u16, u16);

#[derive(Debug)]
struct PointsPair (Point, Point);

impl PointsPair {
    fn from_line(_s: &str) -> Result<PointsPair> {
        Ok(PointsPair( Point(1, 2), Point(3, 4)))
    }
}

fn read_input(s: &str) -> Result<Vec<PointsPair>> {
    let mut ret = Vec::<PointsPair>::new();
    for l in s.lines() {
        let p = PointsPair::from_line(l)?;
        println!("{:?}", p);
    }
    // let _ = s.lines()
    //     .map(|a| a.splitn(2, " -> "))
    //     .tuple_windows()
    //     .map(|b: (&str, &str)| println!("{:#?}", b))
    //     .count();

        // .filter_map(|s| s.parse::<usize>().ok())
    // .tuple_windows()
    // .filter(|(a, b)| *a < *b)
    // .count();
    Ok(ret)

    // let nums: Vec<u8> = it
    //     .next()
    //     .ok_or("bad input")?
    //     .split(',')
    //     .map(|s| s.parse::<u8>().unwrap())
    //     .collect();
    // let mut ret = Vec::<Matrix5<u8>>::new();
    // for el in it {
    //     let num_iter = el
    //         .split_ascii_whitespace()
    //         .map(|s| s.parse::<u8>().unwrap());
    //     let m5: Matrix5<u8> = Matrix5::from_iterator(num_iter);
    //     ret.push(m5);
    // }
    // Ok((nums, ret))
}

fn part1(_data: &[PointsPair]) -> Result<usize> {
    Ok(42)
}

// mod tests {
//     use super::{part1, part2, read_input};

//     #[test]
//     fn test_small() {
//         let input = include_str!("../input/test01.txt");
//         let (nums, boards) = read_input(&input).unwrap();
//         assert_eq!(4512, part1(&nums, &boards).unwrap());
//         assert_eq!(1924, part2(&nums, &boards).unwrap());
//     }

//     #[test]
//     fn test_full() {
//         let input = include_str!("../input/input.txt");
//         let (nums, boards) = read_input(&input).unwrap();
//         assert_eq!(11536, part1(&nums, &boards).unwrap());
//         assert_eq!(1284, part2(&nums, &boards).unwrap());
//     }
// }

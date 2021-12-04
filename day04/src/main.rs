use nalgebra::{Matrix1x5, Matrix5, Matrix5x1};
use std::{
    collections::HashSet,
    io::{self, Read, Write},
};

type Result<T> = ::std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;
    let (nums, boards) = read_input(&input)?;
    let result = part1(&nums, &boards)?;
    writeln!(io::stdout(), "part 1: {}", result)?;
    let result = part2(&nums, &boards)?;
    writeln!(io::stdout(), "part 2: {}", result)?;
    Ok(())
}

fn read_input(s: &str) -> Result<(Vec<u8>, Vec<Matrix5<u8>>)> {
    let mut it = s.split("\n\n");
    let nums: Vec<u8> = it
        .next()
        .ok_or("bad input")?
        .split(',')
        .map(|s| s.parse::<u8>().unwrap())
        .collect();
    let mut ret = Vec::<Matrix5<u8>>::new();
    for el in it {
        let num_iter = el
            .split_ascii_whitespace()
            .map(|s| s.parse::<u8>().unwrap());
        let m5: Matrix5<u8> = Matrix5::from_iterator(num_iter);
        ret.push(m5);
    }
    Ok((nums, ret))
}

fn part1(nums: &[u8], boards: &[Matrix5<u8>]) -> Result<usize> {
    let num_boards = boards.len();
    let mut marks: Vec<Matrix5<u8>> = Vec::with_capacity(num_boards);
    for _ in 0..num_boards {
        marks.push(Matrix5::zeros());
    }

    for &n in nums {
        for i in 0..num_boards {
            if check_and_mark(n, i, boards, &mut marks) {
                return Ok(calculate_result(n, i, boards, &marks));
            }
        }
    }
    Err("no winning board found".into())
}

fn part2(nums: &[u8], boards: &[Matrix5<u8>]) -> Result<usize> {
    let num_boards = boards.len();
    let mut marks: Vec<Matrix5<u8>> = Vec::with_capacity(num_boards);
    for _ in 0..num_boards {
        marks.push(Matrix5::zeros());
    }

    // winning_boards keeps the track of the boards that have already won.
    let mut winning_boards = HashSet::with_capacity(num_boards - 1);
    for &n in nums {
        for i in 0..num_boards {
            if winning_boards.contains(&i) {
                continue; // already won
            }
            if check_and_mark(n, i, boards, &mut marks) {
                if winning_boards.len() == num_boards - 1 {
                    return Ok(calculate_result(n, i, boards, &marks));
                }
                winning_boards.insert(i);
            }
        }
    }
    Err("no winning board found".into())
}

fn check_and_mark(num: u8, i: usize, boards: &[Matrix5<u8>], marks: &mut Vec<Matrix5<u8>>) -> bool {
    let ones_row = Matrix1x5::<u8>::new(1, 1, 1, 1, 1);
    let ones_col = Matrix5x1::<u8>::new(1, 1, 1, 1, 1);
    for j in 0..5 {
        for k in 0..5 {
            if boards[i][(j, k)] == num {
                marks[i][(j, k)] = 1;
            }
        }
    }
    for r in marks[i].row_iter() {
        if r == ones_row {
            return true;
        }
    }
    for c in marks[i].column_iter() {
        if c == ones_col {
            return true;
        }
    }
    false
}

fn calculate_result(num: u8, i: usize, boards: &[Matrix5<u8>], marks: &[Matrix5<u8>]) -> usize {
    let mut sum_unmarked = 0_usize;
    for j in 0..5 {
        for k in 0..5 {
            if marks[i][(j, k)] == 0 {
                sum_unmarked += boards[i][(j, k)] as usize;
            }
        }
    }
    sum_unmarked * num as usize
}

mod tests {
    use super::{part1, part2, read_input};

    #[test]
    fn test_small() {
        let bytes = include_bytes!("../input/test01.txt");
        let input = String::from_utf8_lossy(bytes);
        let (nums, boards) = read_input(&input).unwrap();
        assert_eq!(4512, part1(&nums, &boards).unwrap());
        assert_eq!(1924, part2(&nums, &boards).unwrap());
    }

    #[test]
    fn test_full() {
        let bytes = include_bytes!("../input/input.txt");
        let input = String::from_utf8_lossy(bytes);
        let (nums, boards) = read_input(&input).unwrap();
        assert_eq!(11536, part1(&nums, &boards).unwrap());
        assert_eq!(1284, part2(&nums, &boards).unwrap());
    }
}

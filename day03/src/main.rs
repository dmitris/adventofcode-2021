use std::io::{self, Read, Write};

type Result<T> = ::std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;
    let result = part1(&input)?;
    writeln!(io::stdout(), "part 1: {}", result)?;
    let result = part2(&input)?;
    writeln!(io::stdout(), "part 2: {}", result)?;
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

use std::collections::HashSet;
fn part2(input: &str) -> Result<usize> {
    let mut oxygen = HashSet::new();
    let mut scrubber = HashSet::new();
    let mut track: Vec<i16> = vec![];
    for (i, line) in input.lines().enumerate() {
        let num = usize::from_str_radix(line, 2)?;
        oxygen.insert(num);
        scrubber.insert(num);
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
    let (oxy_rating, scrubber_rating) = helper_part2(track.len(), oxygen, scrubber)?;
    Ok(oxy_rating * scrubber_rating)
}

fn helper_part2(rec_len: usize, mut oxygen: HashSet<usize>, mut scrubber: HashSet<usize>) -> Result<(usize, usize)> {
    if oxygen.len() == 0 || scrubber.len() == 0 {
        return Err("bad input - oxygen and/or scrubber sets are empty".into());
    }
    let mut mask = 1 << rec_len-1;
    for _ in 0..rec_len {
        if oxygen.len() == 1 && scrubber.len() == 1 {
            break;
        }
        if oxygen.len() > 1 {
            let (zeros, ones) = count_zeros_ones(&oxygen, mask);
            if ones >= zeros {
                oxygen.retain(|&x| (x & mask) > 0);
            } else {
                oxygen.retain(|&x| (x & mask) == 0);
            }
        }
        if scrubber.len() > 1 {
            let (zeros, ones) = count_zeros_ones(&scrubber, mask);
            if zeros <= ones {
                scrubber.retain(|&x| (x & mask) == 0);
            } else {
                scrubber.retain(|&x| (x & mask) > 0);
            }
        }
        mask = mask >> 1;
    }

    if oxygen.len() != 1  {
        return Err(format!("more than one number left unfiltered in oxygen - len: {}", oxygen.len()).into());
    }
    if scrubber.len() != 1  {
        return Err(format!("more than one number left unfiltered in scrubber - len: {}", scrubber.len()).into());
    }
    Ok( (oxygen.into_iter().next().unwrap(), scrubber.into_iter().next().unwrap()) )
}

fn count_zeros_ones(s: &HashSet<usize>, mask: usize) -> (usize, usize) {
    s.iter()
        .fold((0 as usize, 0 as usize), |(a, b), e| {
            if e & mask > 0 {
                (a+1, b)
            } else {
                (a, b+1)
            }
    })

    // // equivalent loop version:
    // let (mut ones, mut zeros) : (usize, usize) = (0, 0);
    // for e in s {
    //     if e & mask > 0 {
    //         ones += 1;
    //     } else {
    //         zeros += 1;
    //     }
    // }
    // (ones, zeros)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::{count_zeros_ones, part1, part2};

    #[test]
    fn test_count_zeros_ones() {
        let hs = HashSet::from([0b00100, 0b11110, 0b10110]);
        let mask = 0b10000;
        assert_eq!( (2, 1), count_zeros_ones(&hs, mask));
        let mask = 0b1000;
        assert_eq!( (1, 2), count_zeros_ones(&hs, mask));
        let mask = 0b100;
        assert_eq!( (3, 0), count_zeros_ones(&hs, mask));
        let mask = 0b10;
        assert_eq!( (2, 1), count_zeros_ones(&hs, mask));
        let mask = 0b1;
        assert_eq!( (0, 3), count_zeros_ones(&hs, mask));
    }

    #[test]
    fn test_small() {
        let bytes = include_bytes!("../input/test01.txt");
        let input = String::from_utf8_lossy(bytes);
        assert_eq!(198, part1(&input).unwrap());
        assert_eq!(230, part2(&input).unwrap());
    }
}
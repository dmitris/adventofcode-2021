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
    // track keeps the sum of the number of ones minus number of zeros
    // seen for each bit position, updated after each record.
    let mut track: Vec<i16> = vec![];
    for (i, line) in input.lines().enumerate() {
        if i == 0 {
            track.resize(line.len(), 0);
        } else {
            // validate the same length of each record
            if line.len() != track.len() {
                return Err(format!(
                    "bad input line {} - must be exactly {} chars (each 0 or 1)",
                    line,
                    track.len()
                )
                .into());
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
    Ok(gamma * epsilon)
}

fn track_to_number(v: &[i16]) -> Result<(usize, usize)> {
    v.iter().fold(Ok((0, 0)), |res, n| {
        let (a, b) = res?;
        if *n == 0 {
            return Err("bad input - equal number of 0s and 1s".to_string().into());
        }
        // both a and b are left-shifted, plus a conditional increment
        if *n > 0 {
            Ok(((a << 1) + 1, b << 1))
        } else {
            Ok((a << 1, (b << 1) + 1))
        }
    })

    // // equivalent loop version
    //
    // let (mut gamma, mut epsilon) = (0, 0);
    // for (i, n) in v.into_iter().enumerate() {
    //     if *n == 0 {
    //         return Err(format!("equal number of 0s and 1s in column {} (zero-based)", i).into());
    //     }
    //     gamma <<= 1;
    //     epsilon <<= 1;

    //     if *n > 0 {
    //         gamma += 1; // set the least significant bit
    //     } else {
    //         epsilon += 1;
    //     }
    // }
    // Ok((gamma, epsilon))
}

use std::collections::HashSet;
fn part2(input: &str) -> Result<usize> {
    let mut oxygen = HashSet::new();
    let mut scrubber = HashSet::new();
    let mut rec_len: usize = 0;
    for (i, line) in input.lines().enumerate() {
        if i == 0 {
            rec_len = line.len();
        }
        let num = usize::from_str_radix(line, 2)?;
        oxygen.insert(num);
        scrubber.insert(num);
        // validate the same length of each record
        if line.len() != rec_len {
            return Err(format!(
                "bad input line {} - must be exactly {} chars (each 0 or 1)",
                line, rec_len
            )
            .into());
        }
    }
    let (oxy_rating, scrubber_rating) = helper_part2(rec_len, oxygen, scrubber)?;
    Ok(oxy_rating * scrubber_rating)
}

fn helper_part2(
    rec_len: usize,
    mut oxygen: HashSet<usize>,
    mut scrubber: HashSet<usize>,
) -> Result<(usize, usize)> {
    if oxygen.is_empty() || scrubber.is_empty() {
        return Err("bad input - oxygen and/or scrubber sets are empty".into());
    }
    let mut mask = 1 << (rec_len - 1);
    for _ in 0..rec_len {
        if oxygen.len() == 1 && scrubber.len() == 1 {
            break;
        }
        if oxygen.len() > 1 {
            oxygen = set_filter(oxygen, mask, true);
        }
        if scrubber.len() > 1 {
            scrubber = set_filter(scrubber, mask, false);
        }
        mask >>= 1;
    }

    if oxygen.len() != 1 || scrubber.len() != 1 {
        return Err(format!(
            "more than one number left not filtered out: oxygen's len: {}, scrubber's - {}",
            oxygen.len(),
            scrubber.len()
        )
        .into());
    }
    let (oxy, scrub) = (
        oxygen.into_iter().next().unwrap(),
        scrubber.into_iter().next().unwrap(),
    );
    Ok((oxy, scrub))
}

fn set_filter(mut hs: HashSet<usize>, mask: usize, retain_majority: bool) -> HashSet<usize> {
    if hs.len() <= 1 {
        return hs;
    }
    let (zeros, ones) = count_zeros_ones(&hs, mask);
    if (ones >= zeros) == retain_majority {
        hs.retain(|&x| (x & mask) > 0);
    } else {
        hs.retain(|&x| (x & mask) == 0);
    }
    hs
}

// count_zeros_ones returns a tuple of (<cnt_of_zeros>, <count_of_ones>)
// for the bits corresponding to the mask.
fn count_zeros_ones(s: &HashSet<usize>, mask: usize) -> (usize, usize) {
    s.iter().fold((0_usize, 0_usize), |(a, b), e| {
        if e & mask > 0 {
            (a, b + 1) // increment the ones counter
        } else {
            (a + 1, b) // increment the zeros counter
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
        assert_eq!((1, 2), count_zeros_ones(&hs, 0b10000));
        assert_eq!((2, 1), count_zeros_ones(&hs, 0b1000));
        assert_eq!((0, 3), count_zeros_ones(&hs, 0b100));
        assert_eq!((1, 2), count_zeros_ones(&hs, 0b10));
        assert_eq!((3, 0), count_zeros_ones(&hs, 0b1));
    }

    #[test]
    fn test_small() {
        let input = include_str!("../input/test01.txt");
        assert_eq!(198, part1(&input).unwrap());
        assert_eq!(230, part2(&input).unwrap());
    }

    #[test]
    fn test_full() {
        let input = include_str!("../input/input.txt");
        assert_eq!(4118544, part1(&input).unwrap());
        assert_eq!(3832770, part2(&input).unwrap());
    }
}

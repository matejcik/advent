use std::io::BufRead;

use bstr::io::BufReadExt;

use crate::Solver;

fn parse_snafu(slice: &[u8]) -> i64 {
    let mut num = 0;
    for c in slice {
        let n = match c {
            b'0'..=b'2' => *c as i64 - b'0' as i64,
            b'-' => -1,
            b'=' => -2,
            _ => unreachable!(),
        };
        num = num * 5 + n;
    }
    num
}

fn to_snafu(num: i64) -> String {
    let mut num = num;
    let mut result = String::with_capacity(10);
    while num != 0 {
        let n = num % 5;
        num /= 5;
        let digit = if (0..=2).contains(&n) {
            b'0' + n as u8
        } else {
            num += 1;
            if n == 3 {
                b'='
            } else {
                b'-'
            }
        };
        result.insert(0, digit as char);
    }
    result
}

fn part1_snafu_sum(mut input: &mut dyn BufRead) -> String {
    let mut sum = 0;
    input
        .for_byte_line(|line| {
            sum += parse_snafu(line);
            Ok(true)
        })
        .unwrap();
    to_snafu(sum)
}

pub const SOLVERS: &[Solver] = &[part1_snafu_sum];

#[allow(unused)]
mod test {
    use super::*;

    #[test]
    fn test_snafu() {
        let VECTORS = vec![
            ("1=-0-2", 1747),
            ("12111", 906),
            ("2=0=", 198),
            ("21", 11),
            ("2=01", 201),
            ("111", 31),
            ("20012", 1257),
            ("112", 32),
            ("1=-1=", 353),
            ("1-12", 107),
            ("12", 7),
            ("1=", 3),
            ("122", 37),
        ];

        for (s, n) in VECTORS {
            assert_eq!(parse_snafu(s.as_bytes()), n);
            assert_eq!(to_snafu(n), s);
        }
    }
}

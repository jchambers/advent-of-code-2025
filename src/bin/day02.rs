use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let mut ranges = String::new();
        File::open(path)?.read_to_string(&mut ranges)?;

        println!("Sum of invalid IDs: {}", invalid_id_sum(&ranges));

        Ok(())
    } else {
        Err("Usage: day02 INPUT_FILE_PATH".into())
    }
}

fn invalid_id_sum(ranges: &str) -> u64 {
    ranges.split(',')
        .filter_map(|s| IdRange::from_str(s).ok())
        .flat_map(|id_range| id_range.invalid_ids())
        .sum()
}

struct IdRange {
    start: u64,
    end: u64,
}

impl IdRange {
    pub fn invalid_ids(&self) -> Vec<u64> {
        let mut invalid_ids = Vec::new();

        let mut left = if self.start < 10 {
            // Don't do anything fancy for single-digit numbers
            self.start
        } else if self.start.ilog10().is_multiple_of(2) {
            // We have an odd number of digits, and we won't find any invalid IDs until we start
            // exploring candidates with even numbers of digits. To get there efficiently, jump up
            // to the next power of ten for the left half of the ID.
            10u64.pow(self.start.ilog10() / 2)
        } else {
            // We have an even number of digits; use the left half as a starting point
            self.start / 10u64.pow(self.start.ilog10().div_ceil(2))
        };

        loop {
            let next_invalid_id = (left * 10u64.pow(left.ilog10() + 1)) + left;

            if next_invalid_id > self.end {
                break;
            }

            if next_invalid_id >= self.start {
                invalid_ids.push(next_invalid_id);
            }

            left += 1;
        }

        invalid_ids
    }
}

impl FromStr for IdRange {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let components: Vec<&str> = s.splitn(2, '-').collect();

        if components.len() != 2 {
            Err("Could not parse range string".into())
        } else {
            Ok(IdRange {
                start: components[0].parse()?,
                end: components[1].parse()?,
            })
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{invalid_id_sum, IdRange};

    #[test]
    fn test_invalid_ids() {
        assert_eq!(vec![11], IdRange { start: 1, end: 19 }.invalid_ids());
        assert_eq!(vec![11, 22], IdRange { start: 11, end: 22 }.invalid_ids());
        assert_eq!(vec![99], IdRange { start: 95, end: 115 }.invalid_ids());
        assert_eq!(vec![1010], IdRange { start: 998, end: 1012 }.invalid_ids());
        assert_eq!(vec![1188511885], IdRange { start: 1188511880, end: 1188511890 }.invalid_ids());
        assert_eq!(vec![222222], IdRange { start: 222220, end: 222224 }.invalid_ids());
        assert_eq!(Vec::<u64>::new(), IdRange { start: 1698522, end: 1698528 }.invalid_ids());
        assert_eq!(vec![446446], IdRange { start: 446443, end: 446449 }.invalid_ids());
        assert_eq!(vec![38593859], IdRange { start: 38593856, end: 38593862 }.invalid_ids());
        assert_eq!(Vec::<u64>::new(), IdRange { start: 565653, end: 565659 }.invalid_ids());
        assert_eq!(Vec::<u64>::new(), IdRange { start: 824824821, end: 824824827 }.invalid_ids());
        assert_eq!(Vec::<u64>::new(), IdRange { start: 2121212118, end: 2121212124 }.invalid_ids());
    }

    #[test]
    fn test_invalid_id_sum() {
        assert_eq!(1227775554, invalid_id_sum("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"));
    }
}

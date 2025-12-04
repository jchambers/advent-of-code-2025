use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        println!(
            "Joltage sum with 2 active batteries: {}",
            joltage_sum(BufReader::new(File::open(path)?), 2)?
        );

        println!(
            "Joltage sum with 12 active batteries: {}",
            joltage_sum(BufReader::new(File::open(path)?), 12)?
        );

        Ok(())
    } else {
        Err("Usage: day03 INPUT_FILE_PATH".into())
    }
}

fn joltage_sum(reader: impl BufRead, active_batteries: usize) -> Result<u64, Box<dyn Error>> {
    reader
        .lines()
        .map_while(|result| result.ok())
        .map(|line| BatteryBank::from_str(&line))
        .map(|battery_bank| battery_bank.map(|b| b.max_joltage(active_batteries)))
        .sum::<Result<_, _>>()
}

struct BatteryBank {
    batteries: Vec<u32>,
}

impl BatteryBank {
    pub fn max_joltage(&self, active_batteries: usize) -> u64 {
        let mut joltage = 0u64;
        let mut left = 0;

        assert!(active_batteries <= self.batteries.len());

        for reserved_batteries in (0..active_batteries).rev() {
            let (position, best_joltage) = self.batteries[left..self.batteries.len() - reserved_batteries]
                .iter()
                .enumerate()
                // This may seem a little funky, but max_by_key will return the LAST element it
                // finds, and we really want the FIRST
                .rev()
                .max_by_key(|(_, j)| *j)
                .expect("Non-empty bank of batteries must have at least one max value");

            left += position + 1;

            joltage = (joltage * 10) + (*best_joltage as u64);
        }

        joltage
    }
}

impl FromStr for BatteryBank {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let batteries: Vec<u32> = s
            .chars()
            .map(|c| c.to_digit(10).ok_or("Could not parse digit"))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(BatteryBank { batteries })
    }
}

#[cfg(test)]
mod test {
    use crate::{BatteryBank, joltage_sum};
    use indoc::indoc;
    use std::io::Cursor;
    use std::str::FromStr;

    const TEST_BATTERY_BANKS: &str = indoc! {"
        987654321111111
        811111111111119
        234234234234278
        818181911112111
    "};

    #[test]
    fn test_max_joltage() {
        assert_eq!(
            98,
            BatteryBank::from_str("987654321111111")
                .unwrap()
                .max_joltage(2)
        );

        assert_eq!(
            89,
            BatteryBank::from_str("811111111111119")
                .unwrap()
                .max_joltage(2)
        );

        assert_eq!(
            78,
            BatteryBank::from_str("234234234234278")
                .unwrap()
                .max_joltage(2)
        );

        assert_eq!(
            92,
            BatteryBank::from_str("818181911112111")
                .unwrap()
                .max_joltage(2)
        );

        assert_eq!(
            987654321111,
            BatteryBank::from_str("987654321111111")
                .unwrap()
                .max_joltage(12)
        );

        assert_eq!(
            811111111119,
            BatteryBank::from_str("811111111111119")
                .unwrap()
                .max_joltage(12)
        );

        assert_eq!(
            434234234278,
            BatteryBank::from_str("234234234234278")
                .unwrap()
                .max_joltage(12)
        );

        assert_eq!(
            888911112111,
            BatteryBank::from_str("818181911112111")
                .unwrap()
                .max_joltage(12)
        );
    }

    #[test]
    fn test_max_joltage_sum() {
        assert_eq!(
            357,
            joltage_sum(Cursor::new(TEST_BATTERY_BANKS), 2).unwrap()
        );

        assert_eq!(
            3121910778619,
            joltage_sum(Cursor::new(TEST_BATTERY_BANKS), 12).unwrap()
        );
    }
}

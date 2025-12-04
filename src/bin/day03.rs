use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        println!(
            "Joltage sum: {}",
            joltage_sum(BufReader::new(File::open(path)?))?
        );

        Ok(())
    } else {
        Err("Usage: day03 INPUT_FILE_PATH".into())
    }
}

fn joltage_sum(reader: impl BufRead) -> Result<u32, Box<dyn Error>> {
    let battery_banks = reader
        .lines()
        .map_while(|result| result.ok())
        .map(|line| BatteryBank::from_str(&line))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(battery_banks.iter().map(|bank| bank.max_joltage()).sum())
}

struct BatteryBank {
    batteries: Vec<u32>,
}

impl BatteryBank {
    pub fn max_joltage(&self) -> u32 {
        let first_joltage = self.batteries[..self.batteries.len() - 1]
            .iter()
            .max()
            .expect("Battery bank must have more than two elements");

        let first_battery_position = self
            .batteries
            .iter()
            .position(|joltage| joltage == first_joltage)
            .expect("Battery must contain previously-identified joltage");

        let second_joltage = self.batteries[first_battery_position + 1..]
            .iter()
            .max()
            .expect("Battery bank must have a battery after the first selected");

        (first_joltage * 10) + second_joltage
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
                .max_joltage()
        );

        assert_eq!(
            89,
            BatteryBank::from_str("811111111111119")
                .unwrap()
                .max_joltage()
        );

        assert_eq!(
            78,
            BatteryBank::from_str("234234234234278")
                .unwrap()
                .max_joltage()
        );

        assert_eq!(
            92,
            BatteryBank::from_str("818181911112111")
                .unwrap()
                .max_joltage()
        );
    }

    #[test]
    fn test_max_joltage_sum() {
        assert_eq!(357, joltage_sum(Cursor::new(TEST_BATTERY_BANKS)).unwrap());
    }
}

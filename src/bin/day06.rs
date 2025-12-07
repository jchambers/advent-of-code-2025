use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let math_homework = {
            let mut homework = String::new();
            File::open(path)?.read_to_string(&mut homework)?;

            MathHomework::from_str(&homework)?
        };

        println!("Grand total: {}", math_homework.grand_total());

        Ok(())
    } else {
        Err("Usage: day06 INPUT_FILE_PATH".into())
    }
}

struct MathHomework {
    numbers: Vec<Vec<u64>>,
    operations: Vec<Operation>,
}

impl MathHomework {
    pub fn grand_total(&self) -> u64 {
        let mut grand_total = 0;

        for i in 0..self.operations.len() {
            grand_total += match self.operations[i] {
                Operation::Add => self.numbers.iter().map(|row| row[i]).sum::<u64>(),
                Operation::Multiply => self.numbers.iter().map(|row| row[i]).product(),
            }
        }

        grand_total
    }
}

impl FromStr for MathHomework {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let operations = lines
            .next_back()
            .expect("Math homework must have at least one line")
            .split_whitespace()
            .map(Operation::from_str)
            .collect::<Result<Vec<_>, _>>()?;

        let numbers = lines
            .map(|line| {
                line.split_whitespace()
                    .map(|number| number.parse())
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        if numbers.is_empty() {
            return Err("Math homework must contain at least one row of numbers".into());
        }

        let expected_length = numbers[0].len();

        if !numbers.iter().all(|row| row.len() == expected_length) {
            return Err("All rows of numbers must have equal length".into());
        }

        if operations.len() != expected_length {
            return Err("Unexpected operator vector length".into());
        }

        Ok(MathHomework {
            numbers,
            operations,
        })
    }
}

enum Operation {
    Add,
    Multiply,
}

impl FromStr for Operation {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operation::Add),
            "*" => Ok(Operation::Multiply),
            _ => Err("Unrecognized operation".into()),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::MathHomework;
    use indoc::indoc;
    use std::str::FromStr;

    const TEST_HOMEWORK: &str = indoc! {"
        123 328  51 64
         45 64  387 23
          6 98  215 314
        *   +   *   +
    "};

    #[test]
    fn test_math_homework_grand_total() {
        assert_eq!(
            4277556,
            MathHomework::from_str(TEST_HOMEWORK).unwrap().grand_total()
        )
    }
}

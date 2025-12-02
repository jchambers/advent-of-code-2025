use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let safe = Safe::try_from(BufReader::new(File::open(path)?))?;

        println!("Password: {}", safe.password());

        Ok(())
    } else {
        Err("Usage: day01 INPUT_FILE_PATH".into())
    }
}

struct Safe {
    rotations: Vec<Rotation>,
}

impl Safe {
    pub fn try_from(reader: impl BufRead) -> Result<Self, Box<dyn Error>> {
        let rotations: Vec<Rotation> = reader.lines()
            .map_while(|result| result.ok())
            .map(|line| Rotation::from_str(&line))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Safe { rotations })
    }

    pub fn password(&self) -> u32 {
        let mut position = 50;
        let mut password = 0;

        for rotation in &self.rotations {
            position = rotation.apply(position);

            if position == 0 {
                password += 1;
            }
        }

        password
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Rotation {
    Left(u32),
    Right(u32),
}

impl Rotation {
    pub fn apply(&self, position: u32) -> u32 {
        match self {
            Rotation::Left(distance) => {
                let position = position as i32;
                (((position - ((distance % 100) as i32)) + 100) % 100) as u32
            },
            Rotation::Right(distance) => (position + distance) % 100
        }
    }
}

impl FromStr for Rotation {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let magnitude:u32 = s[1..].parse()?;

        match s.chars().next() {
            Some('L') => Ok(Rotation::Left(magnitude)),
            Some('R') => Ok(Rotation::Right(magnitude)),
            _ => Err("Could not parse rotation string".into())
        }
    }
}

#[cfg(test)]
mod test {
    use std::io::Cursor;
    use std::str::FromStr;
    use indoc::indoc;
    use crate::{Rotation, Safe};

    const TEST_SAFE: &str = indoc! {"
        L68
        L30
        R48
        L5
        R60
        L55
        L1
        L99
        R14
        L82
    "};

    #[test]
    fn test_rotation_from_str() {
        assert_eq!(Rotation::Left(68), Rotation::from_str("L68").unwrap());
        assert_eq!(Rotation::Right(14), Rotation::from_str("R14").unwrap());
        assert!(Rotation::from_str("Not a rotation").is_err());
    }

    #[test]
    fn test_rotation_apply() {
        assert_eq!(19, Rotation::Right(8).apply(11));
        assert_eq!(0, Rotation::Left(19).apply(19));
        assert_eq!(99, Rotation::Left(1).apply(0));
        assert_eq!(0, Rotation::Right(1).apply(99));
        assert_eq!(19, Rotation::Right(5008).apply(11));
        assert_eq!(7, Rotation::Left(402).apply(9));
    }

    #[test]
    fn test_safe_password() {
        let safe = Safe::try_from(Cursor::new(TEST_SAFE.as_bytes())).unwrap();

        assert_eq!(3, safe.password());
    }
}

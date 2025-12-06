use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let database = IngredientDatabase::try_from_buf_read(BufReader::new(File::open(path)?))?;

        println!("Fresh ingredients: {}", database.fresh_ingredients());

        Ok(())
    } else {
        Err("Usage: day05 INPUT_FILE_PATH".into())
    }
}

struct IngredientDatabase {
    fresh_ranges: Vec<RangeInclusive<u64>>,
    ids: Vec<u64>,
}

impl IngredientDatabase {
    pub fn try_from_buf_read(reader: impl BufRead) -> Result<Self, Box<dyn Error>> {
        let mut read_empty_line = false;

        let mut fresh_ranges = Vec::new();
        let mut ids = Vec::new();

        for line in reader.lines() {
            let line = line?;

            if line.is_empty() {
                read_empty_line = true;
            } else if !read_empty_line {
                // We haven't hit a blank line yet and are still reading ID ranges
                if let [start, end] = line.splitn(2, '-').collect::<Vec<&str>>().as_slice() {
                    fresh_ranges.push(RangeInclusive::new(start.parse()?, end.parse()?));
                } else {
                    return Err("Could not parse ID range".into());
                }
            } else {
                // We're past the split point and are now reading individual IDs
                ids.push(line.parse()?);
            }
        }

        Ok(IngredientDatabase { fresh_ranges, ids })
    }

    pub fn fresh_ingredients(&self) -> usize {
        self.ids
            .iter()
            .filter(|id| self.fresh_ranges.iter().any(|range| range.contains(id)))
            .count()
    }
}

#[cfg(test)]
mod test {
    use crate::IngredientDatabase;
    use indoc::indoc;
    use std::io::Cursor;

    const TEST_DATABASE: &str = indoc! {"
        3-5
        10-14
        16-20
        12-18

        1
        5
        8
        11
        17
        32
    "};

    #[test]
    fn test_fresh_ingredients() {
        assert_eq!(
            3,
            IngredientDatabase::try_from_buf_read(Cursor::new(TEST_DATABASE))
                .unwrap()
                .fresh_ingredients()
        );
    }
}

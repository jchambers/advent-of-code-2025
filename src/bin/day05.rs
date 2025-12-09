use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let database = IngredientDatabase::try_from_buf_read(BufReader::new(File::open(path)?))?;

        println!("Actual fresh ingredients: {}", database.fresh_ingredients());

        println!(
            "Possible fresh ingredients: {}",
            database.possible_fresh_ingredients()
        );

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

    pub fn possible_fresh_ingredients(&self) -> u64 {
        let mut merged_ranges: Vec<RangeInclusive<u64>> = Vec::new();

        let mut sorted_ranges = self.fresh_ranges.clone();
        sorted_ranges.sort_by_key(|range| *range.start());
        let sorted_ranges = sorted_ranges;

        for range in sorted_ranges {
            if merged_ranges
                .last()
                .map(|last_merged_range| last_merged_range.contains(range.start()))
                .unwrap_or(false)
            {
                // The ranges overlap, so smoosh them together
                let last_merged_range = merged_ranges.pop().unwrap();

                merged_ranges.push(RangeInclusive::new(
                    *last_merged_range.start().min(range.start()),
                    *last_merged_range.end().max(range.end()),
                ));
            } else {
                merged_ranges.push(range);
            }
        }

        merged_ranges
            .iter()
            .map(|range| range.end() - range.start() + 1)
            .sum()
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

    #[test]
    fn test_possible_fresh_ingredients() {
        assert_eq!(
            14,
            IngredientDatabase::try_from_buf_read(Cursor::new(TEST_DATABASE))
                .unwrap()
                .possible_fresh_ingredients()
        );
    }
}

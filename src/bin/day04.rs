use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::ops::Index;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let warehouse = {
            let mut warehouse_map = String::new();
            File::open(path)?.read_to_string(&mut warehouse_map)?;

            Warehouse::from_str(warehouse_map.as_str())?
        };

        println!("Movable rolls: {}", warehouse.movable_rolls());

        Ok(())
    } else {
        Err("Usage: day04 INPUT_FILE_PATH".into())
    }
}

struct Warehouse {
    width: usize,
    tiles: Vec<Tile>,
}

impl Warehouse {
    pub fn movable_rolls(&self) -> usize {
        let mut movable_rolls = 0;

        for x in 0..self.width {
            for y in 0..self.height() {
                if matches!(self[(x, y)], Tile::PaperRoll) {
                    let adjacent_rolls = self
                        .neighbors(x, y)
                        .iter()
                        .filter(|tile| matches!(tile, Tile::PaperRoll))
                        .count();

                    if adjacent_rolls < 4 {
                        movable_rolls += 1;
                    }
                }
            }
        }

        movable_rolls
    }

    fn height(&self) -> usize {
        self.tiles.len() / self.width
    }

    fn neighbors(&self, x: usize, y: usize) -> Vec<&Tile> {
        let mut neighbors = Vec::new();

        let min_x = if x == 0 { 0 } else { x - 1 };
        let max_x = (x + 1).min(self.width - 1);
        let min_y = if y == 0 { 0 } else { y - 1 };
        let max_y = (y + 1).min(self.height() - 1);

        for a in min_x..=max_x {
            for b in min_y..=max_y {
                if (a, b) != (x, y) {
                    neighbors.push(&self[(a, b)]);
                }
            }
        }

        neighbors
    }
}

impl Index<(usize, usize)> for Warehouse {
    type Output = Tile;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x, y) = index;

        if x >= self.width || y >= self.height() {
            panic!("Index out of bounds");
        }

        &self.tiles[x + (y * self.height())]
    }
}

impl FromStr for Warehouse {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s
            .lines()
            .next()
            .ok_or("Warehouse string must have at least one line")?
            .len();

        let tiles: Vec<Tile> = s
            .chars()
            .filter_map(|c| match c {
                '.' => Some(Tile::Empty),
                '@' => Some(Tile::PaperRoll),
                _ => None,
            })
            .collect();

        if !tiles.len().is_multiple_of(width) {
            return Err("Warehouse must be rectangular".into());
        }

        Ok(Warehouse { width, tiles })
    }
}

enum Tile {
    Empty,
    PaperRoll,
}

#[cfg(test)]
mod test {
    use crate::Warehouse;
    use indoc::indoc;
    use std::str::FromStr;

    const TEST_WAREHOUSE: &str = indoc! {"
        ..@@.@@@@.
        @@@.@.@.@@
        @@@@@.@.@@
        @.@@@@..@.
        @@.@@@@.@@
        .@@@@@@@.@
        .@.@.@.@@@
        @.@@@.@@@@
        .@@@@@@@@.
        @.@.@@@.@.
    "};

    #[test]
    fn test_movable_rolls() {
        assert_eq!(
            13,
            Warehouse::from_str(TEST_WAREHOUSE).unwrap().movable_rolls()
        );
    }
}

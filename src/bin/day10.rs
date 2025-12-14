use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        println!(
            "Minimum button presses to configure all machines: {}",
            min_button_presses_to_configure(BufReader::new(File::open(path)?))?
        );

        Ok(())
    } else {
        Err("Usage: day10 INPUT_FILE_PATH".into())
    }
}

pub fn min_button_presses_to_configure(reader: impl BufRead) -> Result<usize, Box<dyn Error>> {
    reader
        .lines()
        .map_while(|line| line.ok())
        .map(|line| Machine::from_str(&line).map(|machine| machine.shortest_button_sequence()))
        .sum()
}

struct Machine {
    indicator_light_pattern: u16,
    buttons: Vec<u16>,
}

impl Machine {
    pub fn shortest_button_sequence(&self) -> usize {
        let mut distances = [None; u16::MAX as usize];
        let mut queue = BinaryHeap::new();

        queue.push(QueueEntry::new(0, 0));

        while let Some(entry) = queue.pop() {
            distances[entry.lights as usize] = Some(entry.presses);

            if entry.lights == self.indicator_light_pattern {
                return entry.presses;
            }

            for button in &self.buttons {
                let lights = entry.lights ^ button;

                if distances[lights as usize].is_none() {
                    queue.push(QueueEntry::new(lights, entry.presses + 1));
                }
            }
        }

        // No path to the desired state
        panic!("No path to desired state")
    }

    fn indicator_lights_from_str(s: &str) -> Result<u16, Box<dyn Error>> {
        s.strip_prefix('[')
            .ok_or("Indicator light pattern must begin with '['")?
            .strip_suffix(']')
            .ok_or("Indicator light pattern must end with ']'")?
            .chars()
            .rev()
            .try_fold(0, |acc, c| match c {
                '.' => Ok(acc << 1),
                '#' => Ok((acc << 1) | 1),
                _ => Err("Unrecognized indicator light".into()),
            })
    }

    fn button_from_str(s: &str) -> Result<u16, Box<dyn Error>> {
        s.strip_prefix('(')
            .ok_or("Button definition must begin with '('")?
            .strip_suffix(')')
            .ok_or("Button definition must end with ')'")?
            .split(',')
            .try_fold(0, |acc, n| {
                n.parse()
                    .map(|p: u16| acc | 1 << p)
                    .map_err(|_| "Could not parse button definition".into())
            })
    }
}

impl FromStr for Machine {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut components = s.split_whitespace();

        let indicator_light_pattern = components
            .next()
            .ok_or("Machine must have indicator lights component")
            .map(Machine::indicator_lights_from_str)??;

        let _ = components
            .next_back()
            .expect("Machine must have joltage requirement component");

        let buttons = components
            .map(Machine::button_from_str)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Machine {
            indicator_light_pattern,
            buttons,
        })
    }
}

#[derive(Eq, PartialEq)]
struct QueueEntry {
    lights: u16,
    presses: usize,
}

impl QueueEntry {
    pub fn new(lights: u16, presses: usize) -> Self {
        QueueEntry { lights, presses }
    }
}

impl Ord for QueueEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse order to implement a min-queue
        other.presses.cmp(&self.presses)
    }
}

impl PartialOrd for QueueEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod test {
    use crate::Machine;
    use std::str::FromStr;

    #[test]
    fn test_shortest_button_sequence() {
        assert_eq!(
            2,
            Machine::from_str("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}")
                .unwrap()
                .shortest_button_sequence()
        );

        assert_eq!(
            3,
            Machine::from_str("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}")
                .unwrap()
                .shortest_button_sequence()
        );

        assert_eq!(
            2,
            Machine::from_str("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}")
                .unwrap()
                .shortest_button_sequence()
        );
    }
}

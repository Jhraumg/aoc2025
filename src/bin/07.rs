use std::collections::{HashMap, HashSet};
use std::str::FromStr;

advent_of_code::solution!(7);

#[derive(Debug, Clone, Eq, PartialEq)]
struct Point {
    x: u64,
    y: u64,
}

struct Teleporter {
    start: u64,
    splitters: Vec<Point>,
    depth: u64,
}
impl FromStr for Teleporter {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut depth = 0;
        let mut start = u64::MAX;
        let mut splitters = Vec::new();
        for (j, l) in s.lines().enumerate() {
            depth += 1;
            for (i, c) in l.chars().enumerate() {
                match c {
                    'S' => {
                        assert_eq!(j, 0);
                        start = i as u64
                    }
                    '^' => splitters.push(Point {
                        x: i as u64,
                        y: j as u64,
                    }),
                    '.' => {}
                    _ => unreachable!(),
                }
            }
        }
        Ok(Self {
            start,
            splitters,
            depth,
        })
    }
}
impl Teleporter {
    fn total_beam_split(&self) -> u64 {
        let mut count = 0;
        let mut beams: HashSet<u64> = HashSet::new();
        beams.insert(self.start);

        for j in 0..self.depth {
            beams = beams
                .iter()
                .flat_map(|x| {
                    if self.splitters.contains(&Point { x: *x, y: j + 1 }) {
                        count += 1;
                        [*x - 1, *x + 1].into_iter()
                    } else {
                        [*x, *x].into_iter()
                    }
                })
                .collect();
        }

        count
    }

    fn total_timeline(&self) -> u64 {
        let mut timelines: HashMap<u64, u64> = HashMap::new();
        timelines.insert(self.start, 1);

        for j in 0..self.depth {
            timelines = timelines
                .iter()
                .flat_map(|(x, count)| {
                    if self.splitters.contains(&Point { x: *x, y: j + 1 }) {
                        [Some((*x - 1, *count)), Some((*x + 1, *count))]
                            .into_iter()
                            .flatten()
                    } else {
                        [Some((*x, *count)), None].into_iter().flatten()
                    }
                })
                .fold(HashMap::new(), |mut acc, (x, count)| {
                    acc.entry(x).and_modify(|c| *c += count).or_insert(count);
                    acc
                });
        }
        timelines.values().sum()
    }
}
pub fn part_one(input: &str) -> Option<u64> {
    let teleporter: Teleporter = input.parse().unwrap();

    Some(teleporter.total_beam_split())
}

pub fn part_two(input: &str) -> Option<u64> {
    let teleporter: Teleporter = input.parse().unwrap();

    Some(teleporter.total_timeline())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
